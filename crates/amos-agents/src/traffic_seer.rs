use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::VecDeque;
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent, Pattern, PatternType, NodeType};
use anyhow::Result;
use crate::{CognitiveAgent, BaseAgent, AgentState, AgentCapability};

pub struct TrafficSeer {
    base: BaseAgent,
    pattern_buffer: VecDeque<Pattern>,
    pattern_threshold: f64,
    max_patterns: usize,
}

impl TrafficSeer {
    pub fn new() -> Self {
        Self {
            base: BaseAgent::new(
                "TrafficSeer".to_string(),
                vec![
                    AgentCapability::PatternRecognition,
                    AgentCapability::Monitoring,
                ],
            ),
            pattern_buffer: VecDeque::with_capacity(100),
            pattern_threshold: 0.7,
            max_patterns: 100,
        }
    }
    
    pub async fn analyze_traffic_patterns(&mut self) -> Result<Vec<Pattern>> {
        let mut significant_patterns = Vec::new();
        
        // Analyze buffered patterns
        for pattern in &self.pattern_buffer {
            if self.is_significant(pattern) {
                significant_patterns.push(pattern.clone());
            }
        }
        
        // Create neural pathways for significant patterns
        if let Some(network) = &self.base.neural_network {
            for pattern in &significant_patterns {
                let source = network.add_node_sync(NodeType::Memory);
                let target = network.add_node_sync(NodeType::Thinking);
                
                // Strength based on pattern significance
                let strength = self.calculate_pattern_strength(pattern);
                network.create_pathway_sync(source, target, strength);
            }
        }
        
        Ok(significant_patterns)
    }
    
    fn is_significant(&self, pattern: &Pattern) -> bool {
        // Simple heuristic: patterns with high variance are significant
        if pattern.data.is_empty() {
            return false;
        }
        
        let mean: f64 = pattern.data.iter().sum::<f64>() / pattern.data.len() as f64;
        let variance: f64 = pattern.data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / pattern.data.len() as f64;
        
        variance > self.pattern_threshold
    }
    
    fn calculate_pattern_strength(&self, pattern: &Pattern) -> f64 {
        match pattern.pattern_type {
            PatternType::Attack => 0.9,
            PatternType::Anomaly => 0.7,
            PatternType::Overload => 0.6,
            PatternType::Normal => 0.3,
        }
    }
    
    pub fn add_pattern(&mut self, pattern: Pattern) {
        if self.pattern_buffer.len() >= self.max_patterns {
            self.pattern_buffer.pop_front();
        }
        self.pattern_buffer.push_back(pattern);
    }
}

#[async_trait]
impl CognitiveAgent for TrafficSeer {
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
        
        self.base.logger.info("TrafficSeer initialized");
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn activate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Active).await?;
        self.base.logger.info("TrafficSeer activated");
        Ok(())
    }
    
    async fn process(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Processing).await?;
        
        // Analyze current patterns
        let significant_patterns = self.analyze_traffic_patterns().await?;
        
        // Publish events for significant patterns
        if let Some(event_bus) = &self.base.event_bus {
            for pattern in significant_patterns {
                if pattern.pattern_type != PatternType::Normal {
                    event_bus.publish(SystemEvent::ThreatDetected {
                        threat_id: pattern.id,
                        level: format!("{:?}", pattern.pattern_type),
                    }).await;
                }
            }
        }
        
        self.base.transition_state(AgentState::Active).await?;
        Ok(())
    }
    
    async fn suspend(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Suspended).await?;
        self.base.logger.info("TrafficSeer suspended");
        Ok(())
    }
    
    async fn terminate(&mut self) -> Result<()> {
        self.base.transition_state(AgentState::Terminating).await?;
        
        // Clear pattern buffer
        self.pattern_buffer.clear();
        
        self.base.transition_state(AgentState::Terminated).await?;
        self.base.logger.info("TrafficSeer terminated");
        Ok(())
    }
    
    fn state(&self) -> AgentState {
        self.base.state.clone()
    }
    
    async fn receive_event(&mut self, event: SystemEvent) -> Result<()> {
        match event {
            SystemEvent::NeuralFired { node_id: _ } => {
                // Create pattern from neural activity
                let pattern = Pattern {
                    id: Uuid::new_v4(),
                    data: vec![1.0], // Simplified for now
                    pattern_type: PatternType::Normal,
                };
                self.add_pattern(pattern);
            }
            _ => {}
        }
        
        self.base.update_activity();
        Ok(())
    }
}

impl Default for TrafficSeer {
    fn default() -> Self {
        Self::new()
    }
}