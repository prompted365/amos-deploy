use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NeuralState {
    pub total_nodes: usize,
    pub total_pathways: usize,
    pub active_pathways: usize,
    pub average_strength: f64,
    pub hormonal_levels: HormonalLevels,
    pub immune_status: ImmuneStatus,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HormonalLevels {
    pub dopamine: f64,
    pub serotonin: f64,
    pub cortisol: f64,
    pub oxytocin: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ImmuneStatus {
    pub health: f64,
    pub threats_detected: usize,
    pub patterns_remembered: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PathwayUpdate {
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub strength_delta: f64,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PathwayInfo {
    pub id: Uuid,
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub strength: f64,
    pub activation_count: u64,
    pub last_activated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HormonalUpdate {
    pub hormone: String,
    pub delta: f64,
    pub reason: String,
}