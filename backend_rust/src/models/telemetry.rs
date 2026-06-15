use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sensor Telemetry — 33 kolom sesuai standar VIMS/KOMTRAX + FMS + CMMS + LIMS.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SensorTelemetry {
    pub id: Uuid,
    pub ts: DateTime<Utc>,
    pub unit_id: Uuid,

    // FMS
    pub component_type: String,
    pub operator_id: Option<String>,
    pub payload_tonnage: Option<f64>,

    // CMMS
    pub hour_meter_actual: Option<f64>,
    pub design_life_hm: Option<f64>,
    pub component_age_hm: Option<f64>,
    pub is_remanufactured: bool,

    // Lingkungan
    pub ambient_temp_c: Option<f64>,

    // ECM / VIMS
    pub idle_time_ratio: Option<f64>,
    pub eng_coolant_temp_c: Option<f64>,
    pub eng_oil_press_psi: Option<f64>,
    pub eng_rpm: Option<f64>,
    pub eng_load_pct: Option<f64>,
    pub hyd_pump_press_psi: Option<f64>,
    pub hyd_oil_temp_c: Option<f64>,
    pub trans_oil_temp_c: Option<f64>,
    pub torque_converter_temp_c: Option<f64>,
    pub final_drive_temp_c: Option<f64>,
    pub brake_cooling_temp_c: Option<f64>,
    pub battery_voltage: Option<f64>,
    pub fault_code_severity: i32,

    // LIMS
    pub lab_fe_ppm: Option<f64>,
    pub lab_cu_ppm: Option<f64>,
    pub lab_al_ppm: Option<f64>,
    pub lab_si_ppm: Option<f64>,
    pub lab_viscosity_100c: Option<f64>,
    pub lab_water_content_pct: Option<f64>,
    pub lab_soot_pct: Option<f64>,

    // Engineer generated
    pub delta_eng_temp: Option<f64>,
    pub status_label: Option<String>,
    pub rul_hours: Option<f64>,

    pub created_at: DateTime<Utc>,
}

/// Request body untuk ingestion telemetri (POST /telemetry).
/// Field minimal; engineer-generated (delta/status/rul) dihitung server bila kosong.
#[derive(Debug, Deserialize)]
pub struct CreateTelemetryRequest {
    pub unit_id: Uuid,
    pub component_type: Option<String>,
    pub operator_id: Option<String>,
    pub payload_tonnage: Option<f64>,
    pub hour_meter_actual: Option<f64>,
    pub design_life_hm: Option<f64>,
    pub component_age_hm: Option<f64>,
    pub is_remanufactured: Option<bool>,
    pub ambient_temp_c: Option<f64>,
    pub idle_time_ratio: Option<f64>,
    pub eng_coolant_temp_c: Option<f64>,
    pub eng_oil_press_psi: Option<f64>,
    pub eng_rpm: Option<f64>,
    pub eng_load_pct: Option<f64>,
    pub hyd_pump_press_psi: Option<f64>,
    pub hyd_oil_temp_c: Option<f64>,
    pub trans_oil_temp_c: Option<f64>,
    pub torque_converter_temp_c: Option<f64>,
    pub final_drive_temp_c: Option<f64>,
    pub brake_cooling_temp_c: Option<f64>,
    pub battery_voltage: Option<f64>,
    pub fault_code_severity: Option<i32>,
    pub lab_fe_ppm: Option<f64>,
    pub lab_cu_ppm: Option<f64>,
    pub lab_al_ppm: Option<f64>,
    pub lab_si_ppm: Option<f64>,
    pub lab_viscosity_100c: Option<f64>,
    pub lab_water_content_pct: Option<f64>,
    pub lab_soot_pct: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct TelemetryQuery {
    pub limit: Option<i64>,
}
