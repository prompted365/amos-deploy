use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::collections::HashMap;
use amos_core::{neural::ForgeNeuralNetwork, EventBus};
use amos_agents::CognitiveAgent;
use crate::auth::TokenValidator;
use crate::websocket::WsState;

#[derive(Clone)]
pub struct AppState {
    pub neural_network: Arc<ForgeNeuralNetwork>,
    pub event_bus: Arc<EventBus>,
    pub agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>,
    pub swarms: Arc<RwLock<HashMap<Uuid, SwarmState>>>,
    pub token_validator: Arc<TokenValidator>,
    pub ws_state: Arc<WsState>,
}

#[derive(Clone)]
pub struct SwarmState {
    pub id: Uuid,
    pub name: String,
    pub agent_ids: Vec<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AppState {
    pub fn new(secret_key: String) -> Self {
        Self {
            neural_network: Arc::new(ForgeNeuralNetwork::new()),
            event_bus: Arc::new(EventBus::new()),
            agents: Arc::new(RwLock::new(HashMap::new())),
            swarms: Arc::new(RwLock::new(HashMap::new())),
            token_validator: Arc::new(TokenValidator::new(secret_key)),
            ws_state: Arc::new(WsState::new()),
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Self::new("test-secret-key".to_string())
    }
}