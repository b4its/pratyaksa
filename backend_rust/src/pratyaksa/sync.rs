use crate::config::AppConfig;
use crate::db::mongo::{self, MongoDb};
use crate::models::live::{
    LiveDriftLog, LiveDriftStatus, LiveDigitalTwin, LiveFleetSnapshot, LivePrediction,
    LiveWorkOrder,
};
use crate::pratyaksa::{PratyaksaMode, SharedPratyaksaState};
use chrono::Utc;
use sqlx::postgres::PgPoolOptions;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

/// Start background sync task to migrate/replicate data from ml-pratyaksa PostgreSQL
/// to local MongoDB. Runs every `ml_sync_interval_secs` seconds.
pub async fn start_sync(
    state: SharedPratyaksaState,
    config: AppConfig,
    mongo: MongoDb,
) {
    let ml_pg_url = config.ml_postgres_url.clone();
    let interval_secs = config.ml_sync_interval_secs;

    info!(
        "ML PRATYAKSA Sync dimulai — target: {} (interval: {}s)",
        ml_pg_url, interval_secs
    );

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&ml_pg_url)
        .await
    {
        Ok(pool) => {
            info!("ML PRATYAKSA PostgreSQL terhubung untuk sync");
            pool
        }
        Err(e) => {
            warn!("ML PRATYAKSA PostgreSQL tidak tersedia: {}. Sync akan retry.", e);
            // Retry connection on next interval
            let pool = loop {
                tokio::time::sleep(Duration::from_secs(interval_secs)).await;
                match PgPoolOptions::new()
                    .max_connections(5)
                    .acquire_timeout(Duration::from_secs(10))
                    .connect(&ml_pg_url)
                    .await
                {
                    Ok(pool) => {
                        info!("ML PRATYAKSA PostgreSQL terhubung setelah retry");
                        break pool;
                    }
                    Err(e) => {
                        warn!("Retry ML PostgreSQL gagal: {}", e);
                    }
                }
            };
            pool
        }
    };

    let mut tick = interval(Duration::from_secs(interval_secs));

    loop {
        tick.tick().await;

        // Only sync when in LIVE mode and API is reachable
        {
            let s = state.read();
            if s.mode != PratyaksaMode::Live || !s.api_reachable {
                continue;
            }
        }

        if let Err(e) = sync_predictions(&pool, &mongo).await {
            warn!("Sync predictions gagal: {}", e);
        }
        if let Err(e) = sync_equipment_units(&pool, &mongo).await {
            warn!("Sync equipment units gagal: {}", e);
        }
        if let Err(e) = sync_work_orders(&pool, &mongo).await {
            warn!("Sync work orders gagal: {}", e);
        }
        if let Err(e) = sync_drift_logs(&pool, &mongo).await {
            warn!("Sync drift logs gagal: {}", e);
        }
    }
}

/// Sync predictions from ml-pratyaksa PostgreSQL to MongoDB
async fn sync_predictions(pool: &sqlx::PgPool, mongo: &MongoDb) -> Result<(), String> {
    let rows = sqlx::query_as::<_, MlPredictionRow>(
        r#"
        SELECT
            time, asset_id, equipment_type,
            xgb_anomaly_class, xgb_anomaly_label,
            lstm_rul_hours, rul_uncertainty,
            rul_hydraulic_system, rul_hydraulic_pump, rul_pump_seal_main,
            rul_brake_system, rul_brake_caliper, rul_brake_pad_rear,
            rul_steering_system,
            risk_level, risk_class, model_agreement,
            brake_twin_rul, bearing_twin_rul, hydraulic_twin_rul,
            drift_detected, drift_max_z_score, drift_n_features,
            latency_ms, api_version
        FROM predictions
        WHERE time > NOW() - INTERVAL '1 hour'
        ORDER BY time DESC
        LIMIT 500
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Query predictions gagal: {}", e))?;

    let count = rows.len();
    let now = Utc::now();
    for row in rows {
        let prediction = LivePrediction {
            id: None,
            asset_id: row.asset_id,
            equipment_type: row.equipment_type.unwrap_or_default(),
            timestamp: row.time.to_rfc3339(),
            xgb_anomaly_class: row.xgb_anomaly_class.unwrap_or(0) as i32,
            xgb_anomaly_label: row.xgb_anomaly_label.unwrap_or_default(),
            lstm_rul_hours: row.lstm_rul_hours.unwrap_or(0.0) as f64,
            rul_uncertainty: row.rul_uncertainty.unwrap_or(0.0) as f64,
            risk_level: row.risk_level.unwrap_or_default(),
            risk_class: row.risk_class.unwrap_or(0) as i32,
            model_agreement: row.model_agreement.unwrap_or(false),
            RUL_hydraulic_system: row.rul_hydraulic_system.unwrap_or(0.0) as f64,
            RUL_hydraulic_pump: row.rul_hydraulic_pump.unwrap_or(0.0) as f64,
            RUL_pump_seal_main: row.rul_pump_seal_main.unwrap_or(0.0) as f64,
            RUL_brake_system: row.rul_brake_system.unwrap_or(0.0) as f64,
            RUL_brake_caliper: row.rul_brake_caliper.unwrap_or(0.0) as f64,
            RUL_brake_pad_rear: row.rul_brake_pad_rear.unwrap_or(0.0) as f64,
            RUL_steering_system: row.rul_steering_system.unwrap_or(0.0) as f64,
            digital_twin: LiveDigitalTwin {
                brake_twin_rul: row.brake_twin_rul.unwrap_or(0.0) as f64,
                bearing_twin_rul: row.bearing_twin_rul.unwrap_or(0.0) as f64,
                hydraulic_twin_rul: row.hydraulic_twin_rul.unwrap_or(0.0) as f64,
            },
            drift_status: LiveDriftStatus {
                drift_detected: row.drift_detected.unwrap_or(false),
                drifted_features: Vec::new(),
                max_z_score: row.drift_max_z_score.unwrap_or(0.0) as f64,
                n_drifted: row.drift_n_features.unwrap_or(0),
            },
            processed_at: row.time.timestamp() as f64,
            latency_ms: row.latency_ms.unwrap_or(0.0) as f64,
            stored_at: now,
        };
        mongo::store_prediction(mongo, prediction);
    }

    if count > 0 {
        info!("Sync: {} predictions dari ML PostgreSQL", count);
    }
    Ok(())
}

/// Sync equipment units from ml-pratyaksa PostgreSQL to MongoDB (as fleet snapshots)
async fn sync_equipment_units(pool: &sqlx::PgPool, mongo: &MongoDb) -> Result<(), String> {
    let rows = sqlx::query_as::<_, MlEquipmentRow>(
        r#"
        SELECT asset_id, equipment_type, is_active
        FROM equipment_units
        WHERE is_active = TRUE
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Query equipment_units gagal: {}", e))?;

    let count = rows.len();
    let now = Utc::now();
    for row in rows {
        let snapshot = LiveFleetSnapshot {
            id: None,
            asset_id: row.asset_id,
            equipment_type: row.equipment_type,
            risk_level: "UNKNOWN".to_string(),
            lstm_rul_hours: 0.0,
            rul_uncertainty: 0.0,
            model_agreement: true,
            drift_detected: false,
            processed_at: now.timestamp() as f64,
            stored_at: now,
        };
        mongo::store_fleet_snapshot(mongo, snapshot);
    }

    if count > 0 {
        info!("Sync: {} equipment units dari ML PostgreSQL", count);
    }
    Ok(())
}

/// Sync work orders from ml-pratyaksa PostgreSQL to MongoDB
async fn sync_work_orders(pool: &sqlx::PgPool, mongo: &MongoDb) -> Result<(), String> {
    let rows = sqlx::query_as::<_, MlWorkOrderRow>(
        r#"
        SELECT id, asset_id, component, status, created_at, estimated_cost_usd
        FROM work_orders
        WHERE created_at > NOW() - INTERVAL '24 hours'
        ORDER BY created_at DESC
        LIMIT 200
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Query work_orders gagal: {}", e))?;

    let count = rows.len();
    let now = Utc::now();
    for row in rows {
        let wo = LiveWorkOrder {
            id: None,
            work_order_id: row.id,
            asset_id: row.asset_id,
            component: row.component.unwrap_or_default(),
            risk_score: row.estimated_cost_usd.unwrap_or(0.0) as f64 / 1000.0,
            status: row.status.unwrap_or_default(),
            created_at: row.created_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
            stored_at: now,
        };
        mongo::store_work_order(mongo, wo);
    }

    if count > 0 {
        info!("Sync: {} work orders dari ML PostgreSQL", count);
    }
    Ok(())
}

/// Sync drift logs from ml-pratyaksa PostgreSQL to MongoDB
async fn sync_drift_logs(pool: &sqlx::PgPool, mongo: &MongoDb) -> Result<(), String> {
    let rows = sqlx::query_as::<_, MlDriftLogRow>(
        r#"
        SELECT asset_id, equipment_type, drifted_features, max_z_score, n_drifted, logged_at
        FROM drift_log
        WHERE logged_at > NOW() - INTERVAL '1 hour'
        ORDER BY logged_at DESC
        LIMIT 200
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Query drift_log gagal: {}", e))?;

    let count = rows.len();
    let now = Utc::now();
    for row in rows {
        let features: Vec<String> = row
            .drifted_features
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        let log = LiveDriftLog {
            id: None,
            asset_id: row.asset_id.unwrap_or_default(),
            equipment_type: row.equipment_type.unwrap_or_default(),
            drifted_features: features,
            max_z_score: row.max_z_score.unwrap_or(0.0) as f64,
            n_drifted: row.n_drifted.unwrap_or(0),
            drift_detected: row.n_drifted.unwrap_or(0) > 0,
            logged_at: row.logged_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
            stored_at: now,
        };
        mongo::store_drift_log(mongo, log);
    }

    if count > 0 {
        info!("Sync: {} drift logs dari ML PostgreSQL", count);
    }
    Ok(())
}

// ── Row types for deserializing ml-pratyaksa queries ──

#[derive(Debug, sqlx::FromRow)]
struct MlPredictionRow {
    time: chrono::DateTime<Utc>,
    asset_id: String,
    equipment_type: Option<String>,
    xgb_anomaly_class: Option<i16>,
    xgb_anomaly_label: Option<String>,
    lstm_rul_hours: Option<f32>,
    rul_uncertainty: Option<f32>,
    rul_hydraulic_system: Option<f32>,
    rul_hydraulic_pump: Option<f32>,
    rul_pump_seal_main: Option<f32>,
    rul_brake_system: Option<f32>,
    rul_brake_caliper: Option<f32>,
    rul_brake_pad_rear: Option<f32>,
    rul_steering_system: Option<f32>,
    risk_level: Option<String>,
    risk_class: Option<i16>,
    model_agreement: Option<bool>,
    brake_twin_rul: Option<f32>,
    bearing_twin_rul: Option<f32>,
    hydraulic_twin_rul: Option<f32>,
    drift_detected: Option<bool>,
    drift_max_z_score: Option<f32>,
    drift_n_features: Option<i32>,
    latency_ms: Option<f32>,
    api_version: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct MlEquipmentRow {
    asset_id: String,
    equipment_type: String,
    is_active: Option<bool>,
}

#[derive(Debug, sqlx::FromRow)]
struct MlWorkOrderRow {
    id: String,
    asset_id: String,
    component: Option<String>,
    status: Option<String>,
    created_at: Option<chrono::DateTime<Utc>>,
    estimated_cost_usd: Option<f32>,
}

#[derive(Debug, sqlx::FromRow)]
struct MlDriftLogRow {
    asset_id: Option<String>,
    equipment_type: Option<String>,
    drifted_features: Option<serde_json::Value>,
    max_z_score: Option<f32>,
    n_drifted: Option<i32>,
    logged_at: Option<chrono::DateTime<Utc>>,
}
