use crate::{
    db::mongo::MongoDb,
    errors::AppError,
    models::live::{LiveFleetSnapshot, LivePrediction, LiveWorkOrder},
};
use actix_web::{web, HttpResponse};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use std::time::SystemTime;

/// GET /api/v1/live/predictions — get stored predictions from MongoDB
pub async fn list_predictions(
    mongo: web::Data<MongoDb>,
    query: web::Query<LiveQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(50).min(500);
    let asset_id = query.asset_id.as_deref();

    let filter = match asset_id {
        Some(id) => doc! { "asset_id": id },
        None => doc! {},
    };

    let coll = mongo.collection::<LivePrediction>("live_predictions");
    let mut cursor = coll
        .find(filter)
        .sort(doc! { "stored_at": -1 })
        .limit(limit as i64)
        .await?;

    let mut items = Vec::new();
    while let Some(item) = cursor.try_next().await? {
        items.push(item);
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": items,
        "total": items.len()
    })))
}

/// GET /api/v1/live/predictions/{asset_id}/latest — get latest prediction for asset
pub async fn get_latest_prediction(
    mongo: web::Data<MongoDb>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let asset_id = path.into_inner();
    let coll = mongo.collection::<LivePrediction>("live_predictions");

    let result = coll
        .find_one(doc! { "asset_id": &asset_id })
        .sort(doc! { "stored_at": -1 })
        .await?;

    match result {
        Some(pred) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": pred
        }))),
        None => Err(AppError::NotFound(format!(
            "Prediction untuk asset {} tidak ditemukan",
            asset_id
        ))),
    }
}

/// GET /api/v1/live/fleet — get stored fleet snapshots from MongoDB
pub async fn list_fleet_snapshots(
    mongo: web::Data<MongoDb>,
    query: web::Query<LiveQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(100).min(1000);
    let asset_id = query.asset_id.as_deref();

    let filter = match asset_id {
        Some(id) => doc! { "asset_id": id },
        None => doc! {},
    };

    let coll = mongo.collection::<LiveFleetSnapshot>("live_fleet_snapshots");
    let mut cursor = coll
        .find(filter)
        .sort(doc! { "stored_at": -1 })
        .limit(limit as i64)
        .await?;

    let mut items = Vec::new();
    while let Some(item) = cursor.try_next().await? {
        items.push(item);
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": items,
        "total": items.len()
    })))
}

/// GET /api/v1/live/work-orders — get stored work orders from MongoDB
pub async fn list_work_orders(
    mongo: web::Data<MongoDb>,
    query: web::Query<LiveQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(50).min(500);
    let asset_id = query.asset_id.as_deref();

    let filter = match asset_id {
        Some(id) => doc! { "asset_id": id },
        None => doc! {},
    };

    let coll = mongo.collection::<LiveWorkOrder>("live_work_orders");
    let mut cursor = coll
        .find(filter)
        .sort(doc! { "stored_at": -1 })
        .limit(limit as i64)
        .await?;

    let mut items = Vec::new();
    while let Some(item) = cursor.try_next().await? {
        items.push(item);
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": items,
        "total": items.len()
    })))
}

/// GET /api/v1/live/stats — get aggregate stats from MongoDB live data
pub async fn get_live_stats(
    mongo: web::Data<MongoDb>,
) -> Result<HttpResponse, AppError> {
    let pred_coll = mongo.collection::<LivePrediction>("live_predictions");
    let fleet_coll = mongo.collection::<LiveFleetSnapshot>("live_fleet_snapshots");
    let wo_coll = mongo.collection::<LiveWorkOrder>("live_work_orders");

    let total_predictions = pred_coll.count_documents(doc! {}).await?;
    let total_fleet = fleet_coll.count_documents(doc! {}).await?;
    let total_work_orders = wo_coll.count_documents(doc! {}).await?;

    let cutoff = SystemTime::now() - std::time::Duration::from_secs(3600);
    let recent_predictions = pred_coll
        .count_documents(doc! { "stored_at": { "$gte": mongodb::bson::DateTime::from(cutoff) } })
        .await?;

    let critical_count = pred_coll
        .count_documents(doc! { "risk_level": "CRITICAL" })
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "total_predictions": total_predictions,
            "total_fleet_snapshots": total_fleet,
            "total_work_orders": total_work_orders,
            "predictions_last_hour": recent_predictions,
            "critical_predictions": critical_count,
        }
    })))
}

/// Query params for live data endpoints
#[derive(serde::Deserialize)]
pub struct LiveQuery {
    pub limit: Option<i64>,
    pub asset_id: Option<String>,
}