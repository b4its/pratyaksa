use crate::{
    db::postgres::PostgresDb,
    errors::AppError,
    models::{
        jenis_alat_berat::PaginatedResponse,
        unit_tambang::{CreateUnitTambangRequest, UnitTambang, UnitListQuery, UpdateUnitTambangRequest},
    },
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

pub async fn list(
    db: web::Data<PostgresDb>,
    query: web::Query<UnitListQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).min(100);
    let offset = (page - 1) * per_page;

    // Build dynamic query
    let mut conditions: Vec<String> = Vec::new();
    let mut bind_idx = 1i32;

    if query.search.is_some() {
        conditions.push(format!(
            "(u.code ILIKE ${} OR j.nama ILIKE ${})",
            bind_idx, bind_idx
        ));
        bind_idx += 1;
    }
    if query.status.is_some() {
        conditions.push(format!("u.status = ${}", bind_idx));
        bind_idx += 1;
    }
    if query.jenis_alat_berat_id.is_some() {
        conditions.push(format!("u.jenis_alat_berat_id = ${}", bind_idx));
        bind_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let base_sql = format!(
        r#"
        FROM unit_tambang u
        LEFT JOIN jenis_alat_berat j ON u.jenis_alat_berat_id = j.id
        {}
        "#,
        where_clause
    );

    let select_sql = format!(
        r#"
        SELECT u.*, j.nama as jenis_alat_berat_nama
        {}
        ORDER BY u.created_at DESC
        LIMIT ${} OFFSET ${}
        "#,
        base_sql,
        bind_idx,
        bind_idx + 1
    );

    let count_sql = format!("SELECT COUNT(*) {}", base_sql);

    // Build and execute query dynamically
    macro_rules! bind_params {
        ($q:expr) => {{
            let mut q = $q;
            if let Some(ref search) = query.search {
                let pattern = format!("%{}%", search);
                q = q.bind(pattern.clone()).bind(pattern);
            }
            if let Some(ref status) = query.status {
                q = q.bind(status);
            }
            if let Some(ref jenis_id) = query.jenis_alat_berat_id {
                q = q.bind(jenis_id);
            }
            q
        }};
    }

    let mut select_query = sqlx::query_as::<_, UnitTambang>(&select_sql);
    if let Some(ref search) = query.search {
        let pattern = format!("%{}%", search);
        select_query = select_query.bind(pattern.clone()).bind(pattern);
    }
    if let Some(ref status) = query.status {
        select_query = select_query.bind(status);
    }
    if let Some(jenis_id) = query.jenis_alat_berat_id {
        select_query = select_query.bind(jenis_id);
    }
    let items = select_query
        .bind(per_page)
        .bind(offset)
        .fetch_all(&db.pool)
        .await?;

    let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);
    if let Some(ref search) = query.search {
        let pattern = format!("%{}%", search);
        count_query = count_query.bind(pattern.clone()).bind(pattern);
    }
    if let Some(ref status) = query.status {
        count_query = count_query.bind(status);
    }
    if let Some(jenis_id) = query.jenis_alat_berat_id {
        count_query = count_query.bind(jenis_id);
    }
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
    body: web::Json<CreateUnitTambangRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Validate status value
    let valid_statuses = ["SEHAT", "WARNING", "CRITICAL", "RUSAK"];
    if !valid_statuses.contains(&body.status.as_str()) {
        return Err(AppError::ValidationError(
            "Status harus salah satu dari: SEHAT, WARNING, CRITICAL, RUSAK".to_string(),
        ));
    }

    // Check jenis alat berat exists
    let jenis_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM jenis_alat_berat WHERE id = $1"
    )
    .bind(body.jenis_alat_berat_id)
    .fetch_one(&db.pool)
    .await?;

    if jenis_exists == 0 {
        return Err(AppError::NotFound("Jenis alat berat tidak ditemukan".to_string()));
    }

    // Check unique code
    let code_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM unit_tambang WHERE code ILIKE $1"
    )
    .bind(&body.code)
    .fetch_one(&db.pool)
    .await?;

    if code_exists > 0 {
        return Err(AppError::Conflict("Kode unit sudah terdaftar".to_string()));
    }

    let item = sqlx::query_as::<_, UnitTambang>(
        r#"
        WITH inserted AS (
            INSERT INTO unit_tambang (id, code, jenis_alat_berat_id, status, health, maintenance, savings, img_url, model3d_url, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())
            RETURNING *
        )
        SELECT i.*, j.nama as jenis_alat_berat_nama
        FROM inserted i
        LEFT JOIN jenis_alat_berat j ON i.jenis_alat_berat_id = j.id
        "#
    )
    .bind(Uuid::new_v4())
    .bind(&body.code)
    .bind(body.jenis_alat_berat_id)
    .bind(&body.status)
    .bind(body.health)
    .bind(&body.maintenance)
    .bind(body.savings)
    .bind(&body.img_url)
    .bind(&body.model3d_url)
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

    let item = sqlx::query_as::<_, UnitTambang>(
        r#"
        SELECT u.*, j.nama as jenis_alat_berat_nama
        FROM unit_tambang u
        LEFT JOIN jenis_alat_berat j ON u.jenis_alat_berat_id = j.id
        WHERE u.id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Unit tambang dengan id {} tidak ditemukan", id)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": item
    })))
}

pub async fn update(
    db: web::Data<PostgresDb>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUnitTambangRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let id = path.into_inner();

    let existing = sqlx::query_as::<_, UnitTambang>(
        r#"
        SELECT u.*, j.nama as jenis_alat_berat_nama
        FROM unit_tambang u
        LEFT JOIN jenis_alat_berat j ON u.jenis_alat_berat_id = j.id
        WHERE u.id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Unit tambang dengan id {} tidak ditemukan", id)))?;

    if let Some(ref status) = body.status {
        let valid_statuses = ["SEHAT", "WARNING", "CRITICAL", "RUSAK"];
        if !valid_statuses.contains(&status.as_str()) {
            return Err(AppError::ValidationError(
                "Status harus salah satu dari: SEHAT, WARNING, CRITICAL, RUSAK".to_string(),
            ));
        }
    }

    let code = body.code.clone().unwrap_or(existing.code);
    let jenis_id = body.jenis_alat_berat_id.unwrap_or(existing.jenis_alat_berat_id);
    let status = body.status.clone().unwrap_or(existing.status);
    let health = body.health.unwrap_or(existing.health);
    let maintenance = body.maintenance.clone().unwrap_or(existing.maintenance);
    let savings = body.savings.unwrap_or(existing.savings);
    let img_url = body.img_url.clone().or(existing.img_url);
    let model3d_url = body.model3d_url.clone().or(existing.model3d_url);

    let item = sqlx::query_as::<_, UnitTambang>(
        r#"
        WITH updated AS (
            UPDATE unit_tambang
            SET code = $1, jenis_alat_berat_id = $2, status = $3, health = $4,
                maintenance = $5, savings = $6, img_url = $7, model3d_url = $8,
                updated_at = NOW()
            WHERE id = $9
            RETURNING *
        )
        SELECT u.*, j.nama as jenis_alat_berat_nama
        FROM updated u
        LEFT JOIN jenis_alat_berat j ON u.jenis_alat_berat_id = j.id
        "#
    )
    .bind(&code)
    .bind(jenis_id)
    .bind(&status)
    .bind(health)
    .bind(&maintenance)
    .bind(savings)
    .bind(&img_url)
    .bind(&model3d_url)
    .bind(id)
    .fetch_one(&db.pool)
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": item
    })))
}

pub async fn delete(
    db: web::Data<PostgresDb>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let result = sqlx::query("DELETE FROM unit_tambang WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Unit tambang dengan id {} tidak ditemukan", id)));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Unit tambang berhasil dihapus"
    })))
}
