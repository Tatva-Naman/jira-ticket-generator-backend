use axum::http::HeaderValue;
use reqwest::Method;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
mod config;
mod dtos;
mod error;
mod handlers;
mod routes;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);
    let config = config::AppConfig::from_env()?;
    let app = routes::subtasks_routes::create_routes(std::sync::Arc::new(config.clone()))
    .merge(routes::story_routes::story_routes(std::sync::Arc::new(config.clone())))
    .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Starting server at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
