use crate::models::live::{
    LiveDriftLog, LiveFleetSnapshot, LivePrediction, LiveSensorReading, LiveWorkOrder,
};
use mongodb::{Client, Collection, Database};
use serde::Serialize;
use tokio::sync::mpsc;
use tracing::{error, info};

#[derive(Clone)]
pub struct MongoDb {
    pub db: Database,
    /// Channel sender for batch-inserting predictions at high speed
    pub prediction_tx: mpsc::UnboundedSender<LivePrediction>,
    /// Channel sender for batch-inserting fleet snapshots
    pub fleet_tx: mpsc::UnboundedSender<LiveFleetSnapshot>,
    /// Channel sender for batch-inserting sensor readings
    pub sensor_tx: mpsc::UnboundedSender<LiveSensorReading>,
    /// Channel sender for batch-inserting drift logs
    pub drift_tx: mpsc::UnboundedSender<LiveDriftLog>,
    /// Channel sender for batch-inserting work orders
    pub work_order_tx: mpsc::UnboundedSender<LiveWorkOrder>,
}

impl MongoDb {
    pub async fn new(mongodb_url: &str, db_name: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(mongodb_url).await?;

        client
            .database("admin")
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await?;

        let db = client.database(db_name);

        let (prediction_tx, prediction_rx) = mpsc::unbounded_channel();
        let (fleet_tx, fleet_rx) = mpsc::unbounded_channel();
        let (sensor_tx, sensor_rx) = mpsc::unbounded_channel();
        let (drift_tx, drift_rx) = mpsc::unbounded_channel();
        let (work_order_tx, work_order_rx) = mpsc::unbounded_channel();

        let prediction_coll: Collection<LivePrediction> = db.collection("live_predictions");
        let fleet_coll: Collection<LiveFleetSnapshot> = db.collection("live_fleet_snapshots");
        let sensor_coll: Collection<LiveSensorReading> = db.collection("live_sensor_readings");
        let drift_coll: Collection<LiveDriftLog> = db.collection("live_drift_logs");
        let wo_coll: Collection<LiveWorkOrder> = db.collection("live_work_orders");

        // Spawn batch consumer tasks — each flushes every 500ms or 100 items
        tokio::spawn(batch_consumer(
            prediction_rx,
            prediction_coll,
            "predictions",
            100,
            500,
        ));
        tokio::spawn(batch_consumer(fleet_rx, fleet_coll, "fleet", 100, 1000));
        tokio::spawn(batch_consumer(sensor_rx, sensor_coll, "sensor_readings", 100, 500));
        tokio::spawn(batch_consumer(drift_rx, drift_coll, "drift_logs", 50, 2000));
        tokio::spawn(batch_consumer(work_order_rx, wo_coll, "work_orders", 50, 2000));

        info!("MongoDB batch consumers spawned for live API data");

        Ok(Self {
            db,
            prediction_tx,
            fleet_tx,
            sensor_tx,
            drift_tx,
            work_order_tx,
        })
    }

    pub fn collection<T: Send + Sync>(&self, name: &str) -> Collection<T> {
        self.db.collection(name)
    }
}

/// Batch consumer loop: accumulates items on a channel and bulk-inserts
/// into MongoDB when `max_batch` items accumulate or `interval_ms` elapses.
async fn batch_consumer<T>(
    mut rx: mpsc::UnboundedReceiver<T>,
    coll: Collection<T>,
    name: &str,
    max_batch: usize,
    interval_ms: u64,
) where
    T: Send + Sync + Serialize + 'static,
{
    let mut batch: Vec<T> = Vec::with_capacity(max_batch);
    let mut timer = tokio::time::interval(tokio::time::Duration::from_millis(interval_ms));
    timer.tick().await; // skip first immediate tick

    loop {
        tokio::select! {
            _ = timer.tick() => {
                if !batch.is_empty() {
                    flush_batch(&coll, &mut batch, name).await;
                }
            }
            item = rx.recv() => {
                match item {
                    Some(item) => {
                        batch.push(item);
                        if batch.len() >= max_batch {
                            flush_batch(&coll, &mut batch, name).await;
                        }
                    }
                    None => break,
                }
            }
        }
    }
    // Flush remaining on shutdown
    if !batch.is_empty() {
        flush_batch(&coll, &mut batch, name).await;
    }
}

async fn flush_batch<T>(
    coll: &Collection<T>,
    batch: &mut Vec<T>,
    name: &str,
) where
    T: Send + Sync + Serialize,
{
    if batch.is_empty() {
        return;
    }
    let items = std::mem::take(batch);
    let count = items.len();
    if let Err(e) = coll.insert_many(items).await {
        error!("MongoDB batch insert failed for {}: {}", name, e);
    } else {
        info!("MongoDB batch inserted {} {} records", count, name);
    }
}

/// High-speed insert for a single prediction (non-blocking via channel)
pub fn store_prediction(mongo: &MongoDb, pred: LivePrediction) {
    let _ = mongo.prediction_tx.send(pred);
}

pub fn store_fleet_snapshot(mongo: &MongoDb, snapshot: LiveFleetSnapshot) {
    let _ = mongo.fleet_tx.send(snapshot);
}

pub fn store_sensor_reading(mongo: &MongoDb, reading: LiveSensorReading) {
    let _ = mongo.sensor_tx.send(reading);
}

pub fn store_drift_log(mongo: &MongoDb, log: LiveDriftLog) {
    let _ = mongo.drift_tx.send(log);
}

pub fn store_work_order(mongo: &MongoDb, wo: LiveWorkOrder) {
    let _ = mongo.work_order_tx.send(wo);
}
