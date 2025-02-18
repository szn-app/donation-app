// https://docs.rs/tower/latest/tower/trait.Service.html

mod auth;

use axum;
use std::env;

#[tokio::main]
async fn main() {
    let client_secret = env::var("HYDRA_CLIENT_SECRET").expect("HYDRA_CLIENT_SECRET not set");

    let app = auth::routes().layer(axum::Extension(client_secret));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
