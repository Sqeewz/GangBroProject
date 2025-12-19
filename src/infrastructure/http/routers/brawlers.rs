use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{application::use_cases::brawlers::BrawlersUseCase, domain::{repositories::brawlers::BrawlerRepository, value_objects::brawler_model::RegisterBrawlerModel}, infrastructure::database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres}};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawler_repository = BrawlerPostgres::new(db_pool);
    let brawlers_use_case = BrawlersUseCase::new(Arc::new(brawler_repository));

    Router::new()
        .route("/register", post(register))
        .with_state(Arc::new(brawlers_use_case))
    // .with_state(x) ทำให้สามารถเข้าถึง x นั้นในทุก route handler
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