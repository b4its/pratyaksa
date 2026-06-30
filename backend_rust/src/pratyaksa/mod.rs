pub mod models;

use crate::config::AppConfig;
use actix_web::web;
use models::*;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

/// Mode operasi Pratyaksa API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PratyaksaMode {
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "simulasi")]
    Simulasi,
}

impl std::fmt::Display for PratyaksaMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PratyaksaMode::Live => write!(f, "live"),
            PratyaksaMode::Simulasi => write!(f, "simulasi"),
        }
    }
}

/// State bersama yang di-cache dari polling PRATYAKSA API
#[derive(Debug, Clone)]
pub struct PratyaksaState {
    pub mode: PratyaksaMode,
    /// Ketika Some, user sudah memilih mode secara manual.
    /// Polling tidak akan mengubah mode — hanya isi data fleet.
    pub manual_mode: Option<PratyaksaMode>,
    pub fleet_data: Vec<FleetAsset>,
    pub health_status: Option<HealthResponse>,
    pub last_health_check: Option<Instant>,
    pub last_fleet_poll: Option<Instant>,
    pub api_reachable: bool,
    pub polling_active: bool,
}

impl Default for PratyaksaState {
    fn default() -> Self {
        Self {
            mode: PratyaksaMode::Simulasi,
            manual_mode: None,
            fleet_data: Vec::new(),
            health_status: None,
            last_health_check: None,
            last_fleet_poll: None,
            api_reachable: false,
            polling_active: false,
        }
    }
}

pub type SharedPratyaksaState = web::Data<RwLock<PratyaksaState>>;

pub fn new_shared_state() -> SharedPratyaksaState {
    web::Data::new(RwLock::new(PratyaksaState::default()))
}

// ── type alias untuk shared ApiClient ─────────────────────────────────
pub type SharedApiClient = web::Data<PratyaksaApiClient>;

/// Client untuk berkomunikasi dengan PRATYAKSA API eksternal
/// Semua method milik &self karena reqwest::Client dibalik Arc internally.
#[derive(Clone)]
pub struct PratyaksaApiClient {
    pub client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl PratyaksaApiClient {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to build reqwest client"),
            base_url: config.pratyaksa_api_url.clone(),
            api_key: config.pratyaksa_api_key.clone(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    fn auth_header(&self) -> &str {
        "X-API-Key"
    }

    pub async fn check_health(&self) -> Result<HealthResponse, String> {
        let url = format!("{}/health", self.base_url);
        let resp = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| format!("Health check gagal: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Health check response: {}", resp.status()));
        }

        resp.json::<HealthResponse>()
            .await
            .map_err(|e| format!("Parse health response gagal: {}", e))
    }

    pub async fn get_fleet(&self) -> Result<FleetResponse, String> {
        let url = format!("{}/fleet", self.base_url);
        let resp = self
            .client
            .get(&url)
            .header(self.auth_header(), &self.api_key)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("Fleet fetch gagal: {}", e))?;

        if resp.status().is_success() {
            resp.json::<FleetResponse>()
                .await
                .map_err(|e| format!("Parse fleet response gagal: {}", e))
        } else if resp.status().as_u16() == 401 {
            Err("Unauthorized: API Key salah atau tidak dikirim".to_string())
        } else {
            Err(format!("Fleet fetch response: {}", resp.status()))
        }
    }

    pub async fn get_result(&self, asset_id: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}/result/{}", self.base_url, asset_id);
        let resp = self
            .client
            .get(&url)
            .header(self.auth_header(), &self.api_key)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("Result fetch gagal: {}", e))?;

        if resp.status().is_success() {
            resp.json::<serde_json::Value>()
                .await
                .map_err(|e| format!("Parse result response gagal: {}", e))
        } else {
            Err(format!("Result fetch response: {}", resp.status()))
        }
    }

    /// GET /features — daftar 37 nama fitur sensor
    pub async fn get_features(&self) -> Result<FeaturesResponse, String> {
        let url = format!("{}/features", self.base_url);
        let resp = self
            .client
            .get(&url)
            .header(self.auth_header(), &self.api_key)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("Features fetch gagal: {}", e))?;

        if resp.status().is_success() {
            resp.json::<FeaturesResponse>()
                .await
                .map_err(|e| format!("Parse features response gagal: {}", e))
        } else {
            Err(format!("Features fetch response: {}", resp.status()))
        }
    }

    /// GET /explain/{prediction_id} — SHAP explanation untuk prediksi tertentu
    pub async fn get_explain(&self, prediction_id: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}/explain/{}", self.base_url, prediction_id);
        let resp = self
            .client
            .get(&url)
            .header(self.auth_header(), &self.api_key)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("Explain fetch gagal: {}", e))?;

        if resp.status().is_success() {
            resp.json::<serde_json::Value>()
                .await
                .map_err(|e| format!("Parse explain response gagal: {}", e))
        } else {
            Err(format!("Explain fetch response: {}", resp.status()))
        }
    }

    /// POST /reload-models — reload model ML tanpa restart
    pub async fn post_reload_models(&self) -> Result<ReloadModelsResponse, String> {
        let url = format!("{}/reload-models", self.base_url);
        let resp = self
            .client
            .post(&url)
            .header(self.auth_header(), &self.api_key)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| format!("Reload models gagal: {}", e))?;

        if resp.status().is_success() {
            resp.json::<ReloadModelsResponse>()
                .await
                .map_err(|e| format!("Parse reload response gagal: {}", e))
        } else {
            Err(format!("Reload models response: {}", resp.status()))
        }
    }
}

/// Generator data simulasi (fallback ketika PRATYAKSA API tidak reachable)
pub mod simulator {
    use super::*;

    fn frand(seed: f64) -> f64 {
        let x = (seed * 12.9898 + 78.233).sin() * 43758.5453;
        x - x.floor()
    }

    fn time_bucket(interval_secs: u64) -> f64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        (now / interval_secs) as f64
    }

    fn seed_from_string(s: &str) -> f64 {
        s.bytes().map(|b| b as f64).sum::<f64>() + s.len() as f64
    }

    fn r1(x: f64) -> f64 {
        (x * 10.0).round() / 10.0
    }

    pub fn generate_health() -> HealthResponse {
        HealthResponse {
            status: "ok".to_string(),
            redis: "simulasi".to_string(),
            postgres: "simulasi".to_string(),
            experts_loaded: vec![
                "bulldozer".to_string(),
                "haul_truck".to_string(),
                "excavator".to_string(),
                "wheel_loader".to_string(),
            ],
            model_version: "2.0.0-simulasi".to_string(),
        }
    }

    pub fn generate_fleet() -> Vec<FleetAsset> {
        let fleet_defs = [
            ("WA600-001", "wheel_loader"),
            ("HD785-001", "haul_truck"),
            ("HD785-002", "haul_truck"),
            ("D155-001", "bulldozer"),
            ("PC2000-001", "excavator"),
            ("DT-001", "haul_truck"),
        ];

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        fleet_defs
            .iter()
            .map(|(asset_id, eq_type)| {
                let seed = seed_from_string(asset_id);
                // Base degradation dari asset_id (konsisten per asset)
                // Pakai time_bucket 300s (5 menit) agar tidak berubah terlalu cepat
                let t = time_bucket(300);
                let base_degr = frand(seed * 1.7 + 13.0); // 0..1 — unique per asset
                let time_jitter = frand(seed + t * 0.05) * 0.15; // small jitter over time
                let degr = (base_degr + time_jitter).clamp(0.0, 1.0);

                // Risk level lebih realistik: sebagian CRITICAL, sebagian WARNING, sebagian NORMAL
                // Degradation: 0-0.3 = NORMAL, 0.3-0.55 = WARNING, 0.55-1.0 = CRITICAL
                let risk_level = if degr < 0.3 {
                    "NORMAL"
                } else if degr < 0.55 {
                    "WARNING"
                } else {
                    "CRITICAL"
                };

                let rul_max = 2000.0;
                // RUL berbanding terbalik dengan degradation
                let lstm_rul = r1(((1.0 - degr) * rul_max * (0.6 + frand(seed + 60.0) * 0.4))
                    .clamp(0.0, rul_max));
                let uncertainty = r1(lstm_rul * (0.05 + degr * 0.15) + 3.0);
                let drift = frand(seed + t + 90.0) > 0.6;

                FleetAsset {
                    asset_id: asset_id.to_string(),
                    equipment_type: eq_type.to_string(),
                    risk_level: risk_level.to_string(),
                    lstm_rul_hours: lstm_rul,
                    rul_uncertainty: uncertainty,
                    model_agreement: frand(seed + 50.0) > 0.2,
                    drift_detected: drift,
                    processed_at: now,
                }
            })
            .collect()
    }

    pub fn generate_result(asset_id: &str) -> serde_json::Value {
        let eq_type = if asset_id.starts_with("WA") {
            "wheel_loader"
        } else if asset_id.starts_with("HD") {
            "haul_truck"
        } else if asset_id.starts_with("D1") {
            "bulldozer"
        } else if asset_id.starts_with("PC") {
            "excavator"
        } else {
            "haul_truck"
        };

        let seed = seed_from_string(asset_id);
        // Sama seperti generate_fleet: time_bucket 300 detik
        let t = time_bucket(300);
        let base_degr = frand(seed * 1.7 + 13.0);
        let time_jitter = frand(seed + t * 0.05) * 0.15;
        let degr = (base_degr + time_jitter).clamp(0.0, 1.0);

        let xgb_class: i32 = if degr < 0.3 {
            0
        } else if degr < 0.55 {
            1
        } else {
            2
        };
        let xgb_class: i32 = if degr < 0.5 {
            0
        } else if degr < 0.78 {
            1
        } else {
            2
        };

        let rul_max = 2000.0;
        let lstm_rul = r1(((1.0 - degr) * rul_max * (0.7 + frand(seed + 60.0) * 0.5))
            .clamp(8.0, rul_max));
        let uncertainty = r1(lstm_rul * (0.08 + degr * 0.12) + 2.0);

        let lstm_class: i32 = if lstm_rul < 120.0 {
            2
        } else if lstm_rul < 400.0 {
            1
        } else {
            0
        };
        let risk_class = xgb_class.max(lstm_class);
        let xgb_label = ["NORMAL", "WARNING", "CRITICAL"][xgb_class as usize].to_string();
        let risk_lbl = ["NORMAL", "WARNING", "CRITICAL"][risk_class as usize].to_string();
        let model_agreement = xgb_class == lstm_class;

        let comp = |nominal: f64, idx: f64| -> f64 {
            r1(
                ((1.0 - degr) * nominal * (0.6 + frand(seed + 70.0 + idx) * 0.6))
                    .clamp(10.0, nominal),
            )
        };

        let drift_detected = frand(seed + t + 90.0) > 0.75;
        let max_z_score = r1(0.6 + frand(seed + t + 91.0) * 2.4);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        serde_json::json!({
            "asset_id": asset_id,
            "equipment_type": eq_type,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "xgb_anomaly_class": xgb_class,
            "xgb_anomaly_label": xgb_label,
            "lstm_rul_hours": lstm_rul,
            "rul_uncertainty": uncertainty,
            "risk_level": risk_lbl,
            "risk_class": risk_class,
            "model_agreement": model_agreement,
            "RUL_hydraulic_system": comp(900.0, 1.0),
            "RUL_hydraulic_pump": comp(760.0, 2.0),
            "RUL_pump_seal_main": comp(560.0, 3.0),
            "RUL_brake_system": comp(820.0, 4.0),
            "RUL_brake_caliper": comp(640.0, 5.0),
            "RUL_brake_pad_rear": comp(360.0, 6.0),
            "RUL_steering_system": comp(880.0, 7.0),
            "digital_twin": {
                "brake_twin_rul": comp(700.0, 8.0),
                "bearing_twin_rul": comp(900.0, 9.0),
                "hydraulic_twin_rul": comp(820.0, 10.0)
            },
            "drift_status": {
                "drift_detected": drift_detected,
                "drifted_features": if drift_detected {
                    let feat_pool = [
                        "engine_oil_temp_c",
                        "vibration_z_g",
                        "coolant_temp_c",
                        "acoustic_emission_db",
                        "oil_particle_count_iso",
                    ];
                    let n = 1 + (frand(seed + 92.0) * 2.0) as usize;
                    feat_pool[..n.min(feat_pool.len())].to_vec()
                } else {
                    vec![]
                },
                "max_z_score": max_z_score,
                "n_drifted": if drift_detected { 1 + (frand(seed + 93.0) * 2.0) as i32 } else { 0 }
            },
            "processed_at": now,
            "latency_ms": r1(28.0 + frand(seed + t + 92.0) * 40.0)
        })
    }

    pub fn generate_workorder(component: &str, risk_score: f64) -> WorkOrderResponse {
        let allowed = ["brake", "hydraulic", "engine", "transmission"];
        let comp = if allowed.contains(&component) {
            component
        } else {
            "brake"
        };
        let risk = risk_score.clamp(0.0, 1.0);
        WorkOrderResponse {
            work_order_id: format!(
                "WO-SIM-{:06}",
                (frand(seed_from_string(comp) + risk) * 999999.0) as u64
            ),
            component: comp.to_string(),
            risk_score: risk,
            status: "CREATED".to_string(),
        }
    }

    pub fn generate_predict(req: &PredictRequest) -> PredictResponse {
        let seed = seed_from_string(&req.asset_id);
        let t = time_bucket(5);
        let degr = frand(seed + t * 0.1);
        let rul_max = 2000.0;
        let lstm_rul = r1(((1.0 - degr) * rul_max * (0.7 + frand(seed + 60.0) * 0.5))
            .clamp(8.0, rul_max));

        let risk_level = if degr < 0.5 {
            "NORMAL"
        } else if degr < 0.78 {
            "WARNING"
        } else {
            "CRITICAL"
        };

        PredictResponse {
            asset_id: req.asset_id.clone(),
            risk_level: risk_level.to_string(),
            lstm_rul_hours: lstm_rul,
            rul_uncertainty: r1(lstm_rul * (0.08 + degr * 0.12) + 2.0),
            model_agreement: frand(seed + 50.0) > 0.25,
        }
    }

    /// GET /features simulasi — mengembalikan 37 nama fitur standar
    pub fn generate_features() -> FeaturesResponse {
        let features: Vec<String> = crate::pratyaksa::models::FEATURE_NAMES
            .iter()
            .map(|s| s.to_string())
            .collect();
        let total = features.len() as i32;
        FeaturesResponse { features, total }
    }

    /// POST /reload-models simulasi
    pub fn generate_reload_models() -> ReloadModelsResponse {
        ReloadModelsResponse {
            status: "ok".to_string(),
            message: "Model reloaded successfully (simulated)".to_string(),
            model_version: "2.0.0-simulasi".to_string(),
            experts_loaded: vec![
                "bulldozer".to_string(),
                "haul_truck".to_string(),
                "excavator".to_string(),
                "wheel_loader".to_string(),
            ],
        }
    }

    /// GET /explain simulasi — SHAP explanation dummy
    pub fn generate_explain() -> serde_json::Value {
        serde_json::json!({
            "prediction_id": "sim-000000",
            "shap_values": [
                {"feature": "payload_tonnage_t", "value": 0.42},
                {"feature": "hour_meter_h", "value": 0.35},
                {"feature": "engine_oil_temp_c", "value": -0.28},
                {"feature": "vibration_z_g", "value": 0.22},
                {"feature": "coolant_temp_c", "value": -0.18},
                {"feature": "oil_viscosity_cst", "value": 0.15},
                {"feature": "acoustic_emission_db", "value": 0.12},
                {"feature": "brake_temp_c", "value": -0.10}
            ],
            "base_value": 0.0,
            "prediction_class": 1,
            "prediction_label": "WARNING"
        })
    }
}

/// Background polling task — cek health API eksternal tiap N detik
/// Menghormati manual_mode: jika user sudah set mode, polling tidak mengubahnya.
pub async fn start_polling(
    state: SharedPratyaksaState,
    client: PratyaksaApiClient,
    poll_interval_secs: u64,
) {
    let mut tick = interval(Duration::from_secs(poll_interval_secs));

    info!(
        "PRATYAKSA Polling dimulai — target: {} (interval: {}s)",
        client.base_url,
        poll_interval_secs
    );

    loop {
        tick.tick().await;

        // Cek apakah user sudah mengunci mode secara manual
        let manual;
        {
            let s = state.read();
            manual = s.manual_mode.clone();
        }

        match manual {
            // ── Mode SIMULASI manual: jangan sentuh API sama sekali ──
            Some(PratyaksaMode::Simulasi) => {
                let mut s = state.write();
                s.mode = PratyaksaMode::Simulasi;
                s.api_reachable = false;
                s.fleet_data = simulator::generate_fleet();
                s.last_fleet_poll = Some(Instant::now());
                s.last_health_check = Some(Instant::now());
            }

            // ── Mode LIVE manual: harus hit API, fallback simulasi ──
            Some(PratyaksaMode::Live) => {
                let health_ok = match client.check_health().await {
                    Ok(health) => {
                        {
                            let mut s = state.write();
                            s.mode = PratyaksaMode::Live;
                            s.health_status = Some(health);
                            s.last_health_check = Some(Instant::now());
                            s.api_reachable = true;
                        }
                        true
                    }
                    Err(e) => {
                        warn!("Health check gagal (mode LIVE manual): {}", e);
                        {
                            let mut s = state.write();
                            s.mode = PratyaksaMode::Live;
                            s.api_reachable = false;
                            s.last_health_check = Some(Instant::now());
                            s.fleet_data = simulator::generate_fleet();
                            s.last_fleet_poll = Some(Instant::now());
                        }
                        false
                    }
                };

                if health_ok {
                    match client.get_fleet().await {
                        Ok(fleet_resp) => {
                            let mut s = state.write();
                            s.fleet_data = fleet_resp.fleet;
                            s.last_fleet_poll = Some(Instant::now());
                        }
                        Err(e) => {
                            warn!("Fleet fetch gagal (mode LIVE manual): {}", e);
                            let mut s = state.write();
                            s.fleet_data = simulator::generate_fleet();
                            s.last_fleet_poll = Some(Instant::now());
                        }
                    }
                }
            }

            // ── Mode AUTO (default): deteksi otomatis dari health check ──
            None => {
                match client.check_health().await {
                    Ok(health) => {
                        {
                            let mut s = state.write();
                            s.mode = PratyaksaMode::Live;
                            s.health_status = Some(health);
                            s.last_health_check = Some(Instant::now());
                            s.api_reachable = true;
                        }

                        match client.get_fleet().await {
                            Ok(fleet_resp) => {
                                let mut s = state.write();
                                s.fleet_data = fleet_resp.fleet;
                                s.last_fleet_poll = Some(Instant::now());
                            }
                            Err(e) => {
                                warn!("Fleet fetch gagal (auto mode): {}", e);
                                let mut s = state.write();
                                s.fleet_data = simulator::generate_fleet();
                                s.last_fleet_poll = Some(Instant::now());
                            }
                        }
                    }
                    Err(e) => {
                        {
                            let mut s = state.write();
                            if s.mode != PratyaksaMode::Simulasi {
                                warn!("PRATYAKSA API tidak reachable — beralih ke mode SIMULASI");
                            }
                            s.mode = PratyaksaMode::Simulasi;
                            s.api_reachable = false;
                            s.last_health_check = Some(Instant::now());
                            s.fleet_data = simulator::generate_fleet();
                            s.last_fleet_poll = Some(Instant::now());
                        }
                        warn!("Health check gagal: {}", e);
                    }
                }
            }
        }
    }
}
