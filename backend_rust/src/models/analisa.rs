use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Analisa Kerusakan — stored in MongoDB (flexible schema for sensor data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalisaKerusakan {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub unit_tambang_id: String, // UUID as string for MongoDB
    pub unit_code: String,
    pub tipe_kerusakan: String,
    pub deskripsi: String,
    pub severity: String,        // LOW, MEDIUM, HIGH, CRITICAL
    pub sensor_data: SensorData,
    pub rekomendasi: String,
    pub status_analisa: String,  // OPEN, IN_PROGRESS, RESOLVED
    pub dilaporkan_oleh: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Real-time sensor telemetry embedded in analisa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub suhu_mesin: f64,     // °C
    pub tekanan_oli: f64,    // bar
    pub rpm: f64,
    pub fuel_level: f64,     // percentage
    pub vibration: f64,      // g-force
    pub jam_operasi: f64,    // hours
    pub timestamp: DateTime<Utc>,
}

/// POST /analisa
#[derive(Debug, Deserialize, Validate)]
pub struct CreateAnalisaRequest {
    pub unit_tambang_id: Uuid,
    pub unit_code: String,
    #[validate(length(min = 2, max = 200))]
    pub tipe_kerusakan: String,
    #[validate(length(min = 5))]
    pub deskripsi: String,
    pub severity: String,
    pub sensor_data: CreateSensorDataRequest,
    pub rekomendasi: String,
    pub dilaporkan_oleh: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSensorDataRequest {
    pub suhu_mesin: f64,
    pub tekanan_oli: f64,
    pub rpm: f64,
    pub fuel_level: f64,
    pub vibration: f64,
    pub jam_operasi: f64,
}

/// PUT /analisa/:id
#[derive(Debug, Deserialize)]
pub struct UpdateAnalisaRequest {
    pub status_analisa: Option<String>,
    pub rekomendasi: Option<String>,
    pub deskripsi: Option<String>,
}

/// Query params for listing analisa
#[derive(Debug, Deserialize)]
pub struct AnalisaListQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub unit_tambang_id: Option<String>,
    pub severity: Option<String>,
    pub status_analisa: Option<String>,
}

/// Dashboard summary stats — assembled from both DBs
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_units: i64,
    pub active_units: i64,
    pub critical_units: i64,
    pub total_savings: i64,
    pub status_distribution: Vec<StatusCount>,
    pub monthly_fleet_data: Vec<MonthlyFleetData>,
    pub map_locations: Vec<MapLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCount {
    pub label: String,
    pub jumlah: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyFleetData {
    pub month: String,
    pub sehat: MonthStatusDetail,
    pub warning: MonthStatusDetail,
    pub critical: MonthStatusDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthStatusDetail {
    pub val: i32,
    pub units: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapLocation {
    pub id: i32,
    pub unit: String,
    pub unit_type: String,
    pub lat: f64,
    pub lng: f64,
    pub status: String,
    pub level: String,
    pub color_hex: String,
    pub fuel: String,
    pub operator: String,
    pub speed: String,
    pub temp: String,
    pub last_update: String,
}
