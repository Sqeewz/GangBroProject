use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{
    Json, Router, extract::State, http::{
        Method, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    }, response::IntoResponse
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    application::use_cases::brawlers::BrawlersUseCase, config::config_model::DotEnvyConfig, domain::{repositories::brawlers::BrawlerRepository, value_objects::brawler_model::RegisterBrawlerModel}, infrastructure::{database::postgresql_connection::PgPoolSquad, http::routers::{self}}
};

fn static_serve() -> Router {
    let dir = "statics";

    let service = ServeDir::new(dir).not_found_service(ServeFile::new(format!("{dir}/index.html")));

    Router::new().fallback_service(service)
}

fn api_serve(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .nest("/brawler", routers::brawlers::routes(Arc::clone(&db_pool)))
        .nest("/auth", routers::authentication::routes(Arc::clone(&db_pool)))
        .nest("/mission-management", routers::missions_management::routes(Arc::clone(&db_pool)))
    .fallback(|| async { (StatusCode::NOT_FOUND, "API not found") })

}

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .merge(static_serve())
        .nest("/api", api_serve(db_pool))
        // .fallback(default_router::health_check)
        // .route("/health_check", get(default_router::health_check)
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_origin(Any)
                .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;

    info!("Server start on port {}", config.server.port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("Fail ctrl + c") };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Receive ctrl + c signal"),
        _ = terminate => info!("Receive terminate signal"),
    }
}

pub async fn register<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Json(register_brawler_model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.register(register_brawler_model).await {
        Ok(brawler_id) => (StatusCode::CREATED, brawler_id.to_string()).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}