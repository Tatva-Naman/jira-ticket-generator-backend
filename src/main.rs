use std::net::SocketAddr;

use axum::http::HeaderValue;
use config::AppConfig;
use reqwest::Method;
use tower_http::cors::{Any, CorsLayer};
mod routes;
mod handlers;
mod services;
mod dtos;
mod config;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any);
    let config = config::AppConfig::from_env()?;
    println!("✅ Loaded config");

    let app = routes::jira_apis::create_routes(std::sync::Arc::new(config)).layer(cors);
    // let app = routes::jira_apis::create_routes(config);

    println!("Server running on port 8080");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Starting server at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}