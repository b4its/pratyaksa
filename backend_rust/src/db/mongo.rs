use mongodb::{Client, Collection, Database};

#[derive(Clone)]
pub struct MongoDb {
    pub db: Database,
}

impl MongoDb {
    pub async fn new(mongodb_url: &str, db_name: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(mongodb_url).await?;

        // Ping to verify connection
        client
            .database("admin")
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await?;

        let db = client.database(db_name);
        Ok(Self { db })
    }

    pub fn collection<T: Send + Sync>(&self, name: &str) -> Collection<T> {
        self.db.collection(name)
    }
}
