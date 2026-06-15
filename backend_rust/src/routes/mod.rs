use actix_web::web;

pub mod analisa;
pub mod auth;
pub mod dashboard;
pub mod health_analytics;
pub mod jenis_alat_berat;
pub mod telemetry;
pub mod unit_tambang;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health check
            .route("/health", web::get().to(health_check))
            // Auth routes (public)
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth::register))
                    .route("/login", web::post().to(auth::login))
                    .route("/me", web::get().to(auth::me)),
            )
            // Dashboard (protected)
            .service(
                web::scope("/dashboard")
                    .wrap(crate::middleware::AuthMiddleware)
                    .route("", web::get().to(dashboard::get_stats)),
            )
            // Jenis Alat Berat CRUD (protected)
            .service(
                web::scope("/jenis-alat-berat")
                    .wrap(crate::middleware::AuthMiddleware)
                    .route("", web::get().to(jenis_alat_berat::list))
                    .route("", web::post().to(jenis_alat_berat::create))
                    .route("/{id}", web::get().to(jenis_alat_berat::get_by_id))
                    .route("/{id}", web::put().to(jenis_alat_berat::update))
                    .route("/{id}", web::delete().to(jenis_alat_berat::delete)),
            )
            // Unit Tambang CRUD (protected)
            .service(
                web::scope("/unit-tambang")
                    .wrap(crate::middleware::AuthMiddleware)
                    .route("", web::get().to(unit_tambang::list))
                    .route("", web::post().to(unit_tambang::create))
                    .route("/{id}", web::get().to(unit_tambang::get_by_id))
                    .route("/{id}", web::put().to(unit_tambang::update))
                    .route("/{id}", web::delete().to(unit_tambang::delete)),
            )
            // Analisa Kerusakan (protected)
            //  - /overview & /unit/{id}: analitik kesehatan realtime (PostgreSQL)
            //  - sisanya: CRUD laporan manual (MongoDB)
            .service(
                web::scope("/analisa")
                    .wrap(crate::middleware::AuthMiddleware)
                    .route("/overview", web::get().to(health_analytics::get_overview))
                    .route("/unit/{id}", web::get().to(health_analytics::get_unit_analysis))
                    .route("", web::get().to(analisa::list))
                    .route("", web::post().to(analisa::create))
                    .route("/{id}", web::get().to(analisa::get_by_id))
                    .route("/{id}", web::put().to(analisa::update))
                    .route("/{id}", web::delete().to(analisa::delete)),
            )
            // Telemetry ingestion & history (protected, PostgreSQL)
            .service(
                web::scope("/telemetry")
                    .wrap(crate::middleware::AuthMiddleware)
                    .route("", web::post().to(telemetry::create))
                    .route("/unit/{id}", web::get().to(telemetry::list_by_unit)),
            ),
    );
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "Pratyaksa Backend",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
