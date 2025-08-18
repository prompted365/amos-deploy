use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use crate::{
    models::metrics::{SystemMetrics, AgentMetrics, SwarmMetrics},
    ApiResult, AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/metrics/system", get(get_system_metrics))
        .route("/metrics/agents", get(get_agent_metrics))
        .route("/metrics/swarms", get(get_swarm_metrics))
}

#[derive(Debug, Deserialize)]
pub struct MetricsQuery {
    #[serde(default = "default_interval")]
    #[allow(dead_code)]
    interval: u64, // seconds
}

fn default_interval() -> u64 {
    60 // 1 minute default
}

#[utoipa::path(
    get,
    path = "/api/v1/metrics/system",
    params(
        ("interval" = u64, Query, description = "Time interval in seconds")
    ),
    responses(
        (status = 200, description = "System metrics", body = SystemMetrics),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "metrics",
)]
pub async fn get_system_metrics(
    State(state): State<AppState>,
    Query(_params): Query<MetricsQuery>,
) -> ApiResult<Json<SystemMetrics>> {
    let agents = state.agents.read().await;
    let swarms = state.swarms.read().await;
    
    let metrics = SystemMetrics {
        cpu_usage: 45.2, // In production, get from system
        memory_usage: 1024 * 1024 * 512, // 512MB
        active_agents: agents.len(),
        active_swarms: swarms.len(),
        neural_pathways: state.neural_network.pathway_count().await,
        neural_nodes: state.neural_network.node_count().await,
        events_processed: 1542, // In production, track this
        timestamp: chrono::Utc::now(),
    };
    
    Ok(Json(metrics))
}

#[utoipa::path(
    get,
    path = "/api/v1/metrics/agents",
    responses(
        (status = 200, description = "Agent metrics", body = Vec<AgentMetrics>),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "metrics",
)]
pub async fn get_agent_metrics(State(state): State<AppState>) -> ApiResult<Json<Vec<AgentMetrics>>> {
    let agents = state.agents.read().await;
    
    let metrics: Vec<AgentMetrics> = agents
        .iter()
        .map(|(id, agent)| AgentMetrics {
            agent_id: *id,
            agent_name: agent.name().to_string(),
            state: format!("{:?}", agent.state()),
            tasks_completed: 0, // In production, track this
            average_response_time: 0.0,
            cpu_usage: 0.0,
            memory_usage: 0,
            last_active: chrono::Utc::now(),
        })
        .collect();
    
    Ok(Json(metrics))
}

#[utoipa::path(
    get,
    path = "/api/v1/metrics/swarms",
    responses(
        (status = 200, description = "Swarm metrics", body = Vec<SwarmMetrics>),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "metrics",
)]
pub async fn get_swarm_metrics(State(state): State<AppState>) -> ApiResult<Json<Vec<SwarmMetrics>>> {
    let swarms = state.swarms.read().await;
    let agents = state.agents.read().await;
    
    let metrics: Vec<SwarmMetrics> = swarms
        .iter()
        .map(|(id, swarm)| {
            let active_agents = swarm.agent_ids.iter()
                .filter(|agent_id| agents.contains_key(agent_id))
                .count();
                
            SwarmMetrics {
                swarm_id: *id,
                swarm_name: swarm.name.clone(),
                total_agents: swarm.agent_ids.len(),
                active_agents,
                tasks_orchestrated: 0, // In production, track this
                average_task_time: 0.0,
                created_at: swarm.created_at,
            }
        })
        .collect();
    
    Ok(Json(metrics))
}