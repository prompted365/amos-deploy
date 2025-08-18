use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub active_agents: usize,
    pub active_swarms: usize,
    pub neural_pathways: usize,
    pub neural_nodes: usize,
    pub events_processed: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AgentMetrics {
    pub agent_id: Uuid,
    pub agent_name: String,
    pub state: String,
    pub tasks_completed: u64,
    pub average_response_time: f64,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub last_active: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SwarmMetrics {
    pub swarm_id: Uuid,
    pub swarm_name: String,
    pub total_agents: usize,
    pub active_agents: usize,
    pub tasks_orchestrated: u64,
    pub average_task_time: f64,
    pub created_at: DateTime<Utc>,
}