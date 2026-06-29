use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, ResponseError};
use dotenvy::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod errors;
mod middleware;
mod models;
mod pratyaksa;
mod routes;

use config::AppConfig;
use db::{mongo::MongoDb, postgres::PostgresDb};
use pratyaksa::PratyaksaApiClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let config = AppConfig::from_env().expect("Failed to load configuration");

    info!("🚀 Starting Pratyaksa Backend v{}", env!("CARGO_PKG_VERSION"));
    info!("📦 Connecting to PostgreSQL...");

    // Initialize PostgreSQL
    let pg_db = PostgresDb::new(&config.database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Run migrations
    pg_db.run_migrations().await.expect("Failed to run migrations");
    info!("✅ PostgreSQL connected and migrations applied");

    info!("📦 Connecting to MongoDB...");

    // Initialize MongoDB
    let mongo_db = MongoDb::new(&config.mongodb_url, &config.mongodb_name)
        .await
        .expect("Failed to connect to MongoDB");
    info!("✅ MongoDB connected");

    // Initialize shared PRATYAKSA state
    let pratyaksa_state = pratyaksa::new_shared_state();
    let pratyaksa_state_clone = pratyaksa_state.clone();

    // Initialize PRATYAKSA API Client (sekali, di-share via web::Data)
    let pratyaksa_client = PratyaksaApiClient::new(&config);
    let pratyaksa_client_clone = pratyaksa_client.clone();
    let polling_interval = config.pratyaksa_poll_interval_secs;

    // Spawn background polling task untuk PRATYAKSA API
    tokio::spawn(async move {
        pratyaksa::start_polling(pratyaksa_state_clone, pratyaksa_client_clone, polling_interval).await;
    });

    info!("🔁 PRATYAKSA background polling task started");

    let pg_data = web::Data::new(pg_db);
    let mongo_data = web::Data::new(mongo_db);
    let config_data = web::Data::new(config.clone());

    let host = config.server_host.clone();
    let port = config.server_port;

    info!("🌐 Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _| true)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(pg_data.clone())
            .app_data(mongo_data.clone())
            .app_data(config_data.clone())
            .app_data(pratyaksa_state.clone())
            .app_data(web::Data::new(pratyaksa_client.clone()))
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let response = errors::AppError::ValidationError(err.to_string())
                    .error_response();
                actix_web::error::InternalError::from_response(err, response).into()
            }))
            .configure(routes::configure)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
