use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent, HormoneType, HormonalBurst};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::{CognitiveAgent, BaseAgent, AgentState, AgentCapability};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStrategy {
    pub id: Uuid,
    pub name: String,
    pub effectiveness: f64,
    pub context: LearningContext,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningContext {
    Reinforcement,    // Learn from rewards/punishments
    Supervised,       // Learn from examples
    Unsupervised,     // Discover patterns
    MetaLearning,     // Learn how to learn
}

#[derive(Debug, Clone)]
pub struct LearningMetrics {
    pub success_rate: f64,
    pub learning_speed: f64,
    pub retention_rate: f64,
    pub generalization_score: f64,
}

pub struct LearningOracle {
    base: BaseAgent,
    strategies: HashMap<Uuid, LearningStrategy>,
    active_strategy: Option<Uuid>,
    learning_history: Vec<(chrono::DateTime<chrono::Utc>, LearningMetrics)>,
    dopamine_threshold: f64,
    cortisol_threshold: f64,
}

impl LearningOracle {
    pub fn new() -> Self {
        let mut oracle = Self {
            base: BaseAgent::new(
                "LearningOracle".to_string(),
                vec![
                    AgentCapability::Learning,
                    AgentCapability::NeuralOptimization,
                ],
            ),
            strategies: HashMap::new(),
            active_strategy: None,
            learning_history: Vec::new(),
            dopamine_threshold: 0.7,
            cortisol_threshold: 0.8,
        };
        
        // Initialize default strategies
        oracle.init_default_strategies();
        oracle
    }
    
    fn init_default_strategies(&mut self) {
        let reinforcement = LearningStrategy {
            id: Uuid::new_v4(),
            name: "Reinforcement Learning".to_string(),
            effectiveness: 0.5,
            context: LearningContext::Reinforcement,
            parameters: HashMap::from([
                ("learning_rate".to_string(), 0.1),
                ("discount_factor".to_string(), 0.9),
                ("exploration_rate".to_string(), 0.2),
            ]),
        };
        
        let meta_learning = LearningStrategy {
            id: Uuid::new_v4(),
            name: "Meta Learning".to_string(),
            effectiveness: 0.5,
            context: LearningContext::MetaLearning,
            parameters: HashMap::from([
                ("adaptation_rate".to_string(), 0.05),
                ("meta_batch_size".to_string(), 10.0),
            ]),
        };
        
        self.strategies.insert(reinforcement.id, reinforcement.clone());
        self.strategies.insert(meta_learning.id, meta_learning);
        self.active_strategy = Some(reinforcement.id);
    }
    
    pub fn select_strategy(&mut self, context: LearningContext) -> Option<Uuid> {
        let best_strategy = self.strategies
            .iter()
            .filter(|(_, s)| s.context == context)
            .max_by(|(_, a), (_, b)| a.effectiveness.partial_cmp(&b.effectiveness).unwrap());
        
        if let Some((id, strategy)) = best_strategy {
            self.active_strategy = Some(*id);
            self.base.logger.info(&format!("Selected strategy: {}", strategy.name));
            Some(*id)
        } else {
            None
        }
    }
    
    pub async fn adapt_learning(&mut self) -> Result<()> {
        if let Some(strategy_id) = self.active_strategy {
            // Calculate current metrics
            let metrics = self.calculate_metrics();
            
            // Get strategy context for potential switching
            let strategy_context = self.strategies.get(&strategy_id).map(|s| s.context.clone());
            
            // Update strategy effectiveness
            if let Some(strategy) = self.strategies.get_mut(&strategy_id) {
                // Adapt strategy based on performance
                if metrics.success_rate > 0.8 {
                    strategy.effectiveness = (strategy.effectiveness * 1.1).min(1.0);
                } else if metrics.success_rate < 0.3 {
                    strategy.effectiveness = (strategy.effectiveness * 0.9).max(0.1);
                }
            }
            
            // Trigger dopamine burst for successful learning
            if metrics.success_rate > 0.8 {
                if let Some(event_bus) = &self.base.event_bus {
                    event_bus.publish(SystemEvent::HormonalBurst {
                        hormone_type: "Dopamine".to_string(),
                        intensity: 0.6,
                    }).await;
                }
            } else if metrics.success_rate < 0.3 {
                // Consider switching strategies
                if let Some(context) = strategy_context {
                    self.select_strategy(context);
                }
            }
            
            // Store metrics
            self.learning_history.push((chrono::Utc::now(), metrics));
        }
        
        Ok(())
    }
    
    fn calculate_metrics(&self) -> LearningMetrics {
        // Simplified metrics calculation
        LearningMetrics {
            success_rate: self.base.hormonal_state.get_level(&HormoneType::Dopamine),
            learning_speed: 0.5,
            retention_rate: 0.7,
            generalization_score: 0.6,
        }
    }
    
    pub fn adjust_parameters(&mut self, hormone_type: &str, intensity: f64) {
        if let Some(strategy_id) = self.active_strategy {
            if let Some(strategy) = self.strategies.get_mut(&strategy_id) {
                match hormone_type {
                    "Dopamine" if intensity > self.dopamine_threshold => {
                        // Increase learning rate for reward
                        if let Some(lr) = strategy.parameters.get_mut("learning_rate") {
                            *lr = (*lr * 1.1).min(1.0);
                        }
                    }
                    "Cortisol" if intensity > self.cortisol_threshold => {
                        // Increase exploration for stress
                        if let Some(er) = strategy.parameters.get_mut("exploration_rate") {
                            *er = (*er * 1.2).min(0.9);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[async_trait]
impl CognitiveAgent for LearningOracle {
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
        
        self.base.logger.info("LearningOracle initialized");
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn activate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Active).await?;
        self.base.logger.info("LearningOracle activated");
        Ok(())
    }
    
    async fn process(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Processing).await?;
        
        // Adapt current learning strategy
        self.adapt_learning().await?;
        
        // Prune old history
        if self.learning_history.len() > 100 {
            self.learning_history.drain(0..50);
        }
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn suspend(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Suspended).await?;
        self.base.logger.info("LearningOracle suspended");
        Ok(())
    }
    
    async fn terminate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Terminating).await?;
        
        // Clear strategies and history
        self.strategies.clear();
        self.learning_history.clear();
        
        self.base.transition_state(AgentState::Terminated).await?;
        self.base.logger.info("LearningOracle terminated");
        Ok(())
    }
    
    fn state(&self) -> AgentState {
        self.base.state.clone()
    }
    
    async fn receive_event(&mut self, event: SystemEvent) -> Result<()> {
        match event {
            SystemEvent::HormonalBurst { hormone_type, intensity } => {
                // Adjust learning parameters based on hormonal state
                self.adjust_parameters(&hormone_type, intensity);
                
                // Apply burst to internal state
                let burst = HormonalBurst {
                    id: Uuid::new_v4(),
                    hormone: match hormone_type.as_str() {
                        "Dopamine" => HormoneType::Dopamine,
                        "Cortisol" => HormoneType::Cortisol,
                        "Serotonin" => HormoneType::Serotonin,
                        _ => HormoneType::Dopamine,
                    },
                    intensity,
                    triggered_at: chrono::Utc::now(),
                    duration_ms: 5000,
                };
                self.base.hormonal_state.apply_burst(&burst);
            }
            SystemEvent::PathwayStrengthened { pathway_id: _, new_strength } => {
                // Track learning success
                if new_strength > 0.8 {
                    self.base.logger.debug("Strong pathway formed - learning successful");
                }
            }
            _ => {}
        }
        
        self.base.update_activity();
        Ok(())
    }
}

impl Default for LearningOracle {
    fn default() -> Self {
        Self::new()
    }
}