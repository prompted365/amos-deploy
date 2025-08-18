pub mod routes;
pub mod auth;
pub mod state;
pub mod error;
pub mod models;
pub mod websocket;

pub use error::{ApiError, ApiResult};
pub use state::AppState;

use axum::{Router, middleware};
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::agents::list_agents,
        routes::agents::get_agent,
        routes::agents::create_agent,
        routes::agents::delete_agent,
        routes::agents::send_agent_command,
        routes::neural::get_neural_state,
        routes::neural::update_neural_pathway,
        routes::swarm::create_swarm,
        routes::swarm::list_swarms,
        routes::swarm::orchestrate_task,
        routes::hormonal::get_hormonal_levels,
        routes::hormonal::update_hormonal_levels,
        routes::metrics::get_system_metrics,
        routes::metrics::get_agent_metrics,
        routes::metrics::get_swarm_metrics,
        routes::auth::login,
        routes::auth::refresh_token,
    ),
    components(
        schemas(
            models::agent::AgentInfo,
            models::agent::CreateAgentRequest,
            models::agent::AgentCommand,
            models::neural::NeuralState,
            models::neural::PathwayUpdate,
            models::swarm::SwarmInfo,
            models::swarm::CreateSwarmRequest,
            models::swarm::OrchestrateTaskRequest,
            models::neural::HormonalUpdate,
            models::metrics::SystemMetrics,
            models::metrics::AgentMetrics,
            models::metrics::SwarmMetrics,
            routes::auth::LoginRequest,
            routes::auth::LoginResponse,
            routes::auth::RefreshRequest,
        )
    ),
    tags(
        (name = "agents", description = "Agent management operations"),
        (name = "neural", description = "Neural network operations"),
        (name = "swarm", description = "Swarm orchestration operations"),
        (name = "hormonal", description = "Hormonal system control"),
        (name = "metrics", description = "Performance metrics and monitoring"),
        (name = "auth", description = "Authentication endpoints"),
    )
)]
pub struct ApiDoc;

pub fn create_app(state: AppState) -> Router {
    // Start neural activity broadcaster
    websocket::start_neural_activity_broadcaster(state.clone());
    
    let api_routes = Router::new()
        .merge(routes::agents::router())
        .merge(routes::neural::router())
        .merge(routes::swarm::router())
        .merge(routes::hormonal::router())
        .merge(routes::metrics::router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::auth_middleware,
        ));
    
    // Auth routes without middleware
    let auth_routes = routes::auth::router();

    Router::new()
        .nest("/api/v1", api_routes)
        .nest("/api/v1", auth_routes)
        .route("/ws", axum::routing::get(websocket::websocket_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(routes::health::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_app(AppState::test());
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}