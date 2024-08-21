use crate::api::query_danmu_statistics_data_from_db;
use crate::error::AppError;
use crate::model::{DanmuStatisticsResponse, QueryBlockerResponse, QueryStatisticsData};
use anyhow::Result;
use axum::http::Method;
use axum::routing::get;
use axum::Router;
use chrono::{Duration, Utc};
use duckdb::DuckdbConnectionManager;
use moka::future::Cache;
use queryer::Queryer;
use r2d2::Pool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utils::utils::{get_local_midnight, get_rooms};

mod api;
pub mod error;
pub mod model;

#[derive(Clone)]
struct AppState {
    queryer: Arc<Queryer>,
    statistics_cache: Arc<Cache<(i64, i64), QueryStatisticsData>>,
    block_user_cache: Arc<Cache<(usize, usize), QueryBlockerResponse>>,
    danmu_statistics_cache: Arc<Cache<(i64, i64, i64), DanmuStatisticsResponse>>,
}

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
    let statistics_cache: Cache<(i64, i64), QueryStatisticsData> = Cache::builder()
        .time_to_live(Duration::minutes(10).to_std()?)
        .build();

    let block_user_cache: Cache<(usize, usize), QueryBlockerResponse> = Cache::builder()
        .time_to_live(Duration::minutes(10).to_std()?)
        .build();

    let danmu_statistics_cache = Cache::builder()
        .time_to_live(Duration::minutes(10).to_std()?)
        .build();

    let state = AppState {
        queryer,
        statistics_cache: Arc::new(statistics_cache),
        block_user_cache: Arc::new(block_user_cache),
        danmu_statistics_cache: Arc::new(danmu_statistics_cache),
    };

    let cache_state = state.clone();

    // 预热 danmu_statistics_cache
    tokio::spawn(async move {
        info!("开始预热缓存");
        let end = match get_local_midnight(Utc::now().timestamp() - 24 * 60 * 60)
            .map_err(|_| AppError::QueryError)
        {
            Ok(t) => t,
            Err(e) => {
                info!("预热缓存错误: {}", e);
                return;
            }
        };
        let start = end - 30 * 24 * 60 * 60;
        for room_id in get_rooms() {
            match query_danmu_statistics_data_from_db(
                cache_state.queryer.clone(),
                room_id,
                start,
                end,
            )
            .await
            {
                Ok(resp) => {
                    cache_state
                        .danmu_statistics_cache
                        .insert((room_id, start, end), resp)
                        .await;
                    info!("加载 {} 缓存成功 {} - {}", room_id, start, end);
                }
                Err(e) => {
                    info!("预热缓存错误: {}", e);
                }
            }
        }
    });

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/:room_id", get(api::query))
        .route("/api/checker", get(api::checker))
        .route("/api/statistics", get(api::query_statistics))
        .route("/api/block_user", get(api::query_block_user))
        .route("/api/danmu_statistics", get(api::query_danmu_statistics))
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    info!("Listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await?;
    Ok(())
}
