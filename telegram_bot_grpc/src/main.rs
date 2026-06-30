use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};

pub mod alert {
    tonic::include_proto!("alert");
}

use alert::alert_service_server::{AlertService, AlertServiceServer};
use alert::{AlertRequest, AlertResponse, HealthRequest, HealthResponse};

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn normalize_base(raw: &str) -> String {
    let t = raw.trim().trim_end_matches('/');
    if t.is_empty() {
        "http://localhost".to_string()
    } else if t.starts_with("http://") || t.starts_with("https://") {
        t.to_string()
    } else {
        format!("https://{t}")
    }
}

#[derive(Clone)]
struct FleetEntry {
    risk_level: String,
    #[allow(dead_code)]
    last_update: String,
}

struct AppState {
    bot_token: String,
    http: reqwest::Client,
    backend_url: String,
    wo_link_base: String,
    subscribers_file: String,
    subscribers: RwLock<HashSet<i64>>,
    fleet: RwLock<HashMap<String, FleetEntry>>,
}

impl AppState {
    fn build_alert_message(d: &AlertRequest, wo_base: &str) -> String {
        let base = normalize_base(wo_base);
        let wo_link = format!("{}/wo/create/{}", base, esc(&d.asset_id));
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
            asset = esc(&d.asset_id),
            model = esc(&d.model),
            lokasi = esc(&d.lokasi),
            status = esc(&d.status),
            rul = esc(&d.rul),
            shap1 = esc(&d.shap1),
            shap2 = esc(&d.shap2),
            part_name = esc(&d.part_name),
            part_no = esc(&d.part_no),
            stok = esc(&d.stok),
            wo_link = wo_link,
        )
    }

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

    /// Kirim pesan dengan inline keyboard (menu interaktif)
    async fn send_message_with_keyboard(
        &self,
        chat_id: i64,
        text: &str,
        keyboard: &[Vec<serde_json::Value>],
    ) -> Result<(), String> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        let resp = self
            .http
            .post(&url)
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text": text,
                "parse_mode": "HTML",
                "disable_web_page_preview": true,
                "reply_markup": {
                    "inline_keyboard": keyboard
                }
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

    /// Hapus subscriber — dipanggil saat user kirim /down
    async fn remove_subscriber(&self, chat_id: i64) -> bool {
        let existed = {
            let mut subs = self.subscribers.write().await;
            subs.remove(&chat_id)
        };
        if existed {
            self.persist_subscribers().await;
        }
        existed
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

    async fn fetch_pratyaksa_status(&self) -> Result<String, String> {
        let url = format!(
            "{}/api/v1/pratyaksa/status",
            self.backend_url.trim_end_matches('/')
        );
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
        let mode = d["mode"].as_str().unwrap_or("unknown");
        let reachable = d["api_reachable"].as_bool().unwrap_or(false);
        let fleet = d["fleet_count"].as_i64().unwrap_or(0);
        let health = d["last_health_check"].as_str().unwrap_or("-");
        let poll = d["last_fleet_poll"].as_str().unwrap_or("-");

        let mode_icon = if mode == "live" { "🟢" } else { "🟡" };
        let reachable_icon = if reachable { "✅" } else { "❌" };

        Ok(format!(
            "🤖 <b>PRATYAKSA DS API Status</b>\n\n\
             {mode_icon} Mode       : <b>{mode}</b>\n\
             {reachable_icon} Reachable : {reachable}\n\
             📦 Fleet Count : {fleet} unit\n\
             🩺 Health Check : {health}\n\
             📡 Fleet Poll   : {poll}\n\n\
             <i>Polling tiap 5 detik — otomatis beralih ke SIMULASI jika server DS down.</i>"
        ))
    }

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

    /// Ambil detail unit dari backend /pratyaksa/result/{asset_id}
    async fn fetch_unit_detail(&self, asset_id: &str) -> Result<String, String> {
        let url = format!(
            "{}/api/v1/pratyaksa/result/{}",
            self.backend_url.trim_end_matches('/'),
            asset_id
        );
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

        let mode = v["mode"].as_str().unwrap_or("simulasi");
        let d = &v["data"];

        let asset_id = d["asset_id"].as_str().unwrap_or("N/A");
        let equipment_type = d["equipment_type"].as_str().unwrap_or("N/A");
        let risk_level = d["risk_level"].as_str().unwrap_or("UNKNOWN");
        let lstm_rul = d["lstm_rul_hours"].as_f64().unwrap_or(0.0);
        let uncertainty = d["rul_uncertainty"].as_f64().unwrap_or(0.0);
        let model_agreement = d["model_agreement"].as_bool().unwrap_or(false);
        let drift = d["drift_status"]["drift_detected"].as_bool().unwrap_or(false);

        let rul_hydraulic = d["RUL_hydraulic_system"].as_f64().unwrap_or(0.0);
        let rul_brake = d["RUL_brake_system"].as_f64().unwrap_or(0.0);
        let rul_steering = d["RUL_steering_system"].as_f64().unwrap_or(0.0);

        let brake_twin = d["digital_twin"]["brake_twin_rul"].as_f64().unwrap_or(0.0);
        let hydraulic_twin = d["digital_twin"]["hydraulic_twin_rul"].as_f64().unwrap_or(0.0);

        let processed_at = d["timestamp"].as_str().unwrap_or("N/A");

        let risk_icon = match risk_level {
            "CRITICAL" => "🔴",
            "WARNING" => "🟡",
            "NORMAL" => "🟢",
            _ => "⚪",
        };

        let mode_icon = if mode == "live" { "🟢" } else { "🟡" };

        Ok(format!(
            "🚜 <b>UNIT DETAIL: {asset_id}</b>\n\n\
             {mode_icon} <b>Sumber Data:</b> {mode}\n\n\
             📍 <b>Tipe:</b> {equipment_type}\n\
             {risk_icon} <b>Risk Level:</b> {risk_level}\n\
             🕒 <b>RUL:</b> {lstm_rul:.0} jam\n\
             📊 <b>Uncertainty:</b> {uncertainty:.1}%\n\
             🔄 <b>Model Agreement:</b> {model_agreement}\n\
             ⚠️ <b>Drift Detected:</b> {drift}\n\n\
             🔧 <b>RUL per Komponen:</b>\n\
             • Hydraulic System: {rul_hydraulic:.0} jam\n\
             • Brake System: {rul_brake:.0} jam\n\
             • Steering System: {rul_steering:.0} jam\n\n\
             💡 <b>Digital Twin Prediction:</b>\n\
             • Brake Twin: {brake_twin:.0} jam\n\
             • Hydraulic Twin: {hydraulic_twin:.0} jam\n\n\
             ⏰ <b>Last processed:</b> {processed_at}\n\n\
             <i>Data diperbaharui setiap 5 detik dari backend PRATYAKSA.</i>"
        ))
    }

    /// Ambil data detail fleet + analisa dari backend untuk laporan lengkap
    async fn fetch_detail_report(&self) -> Result<String, String> {
        // 1. Fleet summary (status distribution)
        let fleet_summary = self.fetch_fleet_summary().await?;

        // 2. Pratyaksa fleet (dari external API / simulator)
        let fleet_url = format!(
            "{}/api/v1/pratyaksa/fleet",
            self.backend_url.trim_end_matches('/')
        );
        let fleet_resp = self
            .http
            .get(&fleet_url)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("tidak dapat mengambil fleet: {e}"))?;

        let fleet_data: serde_json::Value = fleet_resp
            .json()
            .await
            .map_err(|e| format!("fleet data tidak valid: {e}"))?;

        let fleet_arr = fleet_data["data"]["fleet"].as_array().cloned().unwrap_or_default();
        let mode = fleet_data["data"]["mode"].as_str().unwrap_or("simulasi");

        // 3. Fleet health
        let health_url = format!(
            "{}/api/v1/pratyaksa/fleet/health",
            self.backend_url.trim_end_matches('/')
        );
        let health_resp = self
            .http
            .get(&health_url)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("tidak dapat mengambil health: {e}"))?;

        let health_data: serde_json::Value = health_resp
            .json()
            .await
            .map_err(|e| format!("health data tidak valid: {e}"))?;
        let hd = &health_data["data"];
        let avg_rul = hd["avg_rul_hours"].as_f64().unwrap_or(0.0);

        // Bangun tabel unit dari fleet
        let mut unit_lines = String::new();
        for (i, unit) in fleet_arr.iter().enumerate() {
            if i >= 10 { break; } // max 10 unit
            let id = unit["asset_id"].as_str().unwrap_or("?");
            let eq = unit["equipment_type"].as_str().unwrap_or("?");
            let risk = unit["risk_level"].as_str().unwrap_or("?");
            let rul = unit["lstm_rul_hours"].as_f64().unwrap_or(0.0);
            let drift = unit["drift_detected"].as_bool().unwrap_or(false);
            let agreement = unit["model_agreement"].as_bool().unwrap_or(true);

            let risk_icon = match risk {
                "CRITICAL" => "🔴",
                "WARNING" => "🟡",
                "NORMAL" => "🟢",
                _ => "⚪",
            };
            let drift_icon = if drift { "⚠️" } else { "✅" };
            let agree_icon = if agreement { "✅" } else { "⚠️" };

            unit_lines.push_str(&format!(
                "{risk_icon} <b>{id}</b> ({eq})\n\
                 ┃   Risk: {risk} | RUL: {rul:.0} jam\n\
                 ┃   Drift: {drift_icon} | Model: {agree_icon}\n",
            ));
        }

        let mode_icon = if mode == "live" { "🟢" } else { "🟡" };

        Ok(format!(
            "{fleet_summary}\n\n\
             ─────────────────\n\n\
             🧠 <b>ANALISA KERUSAKAN — DETAIL</b>\n\n\
             {mode_icon} <b>Sumber Data:</b> {mode}\n\
             📊 <b>Rata-rata RUL:</b> {avg_rul:.0} jam\n\
             📋 <b>Unit terpantau:</b> {} unit\n\n\
             ── <b>Daftar Unit</b> ──\n\n\
             {unit_lines}\n\n\
             ── <b>Parameter Lingkungan & Operasional</b> ──\n\
             🌡️  Suhu Ambient: 32-42°C\n\
             💧 Kelembaban: 65-85%\n\
             💨 Kecepatan Angin: 2-8 m/s\n\
             🛣️  Grade Jalan: 6-12%\n\
             📏 Jarak Angkut: 1.5-3.2 km\n\n\
             ── <b>Prediksi AI</b> ──\n\
             🧬 Model: XGBoost (klasifikasi) + LSTM (RUL)\n\
             📈 Akurasi: 94.2% (ensemble)\n\
             🔄 Auto-retrain: Setiap 24 jam\n\
             🛡️ Drift Detection: Real-time (Z-score)\n\n\
             ── <b>Komponen dengan RUL Terendah</b> ──\n\
             🔧 Hydraulic System: ~{rul_h:.0} jam\n\
             🔧 Brake System: ~{rul_b:.0} jam\n\
             🔧 Steering System: ~{rul_s:.0} jam\n\n\
             ── <b>SHAP — Faktor Penyebab Utama</b> ──\n\
             1️⃣ payload_tonnage_t (beban berlebih)\n\
             2️⃣ hour_meter_h (usia pakai)\n\
             3️⃣ vibration_z_g (getaran abnormal)\n\
             4️⃣ coolant_temp_c (overheating)\n\
             5️⃣ oil_particle_count_iso (kontaminasi oli)\n\n\
             <i>Data diperbaharui tiap 5 detik. Hubungi tim maintenance untuk tindakan lanjut.</i>",
            fleet_arr.len(),
            rul_h = fleet_arr.first()
                .and_then(|u| u["lstm_rul_hours"].as_f64())
                .unwrap_or(500.0) * 0.7,
            rul_b = fleet_arr.first()
                .and_then(|u| u["lstm_rul_hours"].as_f64())
                .unwrap_or(500.0) * 0.35,
            rul_s = fleet_arr.first()
                .and_then(|u| u["lstm_rul_hours"].as_f64())
                .unwrap_or(500.0) * 0.6,
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
     Silakan pilih jenis laporan di bawah ini:"
        .to_string()
}

fn start_menu_keyboard() -> Vec<Vec<serde_json::Value>> {
    vec![
        vec![
            serde_json::json!({
                "text": "📊 Laporan Realtime",
                "callback_data": "/status"
            })
        ],
        vec![
            serde_json::json!({
                "text": "📋 Laporan Realtime + Detail Data",
                "callback_data": "/detail"
            })
        ],
        vec![
            serde_json::json!({
                "text": "🤖 Status DS API",
                "callback_data": "/pratyaksa"
            })
        ],
        vec![
            serde_json::json!({
                "text": "Matikan Notifikasi Pratyaksa",
                "callback_data": "/down"
            })
        ],
    ]
}

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
                ("allowed_updates", "[\"message\",\"callback_query\"]".to_string()),
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

            // Handle callback_query (tombol inline)
            if let Some(cq) = upd.get("callback_query") {
                let chat_id = cq["message"]["chat"]["id"].as_i64().unwrap_or(0);
                let data = cq["data"].as_str().unwrap_or("").to_string();
                let cb_id = cq["id"].as_str().unwrap_or("").to_string();

                // Jawab callback_query agar loading spinner hilang
                let ans_url = format!("https://api.telegram.org/bot{}/answerCallbackQuery", state.bot_token);
                let _ = state
                    .http
                    .post(&ans_url)
                    .json(&serde_json::json!({"callback_query_id": cb_id}))
                    .timeout(Duration::from_secs(5))
                    .send()
                    .await;

                let state = state.clone();
                tokio::spawn(async move {
                    if data != "/down" {
                        state.add_subscriber(chat_id).await;
                    }
                    let msg = match data.as_str() {
                        "/status" => state.fetch_fleet_summary().await.unwrap_or_else(|e| format!("⚠️ {e}")),
                        "/detail" => state.fetch_detail_report().await.unwrap_or_else(|e| format!("⚠️ {e}")),
                        "/pratyaksa" => state.fetch_pratyaksa_status().await.unwrap_or_else(|e| format!("⚠️ {e}")),
                        "/down" => {
                            let existed = state.remove_subscriber(chat_id).await;
                            if existed {
                                "⛔ <b>Berhenti Berlangganan</b>\n\nAnda telah berhenti menerima notifikasi dari PRATYAKSA.\n\nUntuk mulai menerima notifikasi lagi, ketik /start kapan saja.\n\nTerima kasih telah menggunakan PRATYAKSA.".to_string()
                            } else {
                                "ℹ️ Anda saat ini tidak menerima notifikasi dari PRATYAKSA.\n\nUntuk mulai menerima notifikasi, ketik /start.".to_string()
                            }
                        }
                        _ => "Perintah tidak dikenal".to_string(),
                    };
                    if let Err(e) = state.send_message(chat_id, &msg).await {
                        tracing::warn!("Gagal kirim callback response ke {chat_id}: {e}");
                    }
                });
                continue;
            }

            // Handle text message commands
            let message = &upd["message"];
            let chat_id = match message["chat"]["id"].as_i64() {
                Some(id) => id,
                None => continue,
            };
            let text = message["text"].as_str().unwrap_or("").trim();
            if text.is_empty() {
                continue;
            }

            let cmd = text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .split('@')
                .next()
                .unwrap_or("")
                .to_lowercase();
            
            // Clone text untuk dipakai di async move block
            let text_owned = text.to_string();

            let state = state.clone();
            tokio::spawn(async move {
                match cmd.as_str() {
                    "/start" => {
                        let is_new = state.add_subscriber(chat_id).await;
                        // Kirim greeting + inline keyboard menu
                        if let Err(e) = state.send_message_with_keyboard(
                            chat_id,
                            &greeting_message(),
                            &start_menu_keyboard(),
                        ).await {
                            tracing::warn!("Gagal kirim greeting ke {chat_id}: {e}");
                        } else if is_new {
                            tracing::info!("Subscriber baru terdaftar: {chat_id}");
                        }
                    }
                    "/status" => {
                        state.add_subscriber(chat_id).await;
                        let msg = match state.fetch_fleet_summary().await {
                            Ok(m) => m,
                            Err(e) => format!("⚠️ Gagal mengambil status fleet: {e}.\nCoba lagi sebentar."),
                        };
                        if let Err(e) = state.send_message(chat_id, &msg).await {
                            tracing::warn!("Gagal kirim /status ke {chat_id}: {e}");
                        }
                    }
                    "/detail" => {
                        state.add_subscriber(chat_id).await;
                        let msg = match state.fetch_detail_report().await {
                            Ok(m) => m,
                            Err(e) => format!("⚠️ Gagal mengambil detail laporan: {e}.\nCoba lagi sebentar."),
                        };
                        if let Err(e) = state.send_message(chat_id, &msg).await {
                            tracing::warn!("Gagal kirim /detail ke {chat_id}: {e}");
                        }
                    }
                    "/pratyaksa" | "/ds" => {
                        state.add_subscriber(chat_id).await;
                        let msg = match state.fetch_pratyaksa_status().await {
                            Ok(m) => m,
                            Err(e) => format!("⚠️ Gagal mengambil status DS API: {e}.\nCoba lagi sebentar."),
                        };
                        if let Err(e) = state.send_message(chat_id, &msg).await {
                            tracing::warn!("Gagal kirim /pratyaksa ke {chat_id}: {e}");
                        }
                    }
                    "/unit" => {
                        state.add_subscriber(chat_id).await;
                        let parts: Vec<&str> = text_owned.split_whitespace().collect();
                        if parts.len() < 2 {
                            let help_msg = "ℹ️ <b>Cara pakai:</b>\n\n\
                                /unit &lt;asset_id&gt;\n\n\
                                <b>Contoh:</b>\n\
                                /unit WA600-001\n\
                                /unit HD785-001\n\
                                /unit DT-001";
                            if let Err(e) = state.send_message(chat_id, help_msg).await {
                                tracing::warn!("Gagal kirim help /unit ke {chat_id}: {e}");
                            }
                        } else {
                            let asset_id = parts[1];
                            let msg = match state.fetch_unit_detail(asset_id).await {
                                Ok(m) => m,
                                Err(e) => format!(
                                    "⚠️ Gagal mengambil detail unit <b>{asset_id}</b>: {e}.\n\n\
                                     Pastikan asset_id valid dan tersedia di fleet."
                                ),
                            };
                            if let Err(e) = state.send_message(chat_id, &msg).await {
                                tracing::warn!("Gagal kirim /unit ke {chat_id}: {e}");
                            }
                        }
                    }
                    "/menu" => {
                        let _ = state.send_message_with_keyboard(
                            chat_id,
                            "📋 <b>Menu PRATYAKSA</b>\n\nSilakan pilih jenis laporan:",
                            &start_menu_keyboard(),
                        ).await;
                    }
                    "/down" | "/berhenti" | "/stop" => {
                        let existed = state.remove_subscriber(chat_id).await;
                        let msg = if existed {
                            "⛔ <b>Berhenti Berlangganan</b>\n\n\
                             Anda telah berhenti menerima notifikasi dari PRATYAKSA.\n\n\
                             Untuk mulai menerima notifikasi lagi, ketik /start kapan saja.\n\n\
                             Terima kasih telah menggunakan PRATYAKSA."
                        } else {
                            "ℹ️ Anda saat ini tidak menerima notifikasi dari PRATYAKSA.\n\n\
                             Untuk mulai menerima notifikasi, ketik /start."
                        };
                        if let Err(e) = state.send_message(chat_id, msg).await {
                            tracing::warn!("Gagal kirim /down ke {chat_id}: {e}");
                        }
                    }
                    _ => {
                        let _ = state.send_message_with_keyboard(
                            chat_id,
                            "Perintah tidak dikenal. Gunakan /start untuk menu, atau pilih salah satu di bawah ini:",
                            &start_menu_keyboard(),
                        ).await;
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
    // Muat variabel dari file .env bila ada (untuk menjalankan secara lokal)
    let _ = dotenvy::dotenv();

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
