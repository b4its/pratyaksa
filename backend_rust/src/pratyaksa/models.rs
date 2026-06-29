use serde::{Deserialize, Serialize};

/// Respons dari GET /health (tidak perlu auth)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(default)]
    pub redis: String,
    #[serde(default)]
    pub postgres: String,
    #[serde(default)]
    pub experts_loaded: Vec<String>,
    #[serde(default)]
    pub model_version: String,
}

/// Respons dari GET /fleet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetResponse {
    pub fleet: Vec<FleetAsset>,
    #[serde(default)]
    pub total: i32,
}

/// Satu entri alat dari /fleet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetAsset {
    pub asset_id: String,
    pub equipment_type: String,
    pub risk_level: String,
    pub lstm_rul_hours: f64,
    pub rul_uncertainty: f64,
    pub model_agreement: bool,
    pub drift_detected: bool,
    pub processed_at: f64,
}

/// Respons dari GET /result/{asset_id}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultResponse {
    pub asset_id: String,
    pub equipment_type: String,
    pub xgb_anomaly_class: i32,
    pub xgb_anomaly_label: String,
    pub lstm_rul_hours: f64,
    pub rul_uncertainty: f64,
    pub risk_level: String,
    pub risk_class: i32,
    pub model_agreement: bool,
    pub lstm_hydraulic_system: f64,
    pub lstm_hydraulic_pump: f64,
    pub lstm_pump_seal: f64,
    pub lstm_brake_system: f64,
    pub lstm_brake_caliper: f64,
    pub lstm_brake_pad: f64,
    pub lstm_steering_system: f64,
    pub digital_twin: DigitalTwin,
    pub drift_status: DriftStatus,
    pub latency_ms: f64,
}

/// Simulasi Digital Twin komponen utama
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwin {
    pub brake_twin_rul: f64,
    pub bearing_twin_rul: f64,
    pub hydraulic_twin_rul: f64,
}

/// Status drift sensor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftStatus {
    pub drift_detected: bool,
    pub drifted_features: Vec<String>,
    pub max_z_score: f64,
    pub n_drifted: i32,
}

/// Body POST /predict — wajib 37 fitur sensor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictRequest {
    pub asset_id: String,
    pub equipment_type: String,
    pub timestamp: String,
    /// Harus berisi tepat 37 nilai float sesuai urutan /features
    pub features: Vec<f64>,
}

/// Respons POST /predict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictResponse {
    #[serde(default)]
    pub asset_id: String,
    #[serde(default)]
    pub risk_level: String,
    #[serde(default)]
    pub lstm_rul_hours: f64,
    #[serde(default)]
    pub rul_uncertainty: f64,
    #[serde(default)]
    pub model_agreement: bool,
}

/// Respons POST /workorder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrderResponse {
    #[serde(default)]
    pub work_order_id: String,
    #[serde(default)]
    pub component: String,
    #[serde(default)]
    pub risk_score: f64,
    #[serde(default)]
    pub status: String,
}

/// Respons GET /features — daftar 37 nama fitur sensor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesResponse {
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub total: i32,
}

/// Respons POST /reload-models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReloadModelsResponse {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub model_version: String,
    #[serde(default)]
    pub experts_loaded: Vec<String>,
}

/// 37 nama fitur sensor standar — urutan ini harus sama dengan POST /predict features[]
pub const FEATURE_NAMES: [&str; 37] = [
    "payload_tonnage_t",       // 1  — FMS: muatan aktual (ton)
    "hour_meter_h",            // 2  — CMMS: total jam operasi unit
    "design_life_h",           // 3  — CMMS: desain umur komponen (jam)
    "component_age_h",         // 4  — CMMS: umur komponen aktual (jam)
    "ambient_temp_c",          // 5  — Ambient: suhu lingkungan (°C)
    "wind_speed_mps",          // 6  — Ambient: kecepatan angin (m/s)
    "humidity_pct",            // 7  — Ambient: kelembaban udara (%)
    "coolant_temp_c",          // 8  — ECM/VIMS: suhu coolant mesin (°C)
    "engine_oil_pressure_bar", // 9  — ECM/VIMS: tekanan oli mesin (bar)
    "engine_rpm",              // 10 — ECM/VIMS: putaran mesin (RPM)
    "engine_load_pct",         // 11 — ECM/VIMS: beban mesin (%)
    "hydraulic_temp_c",        // 12 — ECM/VIMS: suhu oli hidrolik (°C)
    "hydraulic_pressure_bar",  // 13 — ECM/VIMS: tekanan sistem hidrolik (bar)
    "trans_oil_temp_c",        // 14 — ECM/VIMS: suhu oli transmisi (°C)
    "torque_conv_temp_c",      // 15 — ECM/VIMS: suhu torque converter (°C)
    "final_drive_temp_c",      // 16 — ECM/VIMS: suhu final drive (°C)
    "brake_temp_c",            // 17 — ECM/VIMS: suhu sistem rem (°C)
    "battery_voltage_v",       // 18 — ECM/VIMS: tegangan baterai (V)
    "alternator_voltage_v",    // 19 — ECM/VIMS: tegangan alternator (V)
    "fault_code_count",        // 20 — ECM/VIMS: jumlah fault code aktif
    "fe_ppm",                  // 21 — LIMS/Lab: partikel besi dalam oli (ppm)
    "cu_ppm",                  // 22 — LIMS/Lab: partikel tembaga (ppm)
    "al_ppm",                  // 23 — LIMS/Lab: partikel aluminium (ppm)
    "si_ppm",                  // 24 — LIMS/Lab: partikel silikon (ppm)
    "oil_viscosity_cst",       // 25 — LIMS/Lab: viskositas oli (cSt)
    "water_content_pct",       // 26 — LIMS/Lab: kandungan air dalam oli (%)
    "soot_pct",                // 27 — LIMS/Lab: kadar jelaga dalam oli (%)
    "delta_eng_temp_c",        // 28 — Engineer: delta suhu mesin vs baseline (°C)
    "boost_pressure_kpa",      // 29 — ECM: tekanan turbo/boost (kPa)
    "exhaust_temp_c",          // 30 — ECM: suhu gas buang (°C)
    "transmission_gear",       // 31 — ECM: gigi transmisi aktif (0–8)
    "vibration_x_g",           // 32 — VIMS: getaran aksial X (g)
    "vibration_y_g",           // 33 — VIMS: getaran aksial Y (g)
    "vibration_z_g",           // 34 — VIMS: getaran aksial Z (g)
    "fuel_consumption_lph",    // 35 — ECM: konsumsi bahan bakar (L/h)
    "oil_particle_count_iso",  // 36 — LIMS: jumlah partikel oli (ISO 4406)
    "acoustic_emission_db",    // 37 — VIMS: emisi akustik struktur (dB)
];
