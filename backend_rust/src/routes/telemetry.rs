//! Telemetry ingestion & history.
//! POST /telemetry        — terima 1 record telemetri dari ECM/edge gateway/simulator.
//! GET  /telemetry/unit/{id} — riwayat telemetri tersimpan untuk satu unit.

use crate::{
    db::postgres::PostgresDb,
    errors::AppError,
    models::telemetry::{CreateTelemetryRequest, SensorTelemetry, TelemetryQuery},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;

/// Status_Label berdasarkan ambang keselamatan industri.
fn classify(
    eng_coolant: f64,
    eng_oil_press: f64,
    hyd_oil_temp: f64,
    brake_temp: f64,
    battery: f64,
    fe_ppm: f64,
    water_pct: f64,
    soot_pct: f64,
    fault_sev: i32,
) -> String {
    if eng_coolant > 110.0
        || eng_oil_press < 25.0
        || hyd_oil_temp > 100.0
        || brake_temp > 95.0
        || battery < 23.0
        || fe_ppm > 100.0
        || water_pct > 0.5
        || fault_sev >= 3
    {
        "CRITICAL".to_string()
    } else if eng_coolant > 100.0
        || eng_oil_press < 35.0
        || hyd_oil_temp > 90.0
        || brake_temp > 85.0
        || fe_ppm > 60.0
        || soot_pct > 3.0
        || fault_sev >= 2
    {
        "WARNING".to_string()
    } else {
        "NORMAL".to_string()
    }
}

pub async fn create(
    pg: web::Data<PostgresDb>,
    body: web::Json<CreateTelemetryRequest>,
) -> Result<HttpResponse, AppError> {
    // Pastikan unit ada
    let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM unit_tambang WHERE id = $1")
        .bind(body.unit_id)
        .fetch_one(&pg.pool)
        .await?;
    if exists == 0 {
        return Err(AppError::NotFound("Unit tidak ditemukan".to_string()));
    }

    let ambient = body.ambient_temp_c.unwrap_or(30.0);
    let eng_coolant = body.eng_coolant_temp_c.unwrap_or(85.0);
    let eng_oil_press = body.eng_oil_press_psi.unwrap_or(55.0);
    let hyd_oil_temp = body.hyd_oil_temp_c.unwrap_or(80.0);
    let brake_temp = body.brake_cooling_temp_c.unwrap_or(65.0);
    let battery = body.battery_voltage.unwrap_or(27.5);
    let fe = body.lab_fe_ppm.unwrap_or(20.0);
    let water = body.lab_water_content_pct.unwrap_or(0.05);
    let soot = body.lab_soot_pct.unwrap_or(0.5);
    let fault_sev = body.fault_code_severity.unwrap_or(0);

    // Engineer-generated features
    let delta_eng_temp = eng_coolant - ambient;
    let status_label = classify(
        eng_coolant, eng_oil_press, hyd_oil_temp, brake_temp, battery, fe, water, soot, fault_sev,
    );
    let design = body.design_life_hm.unwrap_or(20000.0);
    let age = body.component_age_hm.unwrap_or(0.0);
    let rul_hours = ((design - age).max(0.0) * (1.0 - fault_sev as f64 * 0.12).max(0.2)).round();

    let rec = sqlx::query_as::<_, SensorTelemetry>(
        r#"
        INSERT INTO sensor_telemetry (
            unit_id, component_type, operator_id, payload_tonnage,
            hour_meter_actual, design_life_hm, component_age_hm, is_remanufactured,
            ambient_temp_c, idle_time_ratio, eng_coolant_temp_c, eng_oil_press_psi,
            eng_rpm, eng_load_pct, hyd_pump_press_psi, hyd_oil_temp_c, trans_oil_temp_c,
            torque_converter_temp_c, final_drive_temp_c, brake_cooling_temp_c, battery_voltage,
            fault_code_severity, lab_fe_ppm, lab_cu_ppm, lab_al_ppm, lab_si_ppm,
            lab_viscosity_100c, lab_water_content_pct, lab_soot_pct,
            delta_eng_temp, status_label, rul_hours
        ) VALUES (
            $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,
            $22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32
        )
        RETURNING *
        "#,
    )
    .bind(body.unit_id)
    .bind(body.component_type.clone().unwrap_or_else(|| "Heavy Equipment".to_string()))
    .bind(&body.operator_id)
    .bind(body.payload_tonnage)
    .bind(body.hour_meter_actual)
    .bind(body.design_life_hm)
    .bind(body.component_age_hm)
    .bind(body.is_remanufactured.unwrap_or(false))
    .bind(body.ambient_temp_c)
    .bind(body.idle_time_ratio)
    .bind(body.eng_coolant_temp_c)
    .bind(body.eng_oil_press_psi)
    .bind(body.eng_rpm)
    .bind(body.eng_load_pct)
    .bind(body.hyd_pump_press_psi)
    .bind(body.hyd_oil_temp_c)
    .bind(body.trans_oil_temp_c)
    .bind(body.torque_converter_temp_c)
    .bind(body.final_drive_temp_c)
    .bind(body.brake_cooling_temp_c)
    .bind(body.battery_voltage)
    .bind(fault_sev)
    .bind(body.lab_fe_ppm)
    .bind(body.lab_cu_ppm)
    .bind(body.lab_al_ppm)
    .bind(body.lab_si_ppm)
    .bind(body.lab_viscosity_100c)
    .bind(body.lab_water_content_pct)
    .bind(body.lab_soot_pct)
    .bind(delta_eng_temp)
    .bind(&status_label)
    .bind(rul_hours)
    .fetch_one(&pg.pool)
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "data": rec
    })))
}

pub async fn list_by_unit(
    pg: web::Data<PostgresDb>,
    path: web::Path<Uuid>,
    query: web::Query<TelemetryQuery>,
) -> Result<HttpResponse, AppError> {
    let unit_id = path.into_inner();
    let limit = query.limit.unwrap_or(100).clamp(1, 1000);

    let rows = sqlx::query_as::<_, SensorTelemetry>(
        "SELECT * FROM sensor_telemetry WHERE unit_id = $1 ORDER BY ts DESC LIMIT $2",
    )
    .bind(unit_id)
    .bind(limit)
    .fetch_all(&pg.pool)
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": rows
    })))
}
