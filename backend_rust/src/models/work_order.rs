use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Work Order — disimpan di PostgreSQL.
/// `wo_number` adalah kolom turunan (computed) dari `seq`, mis. "WO-00001".
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkOrder {
    pub id: Uuid,
    pub seq: i64,
    pub wo_number: String,
    pub asset_code: String,
    pub equipment_type: String,
    pub status_unit: String,
    pub priority: String,
    pub component: String,
    pub part_no: Option<String>,
    pub rul_hours: i32,
    pub est_cost: i64,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub est_completion_at: Option<DateTime<Utc>>,
    pub technician: Option<String>,
    pub notes: Option<String>,
    pub feedback: Option<String>,
    pub wo_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// POST /work-orders
#[derive(Debug, Deserialize, Validate)]
pub struct CreateWorkOrderRequest {
    #[validate(length(min = 1, max = 80, message = "asset_code wajib diisi"))]
    pub asset_code: String,
    pub equipment_type: Option<String>,
    pub status_unit: String,
    pub priority: Option<String>,
    pub component: Option<String>,
    pub part_no: Option<String>,
    pub rul_hours: Option<i32>,
    pub est_cost: Option<i64>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub est_completion_at: Option<DateTime<Utc>>,
    pub technician: Option<String>,
    pub notes: Option<String>,
}

/// PUT /work-orders/{id}
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWorkOrderRequest {
    pub wo_status: Option<String>,
    pub technician: Option<String>,
    pub notes: Option<String>,
    pub feedback: Option<String>,
}

/// Query params untuk GET /work-orders
#[derive(Debug, Deserialize)]
pub struct WorkOrderListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub wo_status: Option<String>,
    pub asset_code: Option<String>,
}
