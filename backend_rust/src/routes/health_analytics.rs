//! Health Analytics — menurunkan analitik kesehatan & telemetri sensor
//! secara realtime dari data unit_tambang (PostgreSQL).
//!
//! Tidak ada sensor IoT fisik di lingkungan ini, jadi nilai telemetri
//! diturunkan secara deterministik dari `health` + `status` unit dengan
//! variasi berbasis waktu agar terlihat realtime saat di-polling frontend.

use crate::{db::postgres::PostgresDb, errors::AppError, models::unit_tambang::UnitTambang};
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use serde_json::json;
use uuid::Uuid;

// ---- Helper deterministic pseudo-random ----

/// Pseudo-random dalam rentang [0,1) dari sebuah seed (deterministik).
fn frand(seed: f64) -> f64 {
    let x = (seed * 12.9898 + 78.233).sin() * 43758.5453;
    x - x.floor()
}

/// Seed angka stabil dari kode unit.
fn code_seed(code: &str) -> f64 {
    code.bytes().map(|b| b as f64).sum::<f64>() + code.len() as f64
}

/// Bucket waktu 5 detik — membuat nilai berubah tiap 5 detik (efek realtime).
fn time_bucket() -> f64 {
    (Utc::now().timestamp() / 5) as f64
}

fn risk_level(score: f64) -> &'static str {
    if score < 30.0 {
        "LOW"
    } else if score < 55.0 {
        "MEDIUM"
    } else if score < 80.0 {
        "HIGH"
    } else {
        "CRITICAL"
    }
}

/// Jenis unit dari nama jenis alat berat (untuk Component_Type FMS).
fn unit_kind(jenis: &Option<String>) -> &'static str {
    let j = jenis.as_deref().unwrap_or("").to_lowercase();
    if j.contains("excavator") || j.contains("zaxis") {
        "Excavator"
    } else if j.contains("dump") || j.contains("truck") || j.contains("haul") {
        "Dump Truck"
    } else if j.contains("dozer") {
        "Bulldozer"
    } else if j.contains("loader") {
        "Wheel Loader"
    } else {
        "Heavy Equipment"
    }
}

const OPERATORS: [&str; 7] = [
    "OP-Budi S.", "OP-Joko P.", "OP-Agus T.", "OP-Rian M.", "OP-Deni R.", "OP-Siti N.", "OP-Eko W.",
];

/// Status_Label berbasis batas keselamatan industri (engineer-generated target).
fn classify_status(
    eng_coolant: f64,
    eng_oil_press: f64,
    hyd_oil_temp: f64,
    brake_temp: f64,
    battery: f64,
    fe_ppm: f64,
    water_pct: f64,
    soot_pct: f64,
    fault_sev: i32,
) -> &'static str {
    // CRITICAL — di luar ambang aman, risiko kegagalan/keselamatan
    if eng_coolant > 110.0
        || eng_oil_press < 25.0
        || hyd_oil_temp > 100.0
        || brake_temp > 95.0
        || battery < 23.0
        || fe_ppm > 100.0
        || water_pct > 0.5
        || fault_sev >= 3
    {
        return "CRITICAL";
    }
    // WARNING — mendekati ambang, perlu pengawasan
    if eng_coolant > 100.0
        || eng_oil_press < 35.0
        || hyd_oil_temp > 90.0
        || brake_temp > 85.0
        || fe_ppm > 60.0
        || soot_pct > 3.0
        || fault_sev >= 2
    {
        return "WARNING";
    }
    "NORMAL"
}

/// Menurunkan telemetri 33-kolom realtime dari sebuah unit.
fn derive_telemetry(unit: &UnitTambang) -> serde_json::Value {
    let health = unit.health as f64;
    let seed = code_seed(&unit.code);
    let t = time_bucket();
    let degr = ((100.0 - health) / 100.0).clamp(0.0, 1.0);
    let kind = unit_kind(&unit.jenis_alat_berat_nama);

    // --- CMMS / masa hidup ---
    let design_life_hm = match kind {
        "Dump Truck" => 24000.0,
        "Excavator" => 20000.0,
        "Bulldozer" => 18000.0,
        "Wheel Loader" => 16000.0,
        _ => 20000.0,
    };
    let hour_meter_actual = (4000.0 + frand(seed + 11.0) * 18000.0).round();
    // komponen makin tua (mendekati design life) bila health rendah
    let component_age_hm = (design_life_hm * (0.25 + degr * 0.7) + (frand(seed + 12.0) - 0.5) * 800.0)
        .clamp(200.0, design_life_hm * 1.05)
        .round();
    let is_remanufactured = frand(seed + 13.0) > 0.6;

    // --- FMS ---
    let operator_id = OPERATORS[(seed as usize + (t as usize)) % OPERATORS.len()];
    let payload_tonnage = match kind {
        "Dump Truck" => (88.0 + frand(seed + t + 20.0) * 14.0).round(),
        "Wheel Loader" => (10.0 + frand(seed + t + 20.0) * 4.0 * 10.0).round() / 10.0,
        _ => 0.0,
    };

    // --- Lingkungan ---
    let ambient_temp_c = (27.0 + frand(seed + t + 21.0) * 9.0).round();

    // --- ECM / VIMS ---
    let idle_time_ratio = ((0.12 + frand(seed + t + 22.0) * 0.28) * 100.0).round() / 100.0;
    let eng_coolant_temp_c = (82.0 + degr * 38.0 + (frand(seed + t + 1.0) - 0.5) * 5.0).clamp(70.0, 128.0);
    let eng_oil_press_psi = (62.0 - degr * 40.0 + (frand(seed + t + 2.0) - 0.5) * 4.0).clamp(15.0, 72.0);
    let eng_rpm = (1300.0 + frand(seed + t + 3.0) * 650.0).round();
    let eng_load_pct = (45.0 + frand(seed + t + 4.0) * 50.0).round();
    let hyd_pump_press_psi = (3400.0 - degr * 700.0 + (frand(seed + t + 5.0) - 0.5) * 150.0).round();
    let hyd_oil_temp_c = (72.0 + degr * 35.0 + (frand(seed + t + 6.0) - 0.5) * 4.0).clamp(60.0, 115.0);
    let trans_oil_temp_c = (78.0 + degr * 30.0 + (frand(seed + t + 7.0) - 0.5) * 4.0).clamp(65.0, 118.0);
    let torque_converter_temp_c = (85.0 + degr * 32.0 + (frand(seed + t + 8.0) - 0.5) * 4.0).clamp(70.0, 125.0);
    let final_drive_temp_c = (74.0 + degr * 30.0 + (frand(seed + t + 9.0) - 0.5) * 4.0).clamp(60.0, 110.0);
    let brake_cooling_temp_c = (60.0 + degr * 42.0 + (frand(seed + t + 10.0) - 0.5) * 5.0).clamp(45.0, 110.0);
    let battery_voltage = ((27.8 - degr * 5.5 + (frand(seed + t + 14.0) - 0.5) * 0.6) * 10.0).round() / 10.0;
    let fault_code_severity = (degr * 4.2 + frand(seed + t + 15.0) * 0.8).floor().clamp(0.0, 4.0) as i32;

    // --- LIMS / Lab oli ---
    let lab_fe_ppm = (15.0 + degr * 130.0 + (frand(seed + t + 16.0) - 0.5) * 8.0).clamp(5.0, 220.0).round();
    let lab_cu_ppm = (3.0 + degr * 40.0 + (frand(seed + t + 17.0) - 0.5) * 3.0).clamp(1.0, 70.0).round();
    let lab_al_ppm = (2.0 + degr * 28.0 + (frand(seed + t + 18.0) - 0.5) * 2.0).clamp(0.0, 50.0).round();
    let lab_si_ppm = (5.0 + degr * 35.0 + (frand(seed + t + 19.0) - 0.5) * 3.0).clamp(2.0, 60.0).round();
    let lab_viscosity_100c = ((15.0 - degr * 3.0 + (frand(seed + 23.0) - 0.5) * 0.6) * 10.0).round() / 10.0;
    let lab_water_content_pct = ((0.03 + degr * 0.7 + (frand(seed + t + 24.0) - 0.5) * 0.04) * 100.0).round() / 100.0;
    let lab_soot_pct = ((0.4 + degr * 4.0 + (frand(seed + t + 25.0) - 0.5) * 0.2) * 10.0).round() / 10.0;

    // --- Engineer generated ---
    let delta_eng_temp = (eng_coolant_temp_c - ambient_temp_c).round();
    let status_label = classify_status(
        eng_coolant_temp_c,
        eng_oil_press_psi,
        hyd_oil_temp_c,
        brake_cooling_temp_c,
        battery_voltage,
        lab_fe_ppm,
        lab_water_content_pct,
        lab_soot_pct,
        fault_code_severity,
    );
    // RUL = sisa umur desain, dikoreksi tingkat keparahan
    let base_rul = (design_life_hm - component_age_hm).max(0.0);
    let severity_factor = 1.0 - (fault_code_severity as f64 * 0.12);
    let rul_hours = (base_rul * severity_factor.max(0.2)).round();

    serde_json::json!({
        "component_type": kind,
        "operator_id": operator_id,
        "payload_tonnage": payload_tonnage,
        "hour_meter_actual": hour_meter_actual,
        "design_life_hm": design_life_hm,
        "component_age_hm": component_age_hm,
        "is_remanufactured": is_remanufactured,
        "ambient_temp_c": ambient_temp_c,
        "idle_time_ratio": idle_time_ratio,
        "eng_coolant_temp_c": eng_coolant_temp_c.round(),
        "eng_oil_press_psi": eng_oil_press_psi.round(),
        "eng_rpm": eng_rpm,
        "eng_load_pct": eng_load_pct,
        "hyd_pump_press_psi": hyd_pump_press_psi,
        "hyd_oil_temp_c": hyd_oil_temp_c.round(),
        "trans_oil_temp_c": trans_oil_temp_c.round(),
        "torque_converter_temp_c": torque_converter_temp_c.round(),
        "final_drive_temp_c": final_drive_temp_c.round(),
        "brake_cooling_temp_c": brake_cooling_temp_c.round(),
        "battery_voltage": battery_voltage,
        "fault_code_severity": fault_code_severity,
        "lab_fe_ppm": lab_fe_ppm,
        "lab_cu_ppm": lab_cu_ppm,
        "lab_al_ppm": lab_al_ppm,
        "lab_si_ppm": lab_si_ppm,
        "lab_viscosity_100c": lab_viscosity_100c,
        "lab_water_content_pct": lab_water_content_pct,
        "lab_soot_pct": lab_soot_pct,
        "delta_eng_temp": delta_eng_temp,
        "status_label": status_label,
        "rul_hours": rul_hours,
    })
}

/// Komponen yang dipantau untuk radar kesehatan.
const COMPONENTS: [&str; 6] = [
    "Engine",
    "Hidrolik",
    "Transmisi",
    "Rem",
    "Bearing",
    "Kelistrikan",
];

/// Menurunkan satu paket analitik kesehatan dari sebuah unit.
fn derive_unit_analysis(unit: &UnitTambang) -> serde_json::Value {
    let health = unit.health as f64;
    let seed = code_seed(&unit.code);
    let t = time_bucket();
    let degr = ((100.0 - health) / 100.0).clamp(0.0, 1.0); // 0 = sehat, 1 = rusak

    // ---- Risk Score (0-100) ----
    let wobble = (frand(seed + t) - 0.5) * 6.0;
    let mut risk_score = (100.0 - health) + wobble;
    if unit.status == "RUSAK" {
        risk_score += 15.0;
    } else if unit.status == "CRITICAL" {
        risk_score += 8.0;
    }
    let risk_score = risk_score.clamp(0.0, 100.0);

    // ---- Sensor telemetry realtime ----
    let suhu_mesin = (78.0 + degr * 45.0 + (frand(seed + t + 1.0) - 0.5) * 4.0).clamp(60.0, 130.0);
    let vibration = (1.5 + degr * 6.5 + (frand(seed + t + 2.0) - 0.5) * 0.6).clamp(0.5, 10.0);
    let tekanan_oli = (6.8 - degr * 3.2 + (frand(seed + t + 3.0) - 0.5) * 0.3).clamp(1.5, 8.0);
    let rpm = (1300.0 + frand(seed + t + 4.0) * 650.0).round();
    let fuel_level = (22.0 + frand(seed + 9.0) * 73.0).round();
    let oil_particle_iso = (13.0 + degr * 9.0 + (frand(seed + t + 6.0) - 0.5) * 1.0).round();
    let acoustic_db = (42.0 + degr * 55.0 + (frand(seed + t + 7.0) - 0.5) * 5.0).clamp(35.0, 120.0);
    let jam_operasi = (4000.0 + frand(seed + 11.0) * 16000.0).round();

    // ---- Component health (radar) ----
    let component_health: Vec<serde_json::Value> = COMPONENTS
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let v = (health + (frand(seed + i as f64 * 10.0) - 0.5) * 28.0).clamp(3.0, 100.0);
            json!({ "component": name, "health": v.round() })
        })
        .collect();

    // komponen terlemah → target RUL
    let (weak_idx, weak_health) = component_health
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c["health"].as_f64().unwrap_or(100.0)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap_or((0, health));

    // ---- RUL Prediction (jam tersisa) ----
    let rul_hours = (weak_health * 11.0 + frand(seed + 5.0) * 80.0).round();
    let rul_confidence = (72.0 + frand(seed + 8.0) * 23.0).round(); // %
    let rul_lower = (rul_hours * 0.82).round();
    let rul_upper = (rul_hours * 1.18).round();

    // ---- SHAP-like feature contributions (kontribusi ke risk) ----
    // nilai positif = mendorong risk naik
    let shap = vec![
        json!({ "feature": format!("Suhu {}", COMPONENTS[weak_idx]), "value": ((suhu_mesin - 80.0) * 0.9).round() }),
        json!({ "feature": "Vibrasi 3-axis", "value": (vibration * 6.0 - 12.0).round() }),
        json!({ "feature": "Tekanan Hidrolik", "value": ((6.8 - tekanan_oli) * 7.0).round() }),
        json!({ "feature": "Partikel Oli (ISO)", "value": ((oil_particle_iso - 14.0) * 3.0).round() }),
        json!({ "feature": "Emisi Akustik", "value": ((acoustic_db - 50.0) * 0.4).round() }),
        json!({ "feature": "Jam Operasi", "value": ((jam_operasi / 1000.0) - 8.0).round() }),
    ];

    // ---- Sensor history 24 jam terakhir (time-series) ----
    let now = Utc::now();
    let history: Vec<serde_json::Value> = (0..24)
        .rev()
        .map(|h| {
            let ts = now - Duration::hours(h);
            // tren memburuk mendekati sekarang untuk unit yang degradasi
            let progress = (24 - h) as f64 / 24.0;
            let trend = degr * progress;
            let hseed = seed + h as f64;
            json!({
                "time": ts.format("%H:00").to_string(),
                "suhu_mesin": (76.0 + trend * 44.0 + (frand(hseed + 1.0) - 0.5) * 3.0).round(),
                "vibration": ((1.4 + trend * 6.0 + (frand(hseed + 2.0) - 0.5) * 0.5) * 10.0).round() / 10.0,
                "tekanan_oli": ((6.8 - trend * 3.0 + (frand(hseed + 3.0) - 0.5) * 0.25) * 10.0).round() / 10.0,
                "acoustic": (42.0 + trend * 52.0 + (frand(hseed + 4.0) - 0.5) * 4.0).round(),
            })
        })
        .collect();

    json!({
        "unit": {
            "id": unit.id,
            "code": unit.code,
            "jenis_alat_berat_nama": unit.jenis_alat_berat_nama,
            "status": unit.status,
            "health": unit.health,
            "img_url": unit.img_url,
            "model3d_url": unit.model3d_url,
        },
        "risk_score": risk_score.round(),
        "risk_level": risk_level(risk_score),
        "sensor_readings": {
            "suhu_mesin": suhu_mesin.round(),
            "vibration": (vibration * 10.0).round() / 10.0,
            "tekanan_oli": (tekanan_oli * 10.0).round() / 10.0,
            "rpm": rpm,
            "fuel_level": fuel_level,
            "oil_particle_iso": oil_particle_iso,
            "acoustic_db": acoustic_db.round(),
            "jam_operasi": jam_operasi,
        },
        "component_health": component_health,
        "rul_prediction": {
            "component": COMPONENTS[weak_idx],
            "hours_remaining": rul_hours,
            "lower_bound": rul_lower,
            "upper_bound": rul_upper,
            "confidence": rul_confidence,
        },
        "shap_contributions": shap,
        "sensor_history": history,
        "telemetry": derive_telemetry(unit),
        "updated_at": now.to_rfc3339(),
    })
}

/// GET /api/v1/analisa/overview — analitik kesehatan seluruh armada (realtime).
pub async fn get_overview(pg: web::Data<PostgresDb>) -> Result<HttpResponse, AppError> {
    let units = sqlx::query_as::<_, UnitTambang>(
        r#"
        SELECT u.*, j.nama as jenis_alat_berat_nama
        FROM unit_tambang u
        LEFT JOIN jenis_alat_berat j ON u.jenis_alat_berat_id = j.id
        ORDER BY u.health ASC
        "#,
    )
    .fetch_all(&pg.pool)
    .await?;

    let total = units.len() as i64;
    let mut status_count = std::collections::BTreeMap::new();
    let mut risk_count = std::collections::BTreeMap::new();
    let mut sum_health = 0.0;
    let mut sum_risk = 0.0;
    let mut sum_suhu = 0.0;
    let mut sum_vib = 0.0;

    let mut unit_summaries = Vec::new();

    for u in &units {
        *status_count.entry(u.status.clone()).or_insert(0i64) += 1;
        let a = derive_unit_analysis(u);
        let risk = a["risk_score"].as_f64().unwrap_or(0.0);
        let level = a["risk_level"].as_str().unwrap_or("LOW").to_string();
        *risk_count.entry(level.clone()).or_insert(0i64) += 1;
        sum_health += u.health as f64;
        sum_risk += risk;
        sum_suhu += a["sensor_readings"]["suhu_mesin"].as_f64().unwrap_or(0.0);
        sum_vib += a["sensor_readings"]["vibration"].as_f64().unwrap_or(0.0);

        unit_summaries.push(json!({
            "id": u.id,
            "code": u.code,
            "jenis_alat_berat_nama": u.jenis_alat_berat_nama,
            "status": u.status,
            "health": u.health,
            "risk_score": risk,
            "risk_level": level,
        }));
    }

    let n = total.max(1) as f64;

    // Status distribution (untuk pie chart)
    let status_distribution: Vec<serde_json::Value> = ["SEHAT", "WARNING", "CRITICAL", "RUSAK"]
        .iter()
        .map(|s| json!({ "label": s, "count": status_count.get(*s).copied().unwrap_or(0) }))
        .collect();

    // Risk distribution (untuk bar chart)
    let risk_distribution: Vec<serde_json::Value> = ["LOW", "MEDIUM", "HIGH", "CRITICAL"]
        .iter()
        .map(|s| json!({ "label": s, "count": risk_count.get(*s).copied().unwrap_or(0) }))
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": {
            "total_units": total,
            "avg_health": (sum_health / n).round(),
            "avg_risk_score": (sum_risk / n).round(),
            "fleet_avg_suhu": (sum_suhu / n).round(),
            "fleet_avg_vibration": ((sum_vib / n) * 10.0).round() / 10.0,
            "units_at_risk": risk_count.get("HIGH").copied().unwrap_or(0)
                + risk_count.get("CRITICAL").copied().unwrap_or(0),
            "status_distribution": status_distribution,
            "risk_distribution": risk_distribution,
            "units": unit_summaries,
            "updated_at": Utc::now().to_rfc3339(),
        }
    })))
}

/// GET /api/v1/analisa/unit/{id} — analitik detail satu unit (realtime).
pub async fn get_unit_analysis(
    pg: web::Data<PostgresDb>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let unit = sqlx::query_as::<_, UnitTambang>(
        r#"
        SELECT u.*, j.nama as jenis_alat_berat_nama
        FROM unit_tambang u
        LEFT JOIN jenis_alat_berat j ON u.jenis_alat_berat_id = j.id
        WHERE u.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&pg.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Unit dengan id {} tidak ditemukan", id)))?;

    let analysis = derive_unit_analysis(&unit);

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": analysis
    })))
}
