use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AgentInfo {
    pub id: Uuid,
    pub name: String,
    pub agent_type: String,
    pub state: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub neural_network_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAgentRequest {
    pub name: String,
    pub agent_type: AgentType,
    pub shadow_mode: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentType {
    TrafficSeer,
    PathwaySculptor,
    MemoryWeaver,
    CognitionAlchemist,
    LearningOracle,
    MeshHarmonizer,
    ConsciousnessEmergent,
    PerformanceGuardian,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AgentCommand {
    pub command: CommandType,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CommandType {
    Start,
    Stop,
    Pause,
    Resume,
    Reset,
    Process,
}