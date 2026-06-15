use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Unit status options matching the frontend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "unit_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitStatus {
    #[serde(rename = "SEHAT")]
    Sehat,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "CRITICAL")]
    Critical,
    #[serde(rename = "RUSAK")]
    Rusak,
}

impl std::fmt::Display for UnitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitStatus::Sehat => write!(f, "SEHAT"),
            UnitStatus::Warning => write!(f, "WARNING"),
            UnitStatus::Critical => write!(f, "CRITICAL"),
            UnitStatus::Rusak => write!(f, "RUSAK"),
        }
    }
}

/// Unit Tambang — stored in PostgreSQL
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UnitTambang {
    pub id: Uuid,
    pub code: String,
    pub jenis_alat_berat_id: Uuid,
    pub jenis_alat_berat_nama: Option<String>, // Joined from jenis_alat_berat
    pub status: String,
    pub health: i32,
    pub maintenance: String,
    pub savings: i64,
    pub img_url: Option<String>,
    pub model3d_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// POST /unit-tambang
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUnitTambangRequest {
    #[validate(length(min = 2, max = 50, message = "Code harus antara 2-50 karakter"))]
    pub code: String,
    pub jenis_alat_berat_id: Uuid,
    pub status: String,
    #[validate(range(min = 0, max = 100, message = "Health harus antara 0-100"))]
    pub health: i32,
    #[validate(length(min = 1, max = 200))]
    pub maintenance: String,
    pub savings: i64,
    pub img_url: Option<String>,
    pub model3d_url: Option<String>,
}

/// PUT /unit-tambang/:id
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUnitTambangRequest {
    #[validate(length(min = 2, max = 50))]
    pub code: Option<String>,
    pub jenis_alat_berat_id: Option<Uuid>,
    pub status: Option<String>,
    #[validate(range(min = 0, max = 100))]
    pub health: Option<i32>,
    #[validate(length(min = 1, max = 200))]
    pub maintenance: Option<String>,
    pub savings: Option<i64>,
    pub img_url: Option<String>,
    pub model3d_url: Option<String>,
}

/// Query params
#[derive(Debug, Deserialize)]
pub struct UnitListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub jenis_alat_berat_id: Option<Uuid>,
}
