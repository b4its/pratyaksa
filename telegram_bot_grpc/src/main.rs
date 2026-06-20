use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};

// Kode hasil generate dari proto/alert.proto (lihat build.rs)
pub mod alert {
    tonic::include_proto!("alert");
}

use alert::alert_service_server::{AlertService, AlertServiceServer};
use alert::{AlertRequest, AlertResponse, HealthRequest, HealthResponse};

/// Entri ringkas tiap unit di cache in-memory (pengganti Redis dari versi Python).
#[derive(Clone)]
struct FleetEntry {
    risk_level: String,
    // disimpan untuk konteks lokasi terakhir unit; belum dibaca balik via RPC
    #[allow(dead_code)]
    last_update: String,
}

/// State bersama antara gRPC server dan loop polling Telegram.
struct AppState {
    bot_token: String,
    http: reqwest::Client,
    backend_url: String,
    /// Base URL frontend untuk link Work Order, mis. https://pratyaksa.kideco.co.id
    wo_link_base: String,
    subscribers_file: String,
    /// Semua chat_id yang sudah menjalankan /start (broadcast alert ke sini).
    subscribers: RwLock<HashSet<i64>>,
    fleet: RwLock<HashMap<String, FleetEntry>>,
}

impl AppState {
    /// Susun pesan alert HTML persis seperti format bot Python sebelumnya.
    fn build_alert_message(d: &AlertRequest, wo_base: &str) -> String {
        let wo_link = format!("{}/wo/create/{}", wo_base.trim_end_matches('/'), d.asset_id);
        format!(
            "🚨 <b>[URGENT ALARM : PRATYAKSA]</b> 🚨\n\n\
             🚜 <b>Unit:</b> {asset} ({model})\n\
             📍 <b>Lokasi:</b> {lokasi}\n\
             ⚠️ <b>Status:</b> {status} (Estimasi Sisa Umur: {rul} Jam)\n\n\
             🔍 <b>Analisis Kerusakan AI (SHAP):</b>\n\
             1. {shap1}\n\
             2. {shap2}\n\n\
             🛠️ <b>Rekomendasi Tindakan:</b>\n\
             Arahkan unit ke Workshop Pit terdekat sebelum breakdown.\n\n\
             📦 <b>Info Suku Cadang:</b>\n\
             - Part Name: {part_name}\n\
             - Part No: {part_no}\n\
             - Stok Workshop: {stok} Unit\n\n\
             🔗 <a href=\"{wo_link}\">Buat Work Order</a>",
            asset = d.asset_id,
            model = d.model,
            lokasi = d.lokasi,
            status = d.status,
            rul = d.rul,
            shap1 = d.shap1,
            shap2 = d.shap2,
            part_name = d.part_name,
            part_no = d.part_no,
            stok = d.stok,
            wo_link = wo_link,
        )
    }

    /// Kirim satu pesan HTML ke chat_id tertentu.
    async fn send_message(&self, chat_id: i64, text: &str) -> Result<(), String> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        let resp = self
            .http
            .post(&url)
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text": text,
                "parse_mode": "HTML",
                "disable_web_page_preview": true,
            }))
            .timeout(Duration::from_secs(15))
            .send()
            .await
            .map_err(|e| format!("HTTP error ke Telegram: {e}"))?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(format!("Telegram API error {status}: {body}"))
        }
    }

    /// Tambah subscriber baru dan persist ke file. Return true bila benar-benar baru.
    async fn add_subscriber(&self, chat_id: i64) -> bool {
        let is_new = {
            let mut subs = self.subscribers.write().await;
            subs.insert(chat_id)
        };
        if is_new {
            self.persist_subscribers().await;
        }
        is_new
    }

    async fn persist_subscribers(&self) {
        let list: Vec<i64> = {
            let subs = self.subscribers.read().await;
            subs.iter().copied().collect()
        };
        if let Some(parent) = std::path::Path::new(&self.subscribers_file).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match serde_json::to_string(&list) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&self.subscribers_file, json) {
                    tracing::warn!("Gagal menyimpan subscribers: {e}");
                }
            }
            Err(e) => tracing::warn!("Gagal serialize subscribers: {e}"),
        }
    }

    /// Ambil ringkasan fleet dari backend untuk command /status.
    async fn fetch_fleet_summary(&self) -> Result<String, String> {
        let url = format!("{}/api/v1/fleet-summary", self.backend_url.trim_end_matches('/'));
        let resp = self
            .http
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("tidak dapat menghubungi backend: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("backend membalas HTTP {}", resp.status()));
        }

        let v: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("respons backend tidak valid: {e}"))?;
        let d = &v["data"];
        let critical = d["critical"].as_i64().unwrap_or(0);
        let warning = d["warning"].as_i64().unwrap_or(0);
        let normal = d["normal"].as_i64().unwrap_or(0);
        let rusak = d["rusak"].as_i64().unwrap_or(0);
        let total = d["total"].as_i64().unwrap_or(0);

        Ok(format!(
            "📊 <b>PRATYAKSA Fleet Status</b>\n\n\
             🔴 CRITICAL : {critical} unit\n\
             🟡 WARNING  : {warning} unit\n\
             🟢 NORMAL   : {normal} unit\n\
             ⚫ RUSAK    : {rusak} unit\n\
             ─────────────────\n\
             📦 Total Unit : {total} unit\n\n\
             <i>Data real-time dari backend PRATYAKSA.</i>"
        ))
    }
}

fn greeting_message() -> String {
    "⚡️ <b>SYSTEM ONLINE: PRATYAKSA Command Center</b> ⚡️\n\
     <i>Engineered by Oryphem</i>\n\n\
     Selamat datang! Anda telah terhubung dengan asisten Predictive Maintenance berbasis AI.\n\n\
     Tujuan utama: <b>Zero Breakdown</b>, meminimalisir Unplanned Downtime, dan memaksimalkan profit operasional tambang Anda. 📈💰\n\n\
     <b>Kapabilitas Sistem:</b>\n\
     📡 Real-time Telemetry — memantau sensor unit 24/7\n\
     🧠 Smart Diagnostics (SHAP) — ungkap akar masalah sebelum breakdown\n\
     🛠️ CMMS Ready — eskalasi alarm jadi Work Order 1 klik\n\n\
     Ketik /status untuk melihat ringkasan fleet saat ini.\n\
     Status AI: [ACTIVE] 🚜💻"
        .to_string()
}

/// Loop long-polling getUpdates: handle /start dan /status untuk SEMUA chat.
async fn run_telegram_polling(state: Arc<AppState>) {
    if state.bot_token.is_empty() {
        tracing::warn!("TELEGRAM_BOT_TOKEN kosong — polling command dinonaktifkan");
        return;
    }

    let url = format!("https://api.telegram.org/bot{}/getUpdates", state.bot_token);
    let mut offset: i64 = 0;
    tracing::info!("Telegram polling aktif (getUpdates)");

    loop {
        let resp = state
            .http
            .get(&url)
            .query(&[
                ("timeout", "30".to_string()),
                ("offset", offset.to_string()),
                ("allowed_updates", "[\"message\"]".to_string()),
            ])
            .timeout(Duration::from_secs(40))
            .send()
            .await;

        let body: serde_json::Value = match resp {
            Ok(r) => match r.json().await {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!("getUpdates parse error: {e}");
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
                }
            },
            Err(e) => {
                tracing::warn!("getUpdates error: {e}");
                tokio::time::sleep(Duration::from_secs(3)).await;
                continue;
            }
        };

        let updates = match body["result"].as_array() {
            Some(arr) => arr,
            None => {
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
        };

        for upd in updates {
            if let Some(uid) = upd["update_id"].as_i64() {
                offset = offset.max(uid + 1);
            }

            let message = &upd["message"];
            let chat_id = match message["chat"]["id"].as_i64() {
                Some(id) => id,
                None => continue,
            };
            let text = message["text"].as_str().unwrap_or("").trim();
            if text.is_empty() {
                continue;
            }

            // Ambil command pertama, buang suffix @namabot dan argumen
            let cmd = text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .split('@')
                .next()
                .unwrap_or("")
                .to_lowercase();

            let state = state.clone();
            tokio::spawn(async move {
                match cmd.as_str() {
                    "/start" => {
                        let is_new = state.add_subscriber(chat_id).await;
                        if let Err(e) = state.send_message(chat_id, &greeting_message()).await {
                            tracing::warn!("Gagal kirim greeting ke {chat_id}: {e}");
                        } else if is_new {
                            tracing::info!("Subscriber baru terdaftar: {chat_id}");
                        }
                    }
                    "/status" => {
                        // Pastikan pengirim juga terdaftar sebagai subscriber
                        state.add_subscriber(chat_id).await;
                        let msg = match state.fetch_fleet_summary().await {
                            Ok(m) => m,
                            Err(e) => format!(
                                "⚠️ Gagal mengambil status fleet: {e}.\nCoba lagi sebentar."
                            ),
                        };
                        if let Err(e) = state.send_message(chat_id, &msg).await {
                            tracing::warn!("Gagal kirim /status ke {chat_id}: {e}");
                        }
                    }
                    _ => {
                        let _ = state
                            .send_message(
                                chat_id,
                                "Perintah tidak dikenal. Gunakan /start atau /status.",
                            )
                            .await;
                    }
                }
            });
        }
    }
}

/// Implementasi service gRPC bot Telegram.
struct BotGrpc {
    state: Arc<AppState>,
}

#[tonic::async_trait]
impl AlertService for BotGrpc {
    async fn send_alert(
        &self,
        request: Request<AlertRequest>,
    ) -> Result<Response<AlertResponse>, Status> {
        let d = request.into_inner();

        if self.state.bot_token.is_empty() {
            return Err(Status::failed_precondition(
                "TELEGRAM_BOT_TOKEN belum dikonfigurasi pada service",
            ));
        }

        // Update cache fleet in-memory
        {
            let mut fleet = self.state.fleet.write().await;
            fleet.insert(
                d.asset_id.clone(),
                FleetEntry {
                    risk_level: d.status.clone(),
                    last_update: d.lokasi.clone(),
                },
            );
        }

        // Broadcast ke SEMUA subscriber (semua device yang sudah /start)
        let targets: Vec<i64> = {
            let subs = self.state.subscribers.read().await;
            subs.iter().copied().collect()
        };

        if targets.is_empty() {
            return Err(Status::failed_precondition(
                "Belum ada subscriber. Minta user menjalankan /start ke bot.",
            ));
        }

        let message = AppState::build_alert_message(&d, &self.state.wo_link_base);
        let mut delivered = 0usize;
        let mut last_err: Option<String> = None;
        for chat_id in &targets {
            match self.state.send_message(*chat_id, &message).await {
                Ok(()) => delivered += 1,
                Err(e) => {
                    tracing::warn!("Gagal kirim alert {} ke {chat_id}: {e}", d.asset_id);
                    last_err = Some(e);
                }
            }
        }

        if delivered == 0 {
            return Err(Status::unavailable(
                last_err.unwrap_or_else(|| "Gagal mengirim ke semua subscriber".to_string()),
            ));
        }

        tracing::info!(
            "Alert {} terkirim ke {}/{} subscriber",
            d.asset_id,
            delivered,
            targets.len()
        );
        Ok(Response::new(AlertResponse {
            success: true,
            message: format!("Alert terkirim ke {}/{} device", delivered, targets.len()),
            asset_id: d.asset_id,
        }))
    }

    async fn health_check(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let fleet = self.state.fleet.read().await;
        let total = fleet.len() as i32;
        let critical = fleet
            .values()
            .filter(|v| v.risk_level.eq_ignore_ascii_case("CRITICAL"))
            .count() as i32;

        Ok(Response::new(HealthResponse {
            ok: true,
            fleet_total: total,
            fleet_critical: critical,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let host = env::var("GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("{host}:{port}").parse()?;

    let bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
    let default_chat_id = env::var("TELEGRAM_CHAT_ID").ok().and_then(|s| s.trim().parse::<i64>().ok());
    let backend_url = env::var("BACKEND_URL").unwrap_or_else(|_| "http://backend:8080".to_string());
    let wo_link_base =
        env::var("WO_LINK_BASE").unwrap_or_else(|_| "http://localhost".to_string());
    let subscribers_file =
        env::var("SUBSCRIBERS_FILE").unwrap_or_else(|_| "/app/data/subscribers.json".to_string());

    if bot_token.is_empty() {
        tracing::warn!("TELEGRAM_BOT_TOKEN belum diset — bot tidak akan mengirim/menerima pesan");
    }

    // Muat subscriber dari file (persisten antar restart) + seed default chat_id.
    let mut initial: HashSet<i64> = HashSet::new();
    if let Ok(content) = std::fs::read_to_string(&subscribers_file) {
        if let Ok(list) = serde_json::from_str::<Vec<i64>>(&content) {
            initial.extend(list);
        }
    }
    if let Some(cid) = default_chat_id {
        initial.insert(cid);
    }
    tracing::info!("Memuat {} subscriber awal", initial.len());

    let state = Arc::new(AppState {
        bot_token,
        http: reqwest::Client::new(),
        backend_url,
        wo_link_base,
        subscribers_file,
        subscribers: RwLock::new(initial),
        fleet: RwLock::new(HashMap::new()),
    });
    // Simpan kembali agar default chat_id ikut tersimpan
    state.persist_subscribers().await;

    // Jalankan loop polling Telegram di background
    let polling_state = state.clone();
    tokio::spawn(async move {
        run_telegram_polling(polling_state).await;
    });

    let grpc = BotGrpc { state };

    tracing::info!("PRATYAKSA Telegram bot gRPC server listening on {addr}");

    Server::builder()
        .add_service(AlertServiceServer::new(grpc))
        .serve_with_shutdown(addr, async {
            let _ = tokio::signal::ctrl_c().await;
            tracing::info!("Shutdown signal diterima, menutup server...");
        })
        .await?;

    Ok(())
}
