use crate::{
    db::mongo::MongoDb,
    errors::AppError,
    models::analisa::{
        AnalisaKerusakan, AnalisaListQuery, CreateAnalisaRequest, SensorData,
        UpdateAnalisaRequest,
    },
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::FindOptions;
use futures::TryStreamExt;
use validator::Validate;

const COLLECTION: &str = "analisa_kerusakan";

pub async fn list(
    mongo: web::Data<MongoDb>,
    query: web::Query<AnalisaListQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).min(100);
    let skip = (page - 1) * per_page;

    let collection: mongodb::Collection<AnalisaKerusakan> = mongo.collection(COLLECTION);

    // Build filter
    let mut filter = doc! {};
    if let Some(ref unit_id) = query.unit_tambang_id {
        filter.insert("unit_tambang_id", unit_id);
    }
    if let Some(ref severity) = query.severity {
        filter.insert("severity", severity);
    }
    if let Some(ref status) = query.status_analisa {
        filter.insert("status_analisa", status);
    }

    let total = collection
        .count_documents(filter.clone())
        .await?;

    let options = FindOptions::builder()
        .sort(doc! { "created_at": -1 })
        .skip(skip)
        .limit(per_page as i64)
        .build();

    let cursor = collection.find(filter).with_options(options).await?;
    let items: Vec<AnalisaKerusakan> = cursor.try_collect().await?;

    let total_pages = (total + per_page - 1) / per_page;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "data": items,
            "total": total,
            "page": page,
            "per_page": per_page,
            "total_pages": total_pages,
        }
    })))
}

pub async fn create(
    mongo: web::Data<MongoDb>,
    body: web::Json<CreateAnalisaRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let valid_severities = ["LOW", "MEDIUM", "HIGH", "CRITICAL"];
    if !valid_severities.contains(&body.severity.as_str()) {
        return Err(AppError::ValidationError(
            "Severity harus: LOW, MEDIUM, HIGH, atau CRITICAL".to_string(),
        ));
    }

    let now = Utc::now();
    let analisa = AnalisaKerusakan {
        id: None,
        unit_tambang_id: body.unit_tambang_id.to_string(),
        unit_code: body.unit_code.clone(),
        tipe_kerusakan: body.tipe_kerusakan.clone(),
        deskripsi: body.deskripsi.clone(),
        severity: body.severity.clone(),
        sensor_data: SensorData {
            suhu_mesin: body.sensor_data.suhu_mesin,
            tekanan_oli: body.sensor_data.tekanan_oli,
            rpm: body.sensor_data.rpm,
            fuel_level: body.sensor_data.fuel_level,
            vibration: body.sensor_data.vibration,
            jam_operasi: body.sensor_data.jam_operasi,
            timestamp: now,
        },
        rekomendasi: body.rekomendasi.clone(),
        status_analisa: "OPEN".to_string(),
        dilaporkan_oleh: body.dilaporkan_oleh.clone(),
        created_at: now,
        updated_at: now,
    };

    let collection: mongodb::Collection<AnalisaKerusakan> = mongo.collection(COLLECTION);
    let result = collection.insert_one(&analisa).await?;

    let inserted_id = result
        .inserted_id
        .as_object_id()
        .ok_or_else(|| AppError::InternalError("Failed to get inserted ID".to_string()))?;

    let created = collection
        .find_one(doc! { "_id": inserted_id })
        .await?
        .ok_or_else(|| AppError::InternalError("Failed to fetch created document".to_string()))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "data": created
    })))
}

pub async fn get_by_id(
    mongo: web::Data<MongoDb>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id_str = path.into_inner();
    let oid = ObjectId::parse_str(&id_str)
        .map_err(|_| AppError::BadRequest("ID tidak valid".to_string()))?;

    let collection: mongodb::Collection<AnalisaKerusakan> = mongo.collection(COLLECTION);
    let item = collection
        .find_one(doc! { "_id": oid })
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Analisa dengan id {} tidak ditemukan", id_str)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": item
    })))
}

pub async fn update(
    mongo: web::Data<MongoDb>,
    path: web::Path<String>,
    body: web::Json<UpdateAnalisaRequest>,
) -> Result<HttpResponse, AppError> {
    let id_str = path.into_inner();
    let oid = ObjectId::parse_str(&id_str)
        .map_err(|_| AppError::BadRequest("ID tidak valid".to_string()))?;

    let collection: mongodb::Collection<AnalisaKerusakan> = mongo.collection(COLLECTION);

    // Check if exists
    collection
        .find_one(doc! { "_id": oid })
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Analisa dengan id {} tidak ditemukan", id_str)))?;

    let mut update_doc = doc! { "updated_at": Utc::now().to_rfc3339() };

    if let Some(ref status) = body.status_analisa {
        let valid_statuses = ["OPEN", "IN_PROGRESS", "RESOLVED"];
        if !valid_statuses.contains(&status.as_str()) {
            return Err(AppError::ValidationError(
                "Status analisa harus: OPEN, IN_PROGRESS, atau RESOLVED".to_string(),
            ));
        }
        update_doc.insert("status_analisa", status);
    }
    if let Some(ref rekomendasi) = body.rekomendasi {
        update_doc.insert("rekomendasi", rekomendasi);
    }
    if let Some(ref deskripsi) = body.deskripsi {
        update_doc.insert("deskripsi", deskripsi);
    }

    collection
        .update_one(doc! { "_id": oid }, doc! { "$set": update_doc })
        .await?;

    let updated = collection
        .find_one(doc! { "_id": oid })
        .await?
        .ok_or_else(|| AppError::InternalError("Failed to fetch updated document".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": updated
    })))
}

pub async fn delete(
    mongo: web::Data<MongoDb>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id_str = path.into_inner();
    let oid = ObjectId::parse_str(&id_str)
        .map_err(|_| AppError::BadRequest("ID tidak valid".to_string()))?;

    let collection: mongodb::Collection<AnalisaKerusakan> = mongo.collection(COLLECTION);
    let result = collection
        .delete_one(doc! { "_id": oid })
        .await?;

    if result.deleted_count == 0 {
        return Err(AppError::NotFound(format!("Analisa dengan id {} tidak ditemukan", id_str)));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Analisa berhasil dihapus"
    })))
}
