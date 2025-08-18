use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SwarmInfo {
    pub id: Uuid,
    pub name: String,
    pub agent_count: usize,
    pub status: SwarmStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub active_tasks: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SwarmStatus {
    Idle,
    Active,
    Processing,
    Paused,
    Error,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSwarmRequest {
    pub name: String,
    pub agent_ids: Vec<Uuid>,
    pub topology: SwarmTopology,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SwarmTopology {
    Mesh,
    Hierarchical,
    Ring,
    Star,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrchestrateTaskRequest {
    pub task_description: String,
    pub strategy: ExecutionStrategy,
    pub timeout_seconds: Option<u64>,
    pub priority: TaskPriority,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStrategy {
    Parallel,
    Sequential,
    Adaptive,
    Distributed,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub status: TaskStatus,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}