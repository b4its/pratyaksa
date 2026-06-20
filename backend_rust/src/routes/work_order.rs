use crate::{
    db::postgres::PostgresDb,
    errors::AppError,
    models::{
        jenis_alat_berat::PaginatedResponse,
        work_order::{
            CreateWorkOrderRequest, UpdateWorkOrderRequest, WorkOrder, WorkOrderListQuery,
        },
    },
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

const WO_NUMBER_EXPR: &str = "'WO-' || LPAD(seq::text, 5, '0') AS wo_number";

pub async fn list(
    db: web::Data<PostgresDb>,
    query: web::Query<WorkOrderListQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut conditions: Vec<String> = Vec::new();
    let mut idx = 1i32;
    if query.wo_status.is_some() {
        conditions.push(format!("wo_status = ${}", idx));
        idx += 1;
    }
    if query.asset_code.is_some() {
        conditions.push(format!("asset_code ILIKE ${}", idx));
        idx += 1;
    }
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let select_sql = format!(
        "SELECT *, {expr} FROM work_orders {where_clause} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        idx,
        idx + 1,
        expr = WO_NUMBER_EXPR,
        where_clause = where_clause,
    );
    let count_sql = format!("SELECT COUNT(*) FROM work_orders {}", where_clause);

    let mut select_query = sqlx::query_as::<_, WorkOrder>(&select_sql);
    let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);
    if let Some(ref s) = query.wo_status {
        select_query = select_query.bind(s);
        count_query = count_query.bind(s);
    }
    if let Some(ref a) = query.asset_code {
        let pattern = format!("%{}%", a);
        select_query = select_query.bind(pattern.clone());
        count_query = count_query.bind(pattern);
    }

    let items = select_query.bind(per_page).bind(offset).fetch_all(&db.pool).await?;
    let total = count_query.fetch_one(&db.pool).await?;
    let total_pages = (total + per_page - 1) / per_page;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": PaginatedResponse {
            data: items,
            total,
            page,
            per_page,
            total_pages,
        }
    })))
}

pub async fn create(
    db: web::Data<PostgresDb>,
    body: web::Json<CreateWorkOrderRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let valid_status = ["WARNING", "CRITICAL", "RUSAK"];
    if !valid_status.contains(&body.status_unit.as_str()) {
        return Err(AppError::ValidationError(
            "status_unit harus salah satu dari: WARNING, CRITICAL, RUSAK".to_string(),
        ));
    }

    // Prioritas default: HIGH untuk CRITICAL/RUSAK, MEDIUM untuk WARNING
    let priority = body.priority.clone().unwrap_or_else(|| {
        match body.status_unit.as_str() {
            "WARNING" => "MEDIUM".to_string(),
            _ => "HIGH".to_string(),
        }
    });
    let valid_prio = ["HIGH", "MEDIUM", "LOW"];
    if !valid_prio.contains(&priority.as_str()) {
        return Err(AppError::ValidationError(
            "priority harus salah satu dari: HIGH, MEDIUM, LOW".to_string(),
        ));
    }

    let equipment_type = body
        .equipment_type
        .clone()
        .unwrap_or_else(|| "Heavy Equipment".to_string());
    let component = body
        .component
        .clone()
        .unwrap_or_else(|| "Komponen Utama".to_string());
    let rul_hours = body.rul_hours.unwrap_or(0);
    let est_cost = body.est_cost.unwrap_or(0);

    let item = sqlx::query_as::<_, WorkOrder>(&format!(
        r#"
        INSERT INTO work_orders
            (asset_code, equipment_type, status_unit, priority, component, part_no,
             rul_hours, est_cost, scheduled_at, est_completion_at, technician, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *, {}
        "#,
        WO_NUMBER_EXPR
    ))
    .bind(&body.asset_code)
    .bind(&equipment_type)
    .bind(&body.status_unit)
    .bind(&priority)
    .bind(&component)
    .bind(&body.part_no)
    .bind(rul_hours)
    .bind(est_cost)
    .bind(body.scheduled_at)
    .bind(body.est_completion_at)
    .bind(&body.technician)
    .bind(&body.notes)
    .fetch_one(&db.pool)
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "data": item
    })))
}

pub async fn get_by_id(
    db: web::Data<PostgresDb>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let item = sqlx::query_as::<_, WorkOrder>(&format!(
        "SELECT *, {} FROM work_orders WHERE id = $1",
        WO_NUMBER_EXPR
    ))
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Work order {} tidak ditemukan", id)))?;

    // Ambil data unit tambang terkait (by code) agar detail WO lengkap
    let unit = sqlx::query_as::<_, crate::models::unit_tambang::UnitTambang>(
        r#"
        SELECT u.*, j.nama as jenis_alat_berat_nama
        FROM unit_tambang u
        LEFT JOIN jenis_alat_berat j ON u.jenis_alat_berat_id = j.id
        WHERE u.code ILIKE $1
        "#,
    )
    .bind(&item.asset_code)
    .fetch_optional(&db.pool)
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": { "work_order": item, "unit": unit }
    })))
}

pub async fn update(
    db: web::Data<PostgresDb>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateWorkOrderRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let existing = sqlx::query_as::<_, WorkOrder>(&format!(
        "SELECT *, {} FROM work_orders WHERE id = $1",
        WO_NUMBER_EXPR
    ))
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Work order {} tidak ditemukan", id)))?;

    if let Some(ref s) = body.wo_status {
        let valid = ["OPEN", "IN_PROGRESS", "COMPLETED", "CANCELLED"];
        if !valid.contains(&s.as_str()) {
            return Err(AppError::ValidationError(
                "wo_status harus: OPEN, IN_PROGRESS, COMPLETED, CANCELLED".to_string(),
            ));
        }
    }
    if let Some(ref f) = body.feedback {
        let valid = ["as_predicted", "worse", "better", "false_alarm"];
        if !valid.contains(&f.as_str()) {
            return Err(AppError::ValidationError(
                "feedback harus: as_predicted, worse, better, false_alarm".to_string(),
            ));
        }
    }

    let wo_status = body.wo_status.clone().unwrap_or(existing.wo_status);
    let technician = body.technician.clone().or(existing.technician);
    let notes = body.notes.clone().or(existing.notes);
    let feedback = body.feedback.clone().or(existing.feedback);

    let item = sqlx::query_as::<_, WorkOrder>(&format!(
        r#"
        UPDATE work_orders
        SET wo_status = $1, technician = $2, notes = $3, feedback = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING *, {}
        "#,
        WO_NUMBER_EXPR
    ))
    .bind(&wo_status)
    .bind(&technician)
    .bind(&notes)
    .bind(&feedback)
    .bind(id)
    .fetch_one(&db.pool)
    .await?;

    // Saat WO selesai → unit tambang otomatis kembali SEHAT.
    if wo_status == "COMPLETED" {
        sqlx::query(
            r#"
            UPDATE unit_tambang
            SET status = 'SEHAT',
                health = GREATEST(health, 92),
                maintenance = 'Selesai Perbaikan',
                updated_at = NOW()
            WHERE code ILIKE $1
            "#,
        )
        .bind(&item.asset_code)
        .execute(&db.pool)
        .await?;
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "success", "data": item })))
}
