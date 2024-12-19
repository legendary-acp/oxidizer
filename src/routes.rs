use crate::handlers::{
    process_data, get_data, list_data, process_batch, health_check, get_metrics
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router() -> Router {
    Router::new()
        // Core data processing routes
        .route("/api/data", post(process_data))
        .route("/api/data/:id", get(get_data))
        .route("/api/data", get(list_data))
        .route("/api/batch", post(process_batch))
        
        // System routes
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
}