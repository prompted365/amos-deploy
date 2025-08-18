use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::{CognitiveAgent, BaseAgent, AgentState, AgentCapability};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub agent_count: usize,
    pub active_agents: usize,
    pub event_throughput: f64,
    pub harmony_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct AgentCoordination {
    pub agent_id: Uuid,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub activity_level: f64,
    pub capabilities: Vec<AgentCapability>,
}

pub struct MeshHarmonizer {
    base: BaseAgent,
    agent_registry: HashMap<Uuid, AgentCoordination>,
    system_metrics: Vec<SystemMetrics>,
    harmony_threshold: f64,
    coordination_cycles: u64,
    event_buffer: Vec<SystemEvent>,
    max_event_buffer: usize,
}

impl MeshHarmonizer {
    pub fn new() -> Self {
        Self {
            base: BaseAgent::new(
                "MeshHarmonizer".to_string(),
                vec![
                    AgentCapability::Coordination,
                    AgentCapability::Monitoring,
                ],
            ),
            agent_registry: HashMap::new(),
            system_metrics: Vec::new(),
            harmony_threshold: 0.7,
            coordination_cycles: 0,
            event_buffer: Vec::new(),
            max_event_buffer: 100,
        }
    }
    
    pub fn register_agent(&mut self, agent_id: Uuid, agent_type: String, capabilities: Vec<AgentCapability>) {
        let coordination = AgentCoordination {
            agent_id,
            last_seen: chrono::Utc::now(),
            activity_level: 1.0,
            capabilities,
        };
        
        self.agent_registry.insert(agent_id, coordination);
        self.base.logger.info(&format!("Registered agent: {} ({})", agent_type, agent_id));
    }
    
    pub fn update_agent_activity(&mut self, agent_id: Uuid) {
        if let Some(coord) = self.agent_registry.get_mut(&agent_id) {
            coord.last_seen = chrono::Utc::now();
            coord.activity_level = (coord.activity_level * 0.9 + 0.1).min(1.0);
        }
    }
    
    pub async fn harmonize_system(&mut self) -> Result<f64> {
        self.coordination_cycles += 1;
        
        // Calculate current harmony
        let harmony = self.calculate_harmony();
        
        // If harmony is low, coordinate agents
        if harmony < self.harmony_threshold {
            self.coordinate_agents().await?;
        }
        
        // Record metrics
        let metrics = SystemMetrics {
            agent_count: self.agent_registry.len(),
            active_agents: self.count_active_agents(),
            event_throughput: self.calculate_throughput(),
            harmony_score: harmony,
            timestamp: chrono::Utc::now(),
        };
        
        self.system_metrics.push(metrics);
        
        // Prune old metrics
        if self.system_metrics.len() > 1000 {
            self.system_metrics.drain(0..500);
        }
        
        Ok(harmony)
    }
    
    fn calculate_harmony(&self) -> f64 {
        if self.agent_registry.is_empty() {
            return 1.0;
        }
        
        let active_ratio = self.count_active_agents() as f64 / self.agent_registry.len() as f64;
        let activity_variance = self.calculate_activity_variance();
        let capability_coverage = self.calculate_capability_coverage();
        
        // Harmony is high when agents are active, balanced, and diverse
        (active_ratio * 0.4 + (1.0 - activity_variance) * 0.3 + capability_coverage * 0.3).min(1.0)
    }
    
    fn count_active_agents(&self) -> usize {
        let cutoff = chrono::Utc::now() - chrono::Duration::seconds(60);
        self.agent_registry
            .values()
            .filter(|coord| coord.last_seen > cutoff)
            .count()
    }
    
    fn calculate_activity_variance(&self) -> f64 {
        if self.agent_registry.is_empty() {
            return 0.0;
        }
        
        let activities: Vec<f64> = self.agent_registry.values().map(|c| c.activity_level).collect();
        let mean = activities.iter().sum::<f64>() / activities.len() as f64;
        let variance = activities.iter().map(|a| (a - mean).powi(2)).sum::<f64>() / activities.len() as f64;
        
        variance.sqrt()
    }
    
    fn calculate_capability_coverage(&self) -> f64 {
        let all_capabilities: HashSet<AgentCapability> = self.agent_registry
            .values()
            .flat_map(|coord| coord.capabilities.iter().cloned())
            .collect();
        
        // Assume we want at least 5 different capabilities
        (all_capabilities.len() as f64 / 5.0).min(1.0)
    }
    
    fn calculate_throughput(&self) -> f64 {
        // Events per second (simplified)
        self.event_buffer.len() as f64 / 60.0
    }
    
    async fn coordinate_agents(&mut self) -> Result<()> {
        // Find underutilized agents
        let underutilized: Vec<Uuid> = self.agent_registry
            .iter()
            .filter(|(_, coord)| coord.activity_level < 0.3)
            .map(|(id, _)| *id)
            .collect();
        
        if !underutilized.is_empty() && self.base.event_bus.is_some() {
            // Send activation events to underutilized agents
            for agent_id in underutilized {
                if let Some(event_bus) = &self.base.event_bus {
                    event_bus.publish(SystemEvent::AgentActivated {
                        agent_id,
                        agent_type: "Reactivation".to_string(),
                    }).await;
                }
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl CognitiveAgent for MeshHarmonizer {
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
        
        self.base.logger.info("MeshHarmonizer initialized");
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn activate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Active).await?;
        self.base.logger.info("MeshHarmonizer activated");
        Ok(())
    }
    
    async fn process(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Processing).await?;
        
        // Harmonize the system
        let harmony = self.harmonize_system().await?;
        
        self.base.logger.info(&format!("System harmony: {:.2} (cycle: {})", harmony, self.coordination_cycles));
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn suspend(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Suspended).await?;
        self.base.logger.info("MeshHarmonizer suspended");
        Ok(())
    }
    
    async fn terminate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Terminating).await?;
        
        // Clear registries
        self.agent_registry.clear();
        self.system_metrics.clear();
        self.event_buffer.clear();
        
        self.base.transition_state(AgentState::Terminated).await?;
        self.base.logger.info("MeshHarmonizer terminated");
        Ok(())
    }
    
    fn state(&self) -> AgentState {
        self.base.state.clone()
    }
    
    async fn receive_event(&mut self, event: SystemEvent) -> Result<()> {
        // Buffer events for throughput calculation
        if self.event_buffer.len() >= self.max_event_buffer {
            self.event_buffer.remove(0);
        }
        self.event_buffer.push(event.clone());
        
        match event {
            SystemEvent::AgentActivated { agent_id, agent_type } => {
                // Default capabilities based on agent type
                let capabilities = match agent_type.as_str() {
                    "TrafficSeer" => vec![AgentCapability::PatternRecognition, AgentCapability::Monitoring],
                    "PathwaySculptor" => vec![AgentCapability::NeuralOptimization, AgentCapability::Learning],
                    "MemoryWeaver" => vec![AgentCapability::MemoryManagement, AgentCapability::Learning],
                    _ => vec![],
                };
                
                self.register_agent(agent_id, agent_type, capabilities);
            }
            SystemEvent::NeuralFired { node_id: _ } |
            SystemEvent::PathwayStrengthened { .. } |
            SystemEvent::HormonalBurst { .. } => {
                // Track general system activity
                self.base.update_activity();
            }
            _ => {}
        }
        
        Ok(())
    }
}

impl Default for MeshHarmonizer {
    fn default() -> Self {
        Self::new()
    }
}