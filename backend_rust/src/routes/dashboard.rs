use crate::{
    db::{mongo::MongoDb, postgres::PostgresDb},
    errors::AppError,
    models::analisa::{
        DashboardStats, MapLocation, MonthlyFleetData, MonthStatusDetail, StatusCount,
    },
    pratyaksa::{PratyaksaMode, SharedPratyaksaState},
};
use actix_web::{web, HttpResponse};

/// Ringkasan fleet ringkas untuk konsumsi internal (mis. bot Telegram /status).
/// Tidak diproteksi JWT — backend hanya ter-expose di network internal Docker,
/// dan datanya hanya berupa hitungan agregat (non-sensitif).
pub async fn get_fleet_summary(
    pg: web::Data<PostgresDb>,
) -> Result<HttpResponse, AppError> {
    let rows = sqlx::query_as::<_, (String, i64)>(
        "SELECT status, COUNT(*) FROM unit_tambang GROUP BY status",
    )
    .fetch_all(&pg.pool)
    .await?;

    let mut critical = 0i64;
    let mut warning = 0i64;
    let mut normal = 0i64;
    let mut rusak = 0i64;
    let mut total = 0i64;
    for (status, count) in &rows {
        total += count;
        match status.as_str() {
            "CRITICAL" => critical += count,
            "WARNING" => warning += count,
            "SEHAT" => normal += count,
            "RUSAK" => rusak += count,
            _ => {}
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "critical": critical,
            "warning": warning,
            "normal": normal,
            "rusak": rusak,
            "total": total
        }
    })))
}

pub async fn get_stats(
    pg: web::Data<PostgresDb>,
    _mongo: web::Data<MongoDb>,
    pratyaksa_state: SharedPratyaksaState,
) -> Result<HttpResponse, AppError> {
    let (mode, reachable) = {
        let s = pratyaksa_state.read();
        (s.mode.clone(), s.api_reachable)
    };

    // LIVE mode: tampilkan data dari live API fleet
    if mode == PratyaksaMode::Live && reachable {
        let fleet = {
            let s = pratyaksa_state.read();
            s.fleet_data.clone()
        };

        let total_units = fleet.len() as i64;
        let mut normal = 0i64;
        let mut warning = 0i64;
        let mut critical = 0i64;
        let mut sum_rul = 0.0;

        for asset in &fleet {
            match asset.risk_level.as_str() {
                "NORMAL" => normal += 1,
                "WARNING" => warning += 1,
                "CRITICAL" => critical += 1,
                _ => {}
            }
            sum_rul += asset.lstm_rul_hours;
        }

        let active_units = normal;
        let total_savings = 0; // LIVE API tidak punya data savings
        let avg_rul = if total_units > 0 { sum_rul / total_units as f64 } else { 0.0 };

        let status_distribution = vec![
            StatusCount { label: "Sehat".to_string(), jumlah: normal },
            StatusCount { label: "Warning".to_string(), jumlah: warning },
            StatusCount { label: "Critical".to_string(), jumlah: critical },
        ];

        let months = vec![
            "Jan", "Feb", "Mar", "Apr", "Mei", "Jun",
            "Jul", "Ags", "Sep", "Okt", "Nov", "Des",
        ];
        let last = (months.len() - 1) as f64;
        let base_sehat = if total_units > 0 { (normal as f64 / total_units as f64) * 100.0 } else { 0.0 };
        let base_warning = if total_units > 0 { (warning as f64 / total_units as f64) * 100.0 } else { 0.0 };
        let base_critical = if total_units > 0 { (critical as f64 / total_units as f64) * 100.0 } else { 0.0 };

        let gen = |base: f64, early_offset: f64, amp: f64, phase: f64, i: usize| -> i32 {
            if i as f64 >= last { return base.round().clamp(0.0, 100.0) as i32; }
            let t = i as f64 / last;
            let wave = ((i as f64) * 0.8 + phase).sin() * amp;
            let v = base + early_offset * (1.0 - t) + wave;
            v.round().clamp(2.0, 98.0) as i32
        };

        let unit_codes: Vec<String> = fleet.iter().map(|a| a.asset_id.clone()).collect();
        let monthly_fleet_data: Vec<MonthlyFleetData> = months.iter().enumerate().map(|(i, m)| MonthlyFleetData {
            month: m.to_string(),
            sehat: MonthStatusDetail { val: gen(base_sehat, -16.0, 5.0, 0.0, i), units: unit_codes.clone() },
            warning: MonthStatusDetail { val: gen(base_warning, 9.0, 4.0, 1.2, i), units: unit_codes.clone() },
            critical: MonthStatusDetail { val: gen(base_critical, 11.0, 4.5, 2.4, i), units: unit_codes.clone() },
        }).collect();

        let operators = [
            "OP-Budi S.", "OP-Joko P.", "OP-Agus T.", "OP-Rian M.",
            "OP-Deni R.", "OP-Siti N.", "OP-Eko W.",
        ];
        let map_locations: Vec<MapLocation> = fleet.iter().enumerate().map(|(i, asset)| {
            let (color_hex, level) = match asset.risk_level.as_str() {
                "NORMAL" => ("#1FA971", "L"),
                "WARNING" => ("#E0A106", "H"),
                "CRITICAL" => ("#E0413E", "I"),
                _ => ("#7A848E", "L"),
            };
            let rul = asset.lstm_rul_hours as i64;
            MapLocation {
                id: (i + 1) as i32,
                unit: asset.asset_id.clone(),
                unit_type: asset.equipment_type.clone(),
                lat: -7.0 + (i as f64) * 0.5,
                lng: 110.0 + (i as f64) * 0.3,
                status: asset.risk_level.clone(),
                level: level.to_string(),
                color_hex: color_hex.to_string(),
                fuel: format!("{}%", 50 + (i * 7) % 40),
                operator: operators[i % operators.len()].to_string(),
                speed: if asset.risk_level == "NORMAL" { "22 km/h".to_string() } else { "12 km/h".to_string() },
                temp: format!("{}°C", 75 + (100 - rul.min(100)) % 30),
                last_update: "Baru saja".to_string(),
            }
        }).collect();

        let stats = DashboardStats {
            total_units,
            active_units,
            critical_units: critical,
            total_savings,
            status_distribution,
            monthly_fleet_data,
            map_locations,
        };

        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "mode": "live",
            "data": stats
        })));
    }

    // SIMULASI mode: data dari PostgreSQL
    let total_units = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM unit_tambang")
        .fetch_one(&pg.pool)
        .await?;

    let active_units = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM unit_tambang WHERE status = 'SEHAT'"
    )
    .fetch_one(&pg.pool)
    .await?;

    let critical_units = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM unit_tambang WHERE status IN ('CRITICAL', 'RUSAK')"
    )
    .fetch_one(&pg.pool)
    .await?;

    let total_savings = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT CAST(SUM(savings) AS BIGINT) FROM unit_tambang"
    )
    .fetch_one(&pg.pool)
    .await?
    .unwrap_or(0);

    let status_rows = sqlx::query_as::<_, (String, i64)>(
        "SELECT status, COUNT(*) as count FROM unit_tambang GROUP BY status ORDER BY status"
    )
    .fetch_all(&pg.pool)
    .await?;

    let label_map = |s: &str| -> String {
        match s {
            "SEHAT" => "Sehat".to_string(),
            "WARNING" => "Warning".to_string(),
            "CRITICAL" => "Critical".to_string(),
            "RUSAK" => "Rusak".to_string(),
            _ => s.to_string(),
        }
    };

    let status_distribution: Vec<StatusCount> = status_rows
        .into_iter()
        .map(|(status, count)| StatusCount {
            label: label_map(&status),
            jumlah: count,
        })
        .collect();

    let months = vec![
        "Jan", "Feb", "Mar", "Apr", "Mei", "Jun",
        "Jul", "Ags", "Sep", "Okt", "Nov", "Des",
    ];

    let unit_codes = sqlx::query_as::<_, (String, String)>(
        "SELECT code, status FROM unit_tambang ORDER BY code"
    )
    .fetch_all(&pg.pool)
    .await?;

    let sehat_units: Vec<String> = unit_codes.iter()
        .filter(|(_, s)| s == "SEHAT")
        .map(|(c, _)| c.clone())
        .collect();
    let warning_units: Vec<String> = unit_codes.iter()
        .filter(|(_, s)| s == "WARNING")
        .map(|(c, _)| c.clone())
        .collect();
    let critical_units_list: Vec<String> = unit_codes.iter()
        .filter(|(_, s)| s == "CRITICAL" || s == "RUSAK")
        .map(|(c, _)| c.clone())
        .collect();

    let total = total_units.max(1);
    let base_sehat = (sehat_units.len() as f64 / total as f64) * 100.0;
    let base_warning = (warning_units.len() as f64 / total as f64) * 100.0;
    let base_critical = (critical_units_list.len() as f64 / total as f64) * 100.0;

    let last = (months.len() - 1) as f64;
    let gen = |base: f64, early_offset: f64, amp: f64, phase: f64, i: usize| -> i32 {
        if i as f64 >= last {
            return base.round().clamp(0.0, 100.0) as i32;
        }
        let t = i as f64 / last;
        let wave = ((i as f64) * 0.8 + phase).sin() * amp;
        let v = base + early_offset * (1.0 - t) + wave;
        v.round().clamp(2.0, 98.0) as i32
    };

    let monthly_fleet_data: Vec<MonthlyFleetData> = months
        .iter()
        .enumerate()
        .map(|(i, m)| MonthlyFleetData {
            month: m.to_string(),
            sehat: MonthStatusDetail {
                val: gen(base_sehat, -16.0, 5.0, 0.0, i),
                units: sehat_units.clone(),
            },
            warning: MonthStatusDetail {
                val: gen(base_warning, 9.0, 4.0, 1.2, i),
                units: warning_units.clone(),
            },
            critical: MonthStatusDetail {
                val: gen(base_critical, 11.0, 4.5, 2.4, i),
                units: critical_units_list.clone(),
            },
        })
        .collect();

    let map_units = sqlx::query_as::<_, (String, Option<String>, String, i32, f64, f64)>(
        r#"
        SELECT u.code, j.nama, u.status, u.health, u.lat, u.lng
        FROM unit_tambang u
        LEFT JOIN jenis_alat_berat j ON u.jenis_alat_berat_id = j.id
        WHERE u.lat IS NOT NULL AND u.lng IS NOT NULL
        ORDER BY u.code
        "#
    )
    .fetch_all(&pg.pool)
    .await?;

    let operators = [
        "OP-Budi S.", "OP-Joko P.", "OP-Agus T.", "OP-Rian M.", "OP-Deni R.", "OP-Siti N.", "OP-Eko W.",
    ];

    let map_locations: Vec<MapLocation> = map_units
        .iter()
        .enumerate()
        .map(|(i, (code, jenis, status, health, lat, lng))| {
            let (color_hex, level) = match status.as_str() {
                "SEHAT" => ("#1FA971", "L"),
                "WARNING" => ("#E0A106", "H"),
                "CRITICAL" => ("#E0413E", "I"),
                "RUSAK" => ("#7A848E", "X"),
                _ => ("#7A848E", "L"),
            };
            let h = *health as i64;
            let speed = match status.as_str() {
                "SEHAT" => "22 km/h",
                "WARNING" => "12 km/h",
                _ => "0 km/h",
            };
            MapLocation {
                id: (i + 1) as i32,
                unit: code.clone(),
                unit_type: jenis.clone().unwrap_or_else(|| "Heavy Equipment".to_string()),
                lat: *lat,
                lng: *lng,
                status: status.clone(),
                level: level.to_string(),
                color_hex: color_hex.to_string(),
                fuel: format!("{}%", (30 + (h % 70)).clamp(5, 99)),
                operator: operators[i % operators.len()].to_string(),
                speed: speed.to_string(),
                temp: format!("{}°C", 78 + ((100 - h) * 40 / 100)),
                last_update: "Baru saja".to_string(),
            }
        })
        .collect();

    let stats = DashboardStats {
        total_units,
        active_units,
        critical_units,
        total_savings,
        status_distribution,
        monthly_fleet_data,
        map_locations,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "mode": "simulasi",
        "data": stats
    })))
}
