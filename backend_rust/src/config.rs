use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub mongodb_url: String,
    pub mongodb_name: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub live_api_url: String,
    pub live_api_key: String,
    pub pratyaksa_api_url: String,
    pub pratyaksa_api_key: String,
    pub pratyaksa_poll_interval_secs: u64,
    pub custom_api_url: String,
    /// Connection to ml-pratyaksa's PostgreSQL for data migration/sync
    pub ml_postgres_url: String,
    /// Interval in seconds for syncing data from ml-pratyaksa PostgreSQL
    pub ml_sync_interval_secs: u64,
    /// Batch size for MongoDB inserts
    pub mongo_batch_size: usize,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        let live_api_url = env::var("LIVE_API_URL")
            .or_else(|_| env::var("PRATYAKSA_API_URL"))
            .unwrap_or_else(|_| "http://192.168.101.3:6000".to_string());
        
        let live_api_key = env::var("LIVE_API_KEY")
            .or_else(|_| env::var("PRATYAKSA_API_KEY"))
            .unwrap_or_else(|_| "dev-key-pratyaksa".to_string());

        Ok(AppConfig {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .map_err(|_| "Invalid SERVER_PORT")?,
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set")?,
            mongodb_url: env::var("MONGODB_URL")
                .map_err(|_| "MONGODB_URL must be set")?,
            mongodb_name: env::var("MONGODB_NAME")
                .unwrap_or_else(|_| "pratyaksa".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "super_secret_pratyaksa_key_2024".to_string()),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse::<i64>()
                .unwrap_or(24),
            live_api_url: live_api_url.clone(),
            live_api_key: live_api_key.clone(),
            pratyaksa_api_url: live_api_url,
            pratyaksa_api_key: live_api_key,
            pratyaksa_poll_interval_secs: env::var("PRATYAKSA_POLL_INTERVAL")
                .unwrap_or_else(|_| "5".to_string())
                .parse::<u64>()
                .unwrap_or(5),
            custom_api_url: env::var("CUSTOM_API_URL")
                .unwrap_or_else(|_| "http://192.168.101.3:7000".to_string()),
            ml_postgres_url: env::var("ML_POSTGRES_URL")
                .unwrap_or_else(|_| "postgresql://pratyaksa:pratyaksa_secret@192.168.101.3:5432/pratyaksa".to_string()),
            ml_sync_interval_secs: env::var("ML_SYNC_INTERVAL")
                .unwrap_or_else(|_| "60".to_string())
                .parse::<u64>()
                .unwrap_or(60),
            mongo_batch_size: env::var("MONGO_BATCH_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse::<usize>()
                .unwrap_or(100),
        })
    }
}
