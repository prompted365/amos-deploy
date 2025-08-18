use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent, NodeType};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::{CognitiveAgent, BaseAgent, AgentState, AgentCapability};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub id: Uuid,
    pub content: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub importance: f64,
    pub access_count: u32,
    pub last_accessed: DateTime<Utc>,
    pub associated_nodes: Vec<Uuid>,
}

impl EpisodicMemory {
    pub fn new(content: serde_json::Value, importance: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content,
            timestamp: now,
            importance,
            access_count: 0,
            last_accessed: now,
            associated_nodes: Vec::new(),
        }
    }
    
    pub fn access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Utc::now();
    }
    
    pub fn decay(&mut self, decay_rate: f64) {
        self.importance = (self.importance - decay_rate).max(0.0);
    }
}

pub struct MemoryWeaver {
    base: BaseAgent,
    episodic_store: HashMap<Uuid, EpisodicMemory>,
    working_memory: VecDeque<Uuid>,
    consolidation_threshold: f64,
    max_working_memory: usize,
    memory_decay_rate: f64,
}

impl MemoryWeaver {
    pub fn new() -> Self {
        Self {
            base: BaseAgent::new(
                "MemoryWeaver".to_string(),
                vec![
                    AgentCapability::MemoryManagement,
                    AgentCapability::Learning,
                ],
            ),
            episodic_store: HashMap::new(),
            working_memory: VecDeque::with_capacity(10),
            consolidation_threshold: 0.7,
            max_working_memory: 10,
            memory_decay_rate: 0.01,
        }
    }
    
    pub fn store_memory(&mut self, content: serde_json::Value, importance: f64) -> Uuid {
        let memory = EpisodicMemory::new(content, importance);
        let memory_id = memory.id;
        
        // Add to episodic store
        self.episodic_store.insert(memory_id, memory);
        
        // Add to working memory
        if self.working_memory.len() >= self.max_working_memory {
            self.working_memory.pop_front();
        }
        self.working_memory.push_back(memory_id);
        
        self.base.logger.info(&format!("Stored memory: {} (importance: {})", memory_id, importance));
        
        memory_id
    }
    
    pub fn retrieve_memory(&mut self, memory_id: Uuid) -> Option<&EpisodicMemory> {
        if let Some(memory) = self.episodic_store.get_mut(&memory_id) {
            memory.access();
            Some(memory)
        } else {
            None
        }
    }
    
    pub async fn consolidate_memories(&mut self) -> Result<Vec<Uuid>> {
        let mut consolidated = Vec::new();
        
        // Find memories that should be consolidated
        for memory in self.episodic_store.values_mut() {
            if memory.importance >= self.consolidation_threshold {
                // Create neural pathways for important memories
                if let Some(network) = &self.base.neural_network {
                    let memory_node = network.add_node_sync(NodeType::Memory);
                    memory.associated_nodes.push(memory_node);
                    
                    // Connect to other memory nodes
                    for &other_node in &memory.associated_nodes[..memory.associated_nodes.len()-1] {
                        network.create_pathway_sync(memory_node, other_node, memory.importance);
                    }
                }
                
                consolidated.push(memory.id);
            }
        }
        
        Ok(consolidated)
    }
    
    pub fn apply_decay(&mut self) {
        let decay_rate = self.memory_decay_rate;
        for memory in self.episodic_store.values_mut() {
            memory.decay(decay_rate);
        }
        
        // Remove memories with zero importance
        self.episodic_store.retain(|_, memory| memory.importance > 0.0);
    }
    
    pub fn search_memories(&self, predicate: impl Fn(&EpisodicMemory) -> bool) -> Vec<Uuid> {
        self.episodic_store
            .iter()
            .filter(|(_, memory)| predicate(memory))
            .map(|(id, _)| *id)
            .collect()
    }
}

#[async_trait]
impl CognitiveAgent for MemoryWeaver {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn capabilities(&self) -> Vec<AgentCapability> {
        self.base.capabilities.clone()
    }
    
    async fn initialize(&mut self, neural_network: Arc<ForgeNeuralNetwork>, event_bus: Arc<EventBus>) -> Result<()> {
        self.base.transition_state(AgentState::Initializing).await?;
        
        self.base.neural_network = Some(neural_network);
        self.base.event_bus = Some(event_bus.clone());
        
        self.base.logger.info("MemoryWeaver initialized");
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn activate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Active).await?;
        self.base.logger.info("MemoryWeaver activated");
        Ok(())
    }
    
    async fn process(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Processing).await?;
        
        // Apply memory decay
        self.apply_decay();
        
        // Consolidate important memories
        let consolidated = self.consolidate_memories().await?;
        
        if !consolidated.is_empty() {
            self.base.logger.info(&format!("Consolidated {} memories", consolidated.len()));
        }
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn suspend(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Suspended).await?;
        self.base.logger.info("MemoryWeaver suspended");
        Ok(())
    }
    
    async fn terminate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Terminating).await?;
        
        // Clear memory stores
        self.episodic_store.clear();
        self.working_memory.clear();
        
        self.base.transition_state(AgentState::Terminated).await?;
        self.base.logger.info("MemoryWeaver terminated");
        Ok(())
    }
    
    fn state(&self) -> AgentState {
        self.base.state.clone()
    }
    
    async fn receive_event(&mut self, event: SystemEvent) -> Result<()> {
        match event {
            SystemEvent::MemoryStored { memory_id, content_size } => {
                // Track memory storage events
                self.base.logger.debug(&format!("Memory stored: {} (size: {})", memory_id, content_size));
            }
            SystemEvent::NeuralFired { node_id: _ } => {
                // Could trigger memory consolidation based on neural activity
                self.base.update_activity();
            }
            _ => {}
        }
        
        Ok(())
    }
}

impl Default for MemoryWeaver {
    fn default() -> Self {
        Self::new()
    }
}