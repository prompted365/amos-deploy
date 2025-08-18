use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use uuid::Uuid;
use crate::{
    models::swarm::{
        SwarmInfo, CreateSwarmRequest, OrchestrateTaskRequest,
        SwarmStatus, TaskResult, TaskStatus,
    },
    state::SwarmState,
    ApiError, ApiResult, AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/swarms", get(list_swarms).post(create_swarm))
        .route("/swarms/:id/orchestrate", post(orchestrate_task))
}

#[utoipa::path(
    get,
    path = "/api/v1/swarms",
    responses(
        (status = 200, description = "List all swarms", body = Vec<SwarmInfo>),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "swarm",
)]
pub async fn list_swarms(State(state): State<AppState>) -> ApiResult<Json<Vec<SwarmInfo>>> {
    let swarms = state.swarms.read().await;
    
    let swarm_list: Vec<SwarmInfo> = swarms
        .values()
        .map(|swarm| SwarmInfo {
            id: swarm.id,
            name: swarm.name.clone(),
            agent_count: swarm.agent_ids.len(),
            status: SwarmStatus::Idle,
            created_at: swarm.created_at,
            active_tasks: 0,
        })
        .collect();
    
    Ok(Json(swarm_list))
}

#[utoipa::path(
    post,
    path = "/api/v1/swarms",
    request_body = CreateSwarmRequest,
    responses(
        (status = 201, description = "Swarm created", body = SwarmInfo),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "swarm",
)]
pub async fn create_swarm(
    State(state): State<AppState>,
    Json(request): Json<CreateSwarmRequest>,
) -> ApiResult<Json<SwarmInfo>> {
    // Validate all agent IDs exist
    let agents = state.agents.read().await;
    for agent_id in &request.agent_ids {
        if !agents.contains_key(agent_id) {
            return Err(ApiError::BadRequest(format!("Agent {} not found", agent_id)));
        }
    }
    drop(agents);
    
    let swarm_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    
    let swarm_state = SwarmState {
        id: swarm_id,
        name: request.name.clone(),
        agent_ids: request.agent_ids.clone(),
        created_at: now,
    };
    
    let swarm_info = SwarmInfo {
        id: swarm_id,
        name: request.name,
        agent_count: request.agent_ids.len(),
        status: SwarmStatus::Idle,
        created_at: now,
        active_tasks: 0,
    };
    
    state.swarms.write().await.insert(swarm_id, swarm_state);
    
    Ok(Json(swarm_info))
}

#[utoipa::path(
    post,
    path = "/api/v1/swarms/{id}/orchestrate",
    request_body = OrchestrateTaskRequest,
    responses(
        (status = 200, description = "Task orchestrated", body = TaskResult),
        (status = 404, description = "Swarm not found"),
        (status = 401, description = "Unauthorized"),
    ),
    params(
        ("id" = Uuid, Path, description = "Swarm ID"),
    ),
    tag = "swarm",
)]
pub async fn orchestrate_task(
    State(state): State<AppState>,
    Path(swarm_id): Path<Uuid>,
    Json(request): Json<OrchestrateTaskRequest>,
) -> ApiResult<Json<TaskResult>> {
    let swarms = state.swarms.read().await;
    let swarm = swarms
        .get(&swarm_id)
        .ok_or_else(|| ApiError::NotFound(format!("Swarm {} not found", swarm_id)))?;
    
    // Get agents for this swarm
    let agents = state.agents.read().await;
    let swarm_agents: Vec<_> = swarm.agent_ids
        .iter()
        .filter_map(|id| agents.get(id))
        .collect();
    
    if swarm_agents.is_empty() {
        return Err(ApiError::BadRequest("Swarm has no active agents".to_string()));
    }
    
    // In a real implementation, distribute the task across agents
    // For now, simulate task execution
    let task_id = Uuid::new_v4();
    let start_time = std::time::Instant::now();
    
    // Simulate some processing
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let result = TaskResult {
        task_id,
        status: TaskStatus::Completed,
        result: Some(serde_json::json!({
            "message": "Task completed successfully",
            "agents_used": swarm_agents.len(),
            "strategy": format!("{:?}", request.strategy),
        })),
        error: None,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
    };
    
    Ok(Json(result))
}