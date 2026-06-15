use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

#[derive(Clone)]
pub struct PostgresDb {
    pub pool: PgPool,
}

impl PostgresDb {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .acquire_timeout(Duration::from_secs(10))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }
}
