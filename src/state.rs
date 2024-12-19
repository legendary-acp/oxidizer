use mongodb::{Database, Collection};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub collection: Arc<Collection<Value>>,
}