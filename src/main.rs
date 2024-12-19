mod routes;
mod handlers;
mod state;

use tokio;
use tracing_subscriber;
use mongodb::{Client, options::ClientOptions};
use state::AppState;
use std::sync::Arc;
use axum::Extension;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize MongoDB client
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    // Get the database and collection
    let db = client.database("mydb");
    let collection = db.collection("mycollection");

    // Create shared state
    let app_state = AppState {
        db: Arc::new(db),
        collection: Arc::new(collection),
    };

    // Get the router from routes.rs
    let app = routes::create_router().layer(Extension(app_state));

    // Define address to run the server
    let addr = "127.0.0.1:3000";
    println!("Server running on http://{}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}