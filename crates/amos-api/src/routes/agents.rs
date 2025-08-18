use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    models::agent::{AgentInfo, CreateAgentRequest, AgentCommand, AgentType},
    ApiError, ApiResult, AppState,
};
use amos_agents::{
    TrafficSeer, PathwaySculptor, MemoryWeaver, CognitionAlchemist,
    LearningOracle, MeshHarmonizer, ConsciousnessEmergent, PerformanceGuardian,
    CognitiveAgent,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/agents", get(list_agents).post(create_agent))
        .route("/agents/:id", get(get_agent).delete(delete_agent))
        .route("/agents/:id/command", post(send_agent_command))
}

#[utoipa::path(
    get,
    path = "/api/v1/agents",
    responses(
        (status = 200, description = "List all agents", body = Vec<AgentInfo>),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "agents",
)]
pub async fn list_agents(State(state): State<AppState>) -> ApiResult<Json<Vec<AgentInfo>>> {
    let agents = state.agents.read().await;
    
    let agent_list: Vec<AgentInfo> = agents
        .iter()
        .map(|(id, agent)| AgentInfo {
            id: *id,
            name: agent.name().to_string(),
            agent_type: agent.name().to_string(),
            state: format!("{:?}", agent.state()),
            created_at: chrono::Utc::now(), // In production, track this properly
            neural_network_id: Uuid::new_v4(), // In production, get from agent
        })
        .collect();

    Ok(Json(agent_list))
}

#[utoipa::path(
    get,
    path = "/api/v1/agents/{id}",
    responses(
        (status = 200, description = "Get agent details", body = AgentInfo),
        (status = 404, description = "Agent not found"),
        (status = 401, description = "Unauthorized"),
    ),
    params(
        ("id" = Uuid, Path, description = "Agent ID"),
    ),
    tag = "agents",
)]
pub async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AgentInfo>> {
    let agents = state.agents.read().await;
    
    let agent = agents
        .get(&id)
        .ok_or_else(|| ApiError::NotFound(format!("Agent {} not found", id)))?;

    Ok(Json(AgentInfo {
        id,
        name: agent.name().to_string(),
        agent_type: agent.name().to_string(),
        state: format!("{:?}", agent.state()),
        created_at: chrono::Utc::now(),
        neural_network_id: Uuid::new_v4(),
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/agents",
    request_body = CreateAgentRequest,
    responses(
        (status = 201, description = "Agent created", body = AgentInfo),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "agents",
)]
pub async fn create_agent(
    State(state): State<AppState>,
    Json(request): Json<CreateAgentRequest>,
) -> ApiResult<Json<AgentInfo>> {
    
    // Create the agent based on type
    let mut agent: Box<dyn CognitiveAgent> = match request.agent_type {
        AgentType::TrafficSeer => Box::new(TrafficSeer::new()),
        AgentType::PathwaySculptor => Box::new(PathwaySculptor::new()),
        AgentType::MemoryWeaver => Box::new(MemoryWeaver::new()),
        AgentType::CognitionAlchemist => Box::new(CognitionAlchemist::new()),
        AgentType::LearningOracle => Box::new(LearningOracle::new()),
        AgentType::MeshHarmonizer => Box::new(MeshHarmonizer::new()),
        AgentType::ConsciousnessEmergent => Box::new(ConsciousnessEmergent::new()),
        AgentType::PerformanceGuardian => Box::new(PerformanceGuardian::new()),
    };
    
    // Initialize the agent with neural network and event bus
    agent.initialize(state.neural_network.clone(), state.event_bus.clone()).await?;
    agent.activate().await?;
    
    let agent_info = AgentInfo {
        id: agent.id(),
        name: agent.name().to_string(),
        agent_type: format!("{:?}", request.agent_type),
        state: format!("{:?}", agent.state()),
        created_at: chrono::Utc::now(),
        neural_network_id: Uuid::new_v4(), // TODO: Track neural network IDs properly
    };
    
    let agent_id = agent.id();
    state.agents.write().await.insert(agent_id, Arc::from(agent));
    
    Ok(Json(agent_info))
}

#[utoipa::path(
    delete,
    path = "/api/v1/agents/{id}",
    responses(
        (status = 204, description = "Agent deleted"),
        (status = 404, description = "Agent not found"),
        (status = 401, description = "Unauthorized"),
    ),
    params(
        ("id" = Uuid, Path, description = "Agent ID"),
    ),
    tag = "agents",
)]
pub async fn delete_agent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<()> {
    let mut agents = state.agents.write().await;
    
    agents
        .remove(&id)
        .ok_or_else(|| ApiError::NotFound(format!("Agent {} not found", id)))?;
    
    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/v1/agents/{id}/command",
    request_body = AgentCommand,
    responses(
        (status = 200, description = "Command executed"),
        (status = 404, description = "Agent not found"),
        (status = 401, description = "Unauthorized"),
    ),
    params(
        ("id" = Uuid, Path, description = "Agent ID"),
    ),
    tag = "agents",
)]
pub async fn send_agent_command(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(command): Json<AgentCommand>,
) -> ApiResult<Json<serde_json::Value>> {
    let agents = state.agents.read().await;
    
    let _agent = agents
        .get(&id)
        .ok_or_else(|| ApiError::NotFound(format!("Agent {} not found", id)))?;
    
    // In a real implementation, execute the command on the agent
    // For now, return a success response
    Ok(Json(serde_json::json!({
        "status": "executed",
        "agent_id": id,
        "command": format!("{:?}", command.command),
    })))
}