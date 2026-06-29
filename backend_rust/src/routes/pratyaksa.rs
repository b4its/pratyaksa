use crate::config::AppConfig;
use crate::pratyaksa::{
    simulator, PratyaksaApiClient, PratyaksaMode, SharedPratyaksaState,
};
use actix_web::{web, HttpResponse};

/// GET /api/v1/pratyaksa/status — status koneksi ke PRATYAKSA API + mode aktif
pub async fn get_status(state: SharedPratyaksaState) -> HttpResponse {
    let s = state.read();
    let fleet_len = s.fleet_data.len();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "mode": s.mode,
            "manual_mode": s.manual_mode,
            "api_reachable": s.api_reachable,
            "fleet_count": fleet_len,
            "last_health_check": s.last_health_check.map(|i| format!("{}s ago", i.elapsed().as_secs())),
            "last_fleet_poll": s.last_fleet_poll.map(|i| format!("{}s ago", i.elapsed().as_secs())),
        }
    }))
}

/// GET /api/v1/pratyaksa/fleet — data fleet (live dari API atau simulasi)
pub async fn get_fleet(state: SharedPratyaksaState) -> HttpResponse {
    let s = state.read();
    let mode = s.mode.clone();
    let fleet = s.fleet_data.clone();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "mode": mode,
            "fleet": fleet,
            "total": fleet.len(),
        }
    }))
}

/// GET /api/v1/pratyaksa/result/{asset_id} — detail prediksi per alat
pub async fn get_result(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
    path: web::Path<String>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    // Baca state sekali saja
    let mode;
    let api_reachable;
    {
        let s = state.read();
        mode = s.mode.clone();
        api_reachable = s.api_reachable;
    }

    if mode == PratyaksaMode::Live && api_reachable {
        let client_inner = client.get_ref();
        match client_inner.get_result(&asset_id).await {
            Ok(result) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "live",
                "data": result,
            })),
            Err(e) => {
                let simulated = simulator::generate_result(&asset_id);
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "mode": "simulasi (fallback)",
                    "data": simulated,
                    "note": format!("Live fetch gagal: {}", e),
                }))
            }
        }
    } else {
        let simulated = simulator::generate_result(&asset_id);
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "mode": "simulasi",
            "data": simulated,
        }))
    }
}

/// POST /api/v1/pratyaksa/predict — kirim data sensor untuk prediksi
pub async fn post_predict(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
    config: web::Data<AppConfig>,
    body: web::Json<crate::pratyaksa::models::PredictRequest>,
) -> HttpResponse {
    let req = body.into_inner();

    // Validasi: features harus tepat 37 angka
    if req.features.len() != 37 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": format!("features[] harus berisi tepat 37 angka, tapi menerima {}", req.features.len()),
            "expected": 37,
            "received": req.features.len(),
        }));
    }

    // Baca state sekali
    let mode;
    let api_reachable;
    {
        let s = state.read();
        mode = s.mode.clone();
        api_reachable = s.api_reachable;
    }

    if mode == PratyaksaMode::Live && api_reachable {
        let url = format!("{}/predict", config.pratyaksa_api_url);
        let api_key = config.pratyaksa_api_key.clone();

        match client
            .client
            .post(&url)
            .header("X-API-Key", api_key)
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<serde_json::Value>().await {
                        Ok(data) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "mode": "live",
                            "data": data,
                        })),
                        Err(e) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "mode": "simulasi (fallback)",
                            "data": simulator::generate_predict(&req),
                            "note": format!("Parse response gagal: {}", e),
                        })),
                    }
                } else {
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "mode": "simulasi (fallback)",
                        "data": simulator::generate_predict(&req),
                        "note": format!("API response: {}", resp.status()),
                    }))
                }
            }
            Err(e) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "simulasi (fallback)",
                "data": simulator::generate_predict(&req),
                "note": format!("Request gagal: {}", e),
            })),
        }
    } else {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "mode": "simulasi",
            "data": simulator::generate_predict(&req),
        }))
    }
}

/// POST /api/v1/pratyaksa/workorder — buat work order rekomendasi
pub async fn post_workorder(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
    config: web::Data<AppConfig>,
    query: web::Query<WorkOrderQuery>,
) -> HttpResponse {
    // Validasi component
    let allowed_components = ["brake", "hydraulic", "engine", "transmission"];
    if !allowed_components.contains(&query.component.as_str()) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": format!("component harus salah satu dari: {:?}", allowed_components),
        }));
    }

    // Validasi risk_score
    if !(0.0..=1.0).contains(&query.risk_score) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": "risk_score harus antara 0.0 dan 1.0",
        }));
    }

    // Baca state sekali
    let mode;
    let api_reachable;
    {
        let s = state.read();
        mode = s.mode.clone();
        api_reachable = s.api_reachable;
    }

    if mode == PratyaksaMode::Live && api_reachable {
        let url = format!(
            "{}/workorder?component={}&risk_score={}",
            config.pratyaksa_api_url, query.component, query.risk_score
        );
        let api_key = config.pratyaksa_api_key.clone();

        match client
            .client
            .post(&url)
            .header("X-API-Key", api_key)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<serde_json::Value>().await {
                        Ok(data) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "mode": "live",
                            "data": data,
                        })),
                        Err(e) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "mode": "simulasi (fallback)",
                            "data": simulator::generate_workorder(&query.component, query.risk_score),
                            "note": format!("Parse response gagal: {}", e),
                        })),
                    }
                } else {
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "mode": "simulasi (fallback)",
                        "data": simulator::generate_workorder(&query.component, query.risk_score),
                        "note": format!("API response: {}", resp.status()),
                    }))
                }
            }
            Err(e) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "simulasi (fallback)",
                "data": simulator::generate_workorder(&query.component, query.risk_score),
                "note": format!("Request gagal: {}", e),
            })),
        }
    } else {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "mode": "simulasi",
            "data": simulator::generate_workorder(&query.component, query.risk_score),
        }))
    }
}

#[derive(serde::Deserialize)]
pub struct WorkOrderQuery {
    pub component: String,
    pub risk_score: f64,
}

/// GET /api/v1/pratyaksa/fleet/health — ringkasan kesehatan fleet ala PRATYAKSA
pub async fn get_fleet_health(state: SharedPratyaksaState) -> HttpResponse {
    let s = state.read();
    let fleet = &s.fleet_data;

    let mut normal = 0i64;
    let mut warning = 0i64;
    let mut critical = 0i64;

    for asset in fleet {
        match asset.risk_level.as_str() {
            "NORMAL" => normal += 1,
            "WARNING" => warning += 1,
            "CRITICAL" => critical += 1,
            _ => {}
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "mode": s.mode,
            "total": fleet.len(),
            "normal": normal,
            "warning": warning,
            "critical": critical,
            "avg_rul_hours": if fleet.is_empty() {
                0.0
            } else {
                let sum: f64 = fleet.iter().map(|a| a.lstm_rul_hours).sum();
                (sum / fleet.len() as f64 * 10.0).round() / 10.0
            },
        }
    }))
}

/// GET /api/v1/pratyaksa/features — daftar 37 nama fitur sensor
pub async fn get_features(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
) -> HttpResponse {
    let mode;
    let api_reachable;
    {
        let s = state.read();
        mode = s.mode.clone();
        api_reachable = s.api_reachable;
    }

    if mode == PratyaksaMode::Live && api_reachable {
        let client_inner = client.get_ref();
        match client_inner.get_features().await {
            Ok(features) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "live",
                "data": features,
            })),
            Err(e) => {
                let simulated = simulator::generate_features();
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "mode": "simulasi (fallback)",
                    "data": simulated,
                    "note": format!("Live fetch gagal: {}", e),
                }))
            }
        }
    } else {
        let simulated = simulator::generate_features();
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "mode": "simulasi",
            "data": simulated,
        }))
    }
}

/// POST /api/v1/pratyaksa/reload-models — reload model ML dari server DS
pub async fn post_reload_models(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
) -> HttpResponse {
    let mode;
    let api_reachable;
    {
        let s = state.read();
        mode = s.mode.clone();
        api_reachable = s.api_reachable;
    }

    if mode == PratyaksaMode::Live && api_reachable {
        let client_inner = client.get_ref();
        match client_inner.post_reload_models().await {
            Ok(res) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "live",
                "data": res,
            })),
            Err(e) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "simulasi (fallback)",
                "data": simulator::generate_reload_models(),
                "note": format!("Live reload gagal: {}", e),
            })),
        }
    } else {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "mode": "simulasi",
            "data": simulator::generate_reload_models(),
        }))
    }
}

/// GET /api/v1/pratyaksa/explain/{prediction_id} — SHAP explanation untuk prediksi tertentu
pub async fn get_explain(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
    path: web::Path<String>,
) -> HttpResponse {
    let prediction_id = path.into_inner();

    let mode;
    let api_reachable;
    {
        let s = state.read();
        mode = s.mode.clone();
        api_reachable = s.api_reachable;
    }

    if mode == PratyaksaMode::Live && api_reachable {
        let client_inner = client.get_ref();
        match client_inner.get_explain(&prediction_id).await {
            Ok(data) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "live",
                "data": data,
            })),
            Err(e) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "simulasi (fallback)",
                "data": simulator::generate_explain(),
                "note": format!("Live explain gagal: {}", e),
            })),
        }
    } else {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "mode": "simulasi",
            "data": simulator::generate_explain(),
        }))
    }
}

/// POST /api/v1/pratyaksa/mode — set mode manual (live / simulasi)
/// Body: { "mode": "live" } atau { "mode": "simulasi" }
/// Kirim { "mode": null } atau { "reset": true } untuk kembali ke auto-detect.
pub async fn mode_switch(
    state: SharedPratyaksaState,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let mode_val = body.get("mode");

    match mode_val {
        Some(val) if val.is_string() => {
            let mode_str = val.as_str().unwrap_or("");
            match mode_str {
                "live" => {
                    let mut s = state.write();
                    s.manual_mode = Some(PratyaksaMode::Live);
                    s.mode = PratyaksaMode::Live;
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "message": "Mode diatur ke LIVE. Semua data akan diambil dari endpoint eksternal.",
                        "data": { "mode": "live", "manual_mode": "live" }
                    }))
                }
                "simulasi" => {
                    let mut s = state.write();
                    s.manual_mode = Some(PratyaksaMode::Simulasi);
                    s.mode = PratyaksaMode::Simulasi;
                    s.api_reachable = false;
                    s.fleet_data = simulator::generate_fleet();
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "message": "Mode diatur ke SIMULASI. Data dari simulator internal.",
                        "data": { "mode": "simulasi", "manual_mode": "simulasi" }
                    }))
                }
                _ => HttpResponse::BadRequest().json(serde_json::json!({
                    "status": "error",
                    "message": "mode harus 'live' atau 'simulasi', atau kirim reset: true untuk auto-detect"
                })),
            }
        }
        _ => {
            // Reset ke auto-detect
            let mut s = state.write();
            s.manual_mode = None;
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Mode di-reset ke AUTO. Backend akan mendeteksi otomatis dari health check.",
                "data": { "mode": s.mode, "manual_mode": null }
            }))
        }
    }
}