use anyhow::Result;
use axum::http::Method;
use axum::routing::get;
use axum::Router;
use duckdb::DuckdbConnectionManager;
use queryer::Queryer;
use r2d2::Pool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
pub mod error;
pub mod model;

#[tokio::main]
async fn main() -> Result<()> {
    use dotenv::dotenv;
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("debug,hyper=off".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let manager = DuckdbConnectionManager::memory()?;
    let pool = Pool::new(manager)?;
    let queryer = Arc::new(Queryer::new(pool)?);

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/:room_id", get(api::query))
        .route("/api/checker", get(api::checker))
        .route("/api/statistics", get(api::query_statistics))
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(queryer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    info!("Listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await?;
    Ok(())
}
