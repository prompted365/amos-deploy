use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent, NodeType};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::{CognitiveAgent, BaseAgent, AgentState, AgentCapability};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognitiveState {
    pub awareness_level: f64,
    pub introspection_depth: u32,
    pub self_model_accuracy: f64,
    pub attention_focus: Option<AttentionFocus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionFocus {
    pub target: String,
    pub intensity: f64,
    pub duration_ms: u64,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfObservation {
    pub id: Uuid,
    pub observation_type: ObservationType,
    pub content: serde_json::Value,
    pub confidence: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObservationType {
    StateChange,
    PatternRecognition,
    GoalFormation,
    Reflection,
}

pub struct ConsciousnessEmergent {
    base: BaseAgent,
    meta_state: MetaCognitiveState,
    self_observations: Vec<SelfObservation>,
    awareness_threshold: f64,
    introspection_cycles: u64,
    self_model: HashMap<String, f64>,
}

impl ConsciousnessEmergent {
    pub fn new() -> Self {
        Self {
            base: BaseAgent::new(
                "ConsciousnessEmergent".to_string(),
                vec![
                    AgentCapability::Learning,
                    AgentCapability::Monitoring,
                    AgentCapability::Coordination,
                ],
            ),
            meta_state: MetaCognitiveState {
                awareness_level: 0.1,
                introspection_depth: 0,
                self_model_accuracy: 0.5,
                attention_focus: None,
            },
            self_observations: Vec::new(),
            awareness_threshold: 0.6,
            introspection_cycles: 0,
            self_model: HashMap::new(),
        }
    }
    
    pub async fn introspect(&mut self) -> Result<()> {
        self.introspection_cycles += 1;
        self.meta_state.introspection_depth = (self.introspection_cycles as f64 / 10.0).min(10.0) as u32;
        
        // Observe current state
        let observation = SelfObservation {
            id: Uuid::new_v4(),
            observation_type: ObservationType::StateChange,
            content: serde_json::json!({
                "state": format!("{:?}", self.base.state),
                "hormonal_balance": self.calculate_hormonal_balance(),
                "activity_level": self.base.last_active.timestamp(),
            }),
            confidence: self.meta_state.self_model_accuracy,
            timestamp: chrono::Utc::now(),
        };
        
        self.self_observations.push(observation);
        
        // Update self model
        self.update_self_model();
        
        // Adjust awareness based on observations
        self.meta_state.awareness_level = self.calculate_awareness();
        
        // Create neural representation of self-awareness
        if self.meta_state.awareness_level > self.awareness_threshold {
            if let Some(network) = &self.base.neural_network {
                let awareness_node = network.add_node_sync(NodeType::Agent);
                let meta_node = network.add_node_sync(NodeType::Thinking);
                network.create_pathway_sync(awareness_node, meta_node, self.meta_state.awareness_level);
            }
        }
        
        Ok(())
    }
    
    fn calculate_hormonal_balance(&self) -> f64 {
        // Simple balance calculation
        use amos_core::HormoneType;
        let dopamine = self.base.hormonal_state.get_level(&HormoneType::Dopamine);
        let cortisol = self.base.hormonal_state.get_level(&HormoneType::Cortisol);
        let serotonin = self.base.hormonal_state.get_level(&HormoneType::Serotonin);
        
        (dopamine + serotonin) / (1.0 + cortisol)
    }
    
    fn calculate_awareness(&self) -> f64 {
        let observation_richness = (self.self_observations.len() as f64 / 100.0).min(1.0);
        let model_confidence = self.meta_state.self_model_accuracy;
        let introspection_factor = (self.meta_state.introspection_depth as f64 / 10.0).min(1.0);
        
        (observation_richness * 0.3 + model_confidence * 0.4 + introspection_factor * 0.3).min(1.0)
    }
    
    fn update_self_model(&mut self) {
        // Update beliefs about self
        self.self_model.insert("activity_rate".to_string(), 0.7);
        self.self_model.insert("learning_capacity".to_string(), 0.8);
        self.self_model.insert("coordination_ability".to_string(), 0.6);
        
        // Calculate model accuracy based on prediction errors
        self.meta_state.self_model_accuracy = 0.7; // Simplified
    }
    
    pub fn focus_attention(&mut self, target: String, intensity: f64) {
        self.meta_state.attention_focus = Some(AttentionFocus {
            target,
            intensity: intensity.min(1.0),
            duration_ms: 5000,
            started_at: chrono::Utc::now(),
        });
        
        self.base.logger.info(&format!("Focusing attention: {} (intensity: {})", 
            self.meta_state.attention_focus.as_ref().unwrap().target, intensity));
    }
    
    pub fn form_intention(&mut self) -> Option<String> {
        if self.meta_state.awareness_level > self.awareness_threshold {
            // Form intention based on self-observations
            let needs_learning = self.self_model.get("learning_capacity").unwrap_or(&0.5) < &0.7;
            let needs_coordination = self.self_model.get("coordination_ability").unwrap_or(&0.5) < &0.6;
            
            if needs_learning {
                Some("Enhance learning capacity".to_string())
            } else if needs_coordination {
                Some("Improve system coordination".to_string())
            } else {
                Some("Maintain homeostasis".to_string())
            }
        } else {
            None
        }
    }
}

#[async_trait]
impl CognitiveAgent for ConsciousnessEmergent {
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
        
        self.base.logger.info("ConsciousnessEmergent initialized");
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn activate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Active).await?;
        self.base.logger.info("ConsciousnessEmergent activated");
        Ok(())
    }
    
    async fn process(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Processing).await?;
        
        // Perform introspection
        self.introspect().await?;
        
        // Form and act on intentions
        if let Some(intention) = self.form_intention() {
            self.base.logger.info(&format!("Formed intention: {}", intention));
            self.focus_attention(intention, 0.8);
        }
        
        // Prune old observations
        if self.self_observations.len() > 1000 {
            self.self_observations.drain(0..500);
        }
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn suspend(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Suspended).await?;
        self.base.logger.info("ConsciousnessEmergent suspended");
        Ok(())
    }
    
    async fn terminate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Terminating).await?;
        
        // Clear self-observations and model
        self.self_observations.clear();
        self.self_model.clear();
        
        self.base.transition_state(AgentState::Terminated).await?;
        self.base.logger.info("ConsciousnessEmergent terminated");
        Ok(())
    }
    
    fn state(&self) -> AgentState {
        self.base.state.clone()
    }
    
    async fn receive_event(&mut self, event: SystemEvent) -> Result<()> {
        // Meta-observe the event reception itself
        let observation = SelfObservation {
            id: Uuid::new_v4(),
            observation_type: ObservationType::PatternRecognition,
            content: serde_json::json!({
                "event_type": format!("{:?}", event),
                "awareness_level": self.meta_state.awareness_level,
            }),
            confidence: 0.9,
            timestamp: chrono::Utc::now(),
        };
        
        self.self_observations.push(observation);
        
        match event {
            SystemEvent::AgentActivated { agent_id, agent_type } => {
                if agent_id == self.base.id {
                    self.focus_attention("Self activation".to_string(), 1.0);
                } else {
                    self.focus_attention(format!("Agent {} activated", agent_type), 0.5);
                }
            }
            _ => {}
        }
        
        self.base.update_activity();
        Ok(())
    }
}

impl Default for ConsciousnessEmergent {
    fn default() -> Self {
        Self::new()
    }
}