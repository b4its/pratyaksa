use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// Prediction result from ML API — stored in MongoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LivePrediction {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub asset_id: String,
    pub equipment_type: String,
    pub timestamp: String,
    pub xgb_anomaly_class: i32,
    pub xgb_anomaly_label: String,
    pub lstm_rul_hours: f64,
    pub rul_uncertainty: f64,
    pub risk_level: String,
    pub risk_class: i32,
    pub model_agreement: bool,
    pub RUL_hydraulic_system: f64,
    pub RUL_hydraulic_pump: f64,
    pub RUL_pump_seal_main: f64,
    pub RUL_brake_system: f64,
    pub RUL_brake_caliper: f64,
    pub RUL_brake_pad_rear: f64,
    pub RUL_steering_system: f64,
    pub digital_twin: LiveDigitalTwin,
    pub drift_status: LiveDriftStatus,
    pub processed_at: f64,
    pub latency_ms: f64,
    pub stored_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveDigitalTwin {
    pub brake_twin_rul: f64,
    pub bearing_twin_rul: f64,
    pub hydraulic_twin_rul: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveDriftStatus {
    pub drift_detected: bool,
    pub drifted_features: Vec<String>,
    pub max_z_score: f64,
    pub n_drifted: i32,
}

/// Fleet snapshot — periodic fleet data from ML API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveFleetSnapshot {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub asset_id: String,
    pub equipment_type: String,
    pub risk_level: String,
    pub lstm_rul_hours: f64,
    pub rul_uncertainty: f64,
    pub model_agreement: bool,
    pub drift_detected: bool,
    pub processed_at: f64,
    pub stored_at: DateTime<Utc>,
}

/// Sensor reading from ML API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSensorReading {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub asset_id: String,
    pub equipment_type: String,
    pub timestamp: String,
    pub features: Vec<f64>,
    pub stored_at: DateTime<Utc>,
}

/// Work order from ML API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveWorkOrder {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub work_order_id: String,
    pub asset_id: String,
    pub component: String,
    pub risk_score: f64,
    pub status: String,
    pub created_at: String,
    pub stored_at: DateTime<Utc>,
}

/// Drift log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveDriftLog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub asset_id: String,
    pub equipment_type: String,
    pub drifted_features: Vec<String>,
    pub max_z_score: f64,
    pub n_drifted: i32,
    pub drift_detected: bool,
    pub logged_at: String,
    pub stored_at: DateTime<Utc>,
}