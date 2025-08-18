use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent, HormonalState, Logger};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentState {
    Uninitialized,
    Initializing,
    Active,
    Processing,
    Suspended,
    Terminating,
    Terminated,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentCapability {
    PatternRecognition,
    NeuralOptimization,
    MemoryManagement,
    Learning,
    Coordination,
    Monitoring,
    Generation,
}

#[async_trait]
pub trait CognitiveAgent: Send + Sync {
    fn id(&self) -> Uuid;
    fn name(&self) -> &str;
    fn capabilities(&self) -> Vec<AgentCapability>;
    
    async fn initialize(&mut self, neural_network: Arc<ForgeNeuralNetwork>, event_bus: Arc<EventBus>) -> Result<()>;
    async fn activate(&mut self) -> Result<()>;
    async fn process(&mut self) -> Result<()>;
    async fn suspend(&mut self) -> Result<()>;
    async fn terminate(&mut self) -> Result<()>;
    
    fn state(&self) -> AgentState;
    async fn receive_event(&mut self, event: SystemEvent) -> Result<()>;
}

pub struct BaseAgent {
    pub id: Uuid,
    pub name: String,
    pub state: AgentState,
    pub capabilities: Vec<AgentCapability>,
    pub neural_network: Option<Arc<ForgeNeuralNetwork>>,
    pub event_bus: Option<Arc<EventBus>>,
    pub hormonal_state: HormonalState,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub logger: Logger,
}

impl BaseAgent {
    pub fn new(name: String, capabilities: Vec<AgentCapability>) -> Self {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        Self {
            id,
            name: name.clone(),
            state: AgentState::Uninitialized,
            capabilities,
            neural_network: None,
            event_bus: None,
            hormonal_state: HormonalState::new(),
            created_at: now,
            last_active: now,
            logger: Logger::new(&format!("agent.{}", name)),
        }
    }
    
    pub async fn transition_state(&mut self, new_state: AgentState) -> Result<()> {
        let old_state = self.state.clone();
        self.state = new_state.clone();
        self.last_active = Utc::now();
        
        self.logger.info(&format!("State transition: {:?} -> {:?}", old_state, new_state));
        
        if let Some(event_bus) = &self.event_bus {
            event_bus.publish(SystemEvent::AgentActivated {
                agent_id: self.id,
                agent_type: self.name.clone(),
            }).await;
        }
        
        Ok(())
    }
    
    pub fn update_activity(&mut self) {
        self.last_active = Utc::now();
    }
}

pub struct AgentContext {
    pub neural_network: Arc<ForgeNeuralNetwork>,
    pub event_bus: Arc<EventBus>,
    pub shared_hormonal_state: Arc<tokio::sync::RwLock<HormonalState>>,
}

impl AgentContext {
    pub fn new(
        neural_network: Arc<ForgeNeuralNetwork>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            neural_network,
            event_bus,
            shared_hormonal_state: Arc::new(tokio::sync::RwLock::new(HormonalState::new())),
        }
    }
}