use crate::{
    db::postgres::PostgresDb,
    errors::AppError,
    models::jenis_alat_berat::{
        CreateJenisAlatBeratRequest, JenisAlatBerat, ListQuery, PaginatedResponse,
        UpdateJenisAlatBeratRequest,
    },
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

pub async fn list(
    db: web::Data<PostgresDb>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).min(100);
    let offset = (page - 1) * per_page;

    let (items, total) = if let Some(ref search) = query.search {
        let pattern = format!("%{}%", search);
        let items = sqlx::query_as::<_, JenisAlatBerat>(
            "SELECT * FROM jenis_alat_berat WHERE nama ILIKE $1 ORDER BY nama ASC LIMIT $2 OFFSET $3"
        )
        .bind(&pattern)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&db.pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM jenis_alat_berat WHERE nama ILIKE $1"
        )
        .bind(&pattern)
        .fetch_one(&db.pool)
        .await?;

        (items, total)
    } else {
        let items = sqlx::query_as::<_, JenisAlatBerat>(
            "SELECT * FROM jenis_alat_berat ORDER BY nama ASC LIMIT $1 OFFSET $2"
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&db.pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM jenis_alat_berat"
        )
        .fetch_one(&db.pool)
        .await?;

        (items, total)
    };

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
    body: web::Json<CreateJenisAlatBeratRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check duplicate nama
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM jenis_alat_berat WHERE nama ILIKE $1"
    )
    .bind(&body.nama)
    .fetch_one(&db.pool)
    .await?;

    if existing > 0 {
        return Err(AppError::Conflict("Jenis alat berat dengan nama tersebut sudah ada".to_string()));
    }

    let item = sqlx::query_as::<_, JenisAlatBerat>(
        r#"
        INSERT INTO jenis_alat_berat (id, nama, deskripsi, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(&body.nama)
    .bind(&body.deskripsi)
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

    let item = sqlx::query_as::<_, JenisAlatBerat>(
        "SELECT * FROM jenis_alat_berat WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Jenis alat berat dengan id {} tidak ditemukan", id)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": item
    })))
}

pub async fn update(
    db: web::Data<PostgresDb>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateJenisAlatBeratRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let id = path.into_inner();

    // Get existing
    let existing = sqlx::query_as::<_, JenisAlatBerat>(
        "SELECT * FROM jenis_alat_berat WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Jenis alat berat dengan id {} tidak ditemukan", id)))?;

    let nama = body.nama.clone().unwrap_or(existing.nama);
    let deskripsi = body.deskripsi.clone().or(existing.deskripsi);

    let item = sqlx::query_as::<_, JenisAlatBerat>(
        r#"
        UPDATE jenis_alat_berat
        SET nama = $1, deskripsi = $2, updated_at = NOW()
        WHERE id = $3
        RETURNING *
        "#
    )
    .bind(&nama)
    .bind(&deskripsi)
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

    // Check if referenced by unit_tambang
    let referenced = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM unit_tambang WHERE jenis_alat_berat_id = $1"
    )
    .bind(id)
    .fetch_one(&db.pool)
    .await?;

    if referenced > 0 {
        return Err(AppError::Conflict(
            "Tidak bisa menghapus jenis alat berat yang sedang digunakan oleh unit".to_string(),
        ));
    }

    let result = sqlx::query("DELETE FROM jenis_alat_berat WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Jenis alat berat dengan id {} tidak ditemukan", id)));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Jenis alat berat berhasil dihapus"
    })))
}
