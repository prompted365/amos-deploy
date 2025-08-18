use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use crate::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    agents_count: usize,
    swarms_count: usize,
    neural_network_active: bool,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let agents_count = state.agents.read().await.len();
    let swarms_count = state.swarms.read().await.len();

    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        agents_count,
        swarms_count,
        neural_network_active: true,
    })
}