use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Jenis Alat Berat — stored in PostgreSQL
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JenisAlatBerat {
    pub id: Uuid,
    pub nama: String,
    pub deskripsi: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// POST /jenis-alat-berat
#[derive(Debug, Deserialize, Validate)]
pub struct CreateJenisAlatBeratRequest {
    #[validate(length(min = 2, max = 200, message = "Nama harus antara 2-200 karakter"))]
    pub nama: String,
    pub deskripsi: Option<String>,
}

/// PUT /jenis-alat-berat/:id
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateJenisAlatBeratRequest {
    #[validate(length(min = 2, max = 200, message = "Nama harus antara 2-200 karakter"))]
    pub nama: Option<String>,
    pub deskripsi: Option<String>,
}

/// Query params for listing
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
