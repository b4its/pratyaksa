use crate::config::AppConfig;
use crate::pratyaksa::{
    simulator, PratyaksaApiClient, PratyaksaMode, SharedPratyaksaState,
};
use actix_web::{web, HttpResponse};

/// Helper: baca mode + reachable dari state
fn read_mode(state: &SharedPratyaksaState) -> (PratyaksaMode, bool) {
    let s = state.read();
    (s.mode.clone(), s.api_reachable)
}

/// GET /api/v1/pratyaksa/status
pub async fn get_status(state: SharedPratyaksaState) -> HttpResponse {
    let s = state.read();
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "mode": s.mode,
            "manual_mode": s.manual_mode,
            "api_reachable": s.api_reachable,
            "fleet_count": s.fleet_data.len(),
            "last_health_check": s.last_health_check.map(|i| format!("{}s ago", i.elapsed().as_secs())),
            "last_fleet_poll": s.last_fleet_poll.map(|i| format!("{}s ago", i.elapsed().as_secs())),
        }
    }))
}

/// GET /api/v1/pratyaksa/fleet
pub async fn get_fleet(state: SharedPratyaksaState) -> HttpResponse {
    let s = state.read();
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "mode": s.mode,
            "fleet": s.fleet_data,
            "total": s.fleet_data.len(),
        }
    }))
}

/// GET /api/v1/pratyaksa/fleet/health
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
            "avg_rul_hours": if fleet.is_empty() { 0.0 } else {
                let sum: f64 = fleet.iter().map(|a| a.lstm_rul_hours).sum();
                (sum / fleet.len() as f64 * 10.0).round() / 10.0
            },
        }
    }))
}

// ── Endpoint yang panggil API eksternal — pakai live_only_call helper ──

/// Panggil API eksternal. Di LIVE mode: return data atau error (TIDAK fallback ke simulator).
/// Di SIMULASI mode: langsung panggil closure `sim` untuk hasil simulasi.
async fn live_or_sim<T, F>(
    state: &SharedPratyaksaState,
    live: impl std::future::Future<Output = Result<T, String>>,
    sim: F,
) -> HttpResponse
where
    T: serde::Serialize,
    F: FnOnce() -> T,
{
    let (mode, reachable) = read_mode(state);

    if mode == PratyaksaMode::Live && reachable {
        match live.await {
            Ok(data) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": "live",
                "data": data,
            })),
            Err(e) => HttpResponse::BadGateway().json(serde_json::json!({
                "status": "error",
                "mode": "live",
                "message": format!("API Eksternal gagal: {}", e),
                "hint": "Coba ganti ke mode SIMULASI jika ingin data simulasi."
            })),
        }
    } else {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "mode": "simulasi",
            "data": sim(),
        }))
    }
}

/// GET /api/v1/pratyaksa/result/{asset_id}
pub async fn get_result(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
    path: web::Path<String>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    live_or_sim(
        &state,
        client.get_ref().get_result(&asset_id),
        || simulator::generate_result(&asset_id),
    )
    .await
}

/// POST /api/v1/pratyaksa/predict
pub async fn post_predict(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
    config: web::Data<AppConfig>,
    body: web::Json<crate::pratyaksa::models::PredictRequest>,
) -> HttpResponse {
    let req = body.into_inner();

    if req.features.len() != 37 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": format!("features[] harus berisi tepat 37 angka, tapi menerima {}", req.features.len()),
            "expected": 37,
            "received": req.features.len(),
        }));
    }

    let (mode, reachable) = read_mode(&state);

    if mode == PratyaksaMode::Live && reachable {
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
                            "status": "success", "mode": "live", "data": data,
                        })),
                        Err(e) => HttpResponse::BadGateway().json(serde_json::json!({
                            "status": "error", "mode": "live",
                            "message": format!("Gagal parse response API: {}", e),
                        })),
                    }
                } else {
                    HttpResponse::BadGateway().json(serde_json::json!({
                        "status": "error", "mode": "live",
                        "message": format!("API respond HTTP {}", resp.status()),
                    }))
                }
            }
            Err(e) => HttpResponse::BadGateway().json(serde_json::json!({
                "status": "error", "mode": "live",
                "message": format!("Gagal hubungi API: {}", e),
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

/// POST /api/v1/pratyaksa/workorder
pub async fn post_workorder(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
    config: web::Data<AppConfig>,
    query: web::Query<WorkOrderQuery>,
) -> HttpResponse {
    let allowed_components = ["brake", "hydraulic", "engine", "transmission"];
    if !allowed_components.contains(&query.component.as_str()) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": format!("component harus: {:?}", allowed_components),
        }));
    }
    if !(0.0..=1.0).contains(&query.risk_score) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": "risk_score harus 0.0 - 1.0",
        }));
    }

    let (mode, reachable) = read_mode(&state);

    if mode == PratyaksaMode::Live && reachable {
        let url = format!(
            "{}/workorder?component={}&risk_score={}",
            config.pratyaksa_api_url, query.component, query.risk_score
        );
        let api_key = config.pratyaksa_api_key.clone();

        match client.client.post(&url).header("X-API-Key", api_key).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<serde_json::Value>().await {
                        Ok(data) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success", "mode": "live", "data": data,
                        })),
                        Err(e) => HttpResponse::BadGateway().json(serde_json::json!({
                            "status": "error", "mode": "live",
                            "message": format!("Gagal parse response: {}", e),
                        })),
                    }
                } else {
                    HttpResponse::BadGateway().json(serde_json::json!({
                        "status": "error", "mode": "live",
                        "message": format!("API respond HTTP {}", resp.status()),
                    }))
                }
            }
            Err(e) => HttpResponse::BadGateway().json(serde_json::json!({
                "status": "error", "mode": "live",
                "message": format!("Gagal hubungi API: {}", e),
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

/// GET /api/v1/pratyaksa/features
pub async fn get_features(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
) -> HttpResponse {
    live_or_sim(
        &state,
        client.get_ref().get_features(),
        simulator::generate_features,
    )
    .await
}

/// POST /api/v1/pratyaksa/reload-models
pub async fn post_reload_models(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
) -> HttpResponse {
    live_or_sim(
        &state,
        client.get_ref().post_reload_models(),
        simulator::generate_reload_models,
    )
    .await
}

/// GET /api/v1/pratyaksa/explain/{prediction_id}
pub async fn get_explain(
    state: SharedPratyaksaState,
    client: web::Data<PratyaksaApiClient>,
    path: web::Path<String>,
) -> HttpResponse {
    let prediction_id = path.into_inner();

    live_or_sim(
        &state,
        client.get_ref().get_explain(&prediction_id),
        simulator::generate_explain,
    )
    .await
}

/// POST /api/v1/pratyaksa/mode
pub async fn mode_switch(
    state: SharedPratyaksaState,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let mode_val = body.get("mode");

    match mode_val {
        Some(val) if val.is_string() => {
            match val.as_str().unwrap_or("") {
                "live" => {
                    let mut s = state.write();
                    s.manual_mode = Some(PratyaksaMode::Live);
                    s.mode = PratyaksaMode::Live;
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "message": "Mode diatur ke LIVE. Semua data dari endpoint eksternal.",
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
                    "message": "mode harus 'live' atau 'simulasi', atau reset:true untuk auto-detect"
                })),
            }
        }
        _ => {
            let mut s = state.write();
            s.manual_mode = None;
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Mode di-reset ke AUTO.",
                "data": { "mode": s.mode, "manual_mode": null }
            }))
        }
    }
}