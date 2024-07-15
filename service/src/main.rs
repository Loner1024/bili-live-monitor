use anyhow::Result;
use axum::routing::get;
use axum::Router;
use duckdb::DuckdbConnectionManager;
use queryer::Query;
use r2d2::Pool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
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
    let queryer = Arc::new(Query::new(pool)?);

    let app = Router::new()
        .route("/api/:room_id", get(api::query))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(queryer);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
