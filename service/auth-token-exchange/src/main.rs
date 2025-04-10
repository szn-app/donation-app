// https://docs.rs/tower/latest/tower/trait.Service.html

mod auth;

use axum;
use env_logger;
use log;
use std::env;

#[tokio::main]
async fn main() {
    // Initialize logger with appropriate level based on environment
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let client_secret = env::var("HYDRA_CLIENT_SECRET").expect("HYDRA_CLIENT_SECRET not set");

    let app = auth::routes().layer(axum::Extension(client_secret));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
