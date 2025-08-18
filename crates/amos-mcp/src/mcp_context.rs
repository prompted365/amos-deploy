use crate::mcp_protocol::{ContextItem};
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use amos_core::neural::ForgeNeuralNetwork;
use amos_agents::CognitiveAgent;

/// Context provider for MCP
pub struct ContextProvider {
    contexts: Arc<RwLock<HashMap<String, ContextItem>>>,
    neural_network: Arc<ForgeNeuralNetwork>,
    agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>,
}

impl ContextProvider {
    pub fn new(
        neural_network: Arc<ForgeNeuralNetwork>,
        agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>
    ) -> Self {
        let contexts = Arc::new(RwLock::new(HashMap::new()));
        
        // Initialize with default contexts
        let provider = Self {
            contexts: contexts.clone(),
            neural_network,
            agents,
        };
        
        // Setup default contexts
        tokio::spawn(async move {
            let mut ctx = contexts.write().await;
            
            // Neural network context
            ctx.insert("neural_network".to_string(), ContextItem {
                id: "neural_network".to_string(),
                name: "Neural Network State".to_string(),
                description: "Current state of the AMOS neural network".to_string(),
                content_type: "application/json".to_string(),
            });
            
            // Agent swarm context
            ctx.insert("agent_swarm".to_string(), ContextItem {
                id: "agent_swarm".to_string(),
                name: "Agent Swarm Status".to_string(),
                description: "Status and configuration of all cognitive agents".to_string(),
                content_type: "application/json".to_string(),
            });
            
            // System metrics context
            ctx.insert("system_metrics".to_string(), ContextItem {
                id: "system_metrics".to_string(),
                name: "System Metrics".to_string(),
                description: "Real-time system performance metrics".to_string(),
                content_type: "application/json".to_string(),
            });
            
            // Event history context
            ctx.insert("event_history".to_string(), ContextItem {
                id: "event_history".to_string(),
                name: "Event History".to_string(),
                description: "Recent system events and agent activities".to_string(),
                content_type: "application/json".to_string(),
            });
        });
        
        provider
    }
    
    /// List all available contexts
    pub async fn list_contexts(&self) -> Vec<ContextItem> {
        let contexts = self.contexts.read().await;
        contexts.values().cloned().collect()
    }
    
    /// Get a specific context
    pub async fn get_context(&self, context_id: &str) -> Result<Value> {
        let contexts = self.contexts.read().await;
        
        if !contexts.contains_key(context_id) {
            return Err(anyhow!("Context '{}' not found", context_id));
        }
        
        // Generate context content based on ID
        match context_id {
            "neural_network" => self.get_neural_network_context().await,
            "agent_swarm" => self.get_agent_swarm_context().await,
            "system_metrics" => self.get_system_metrics_context().await,
            "event_history" => self.get_event_history_context().await,
            _ => Err(anyhow!("Unknown context: {}", context_id)),
        }
    }
    
    /// Get neural network context
    async fn get_neural_network_context(&self) -> Result<Value> {
        // Get basic stats from the neural network
        let node_count = self.neural_network.node_count().await;
        let pathway_count = self.neural_network.pathway_count().await;
        
        Ok(serde_json::json!({
            "pathways": {
                "total": pathway_count,
                "active": pathway_count, // Would track active separately in production
                "pruned": 0, // Would track pruned pathways in production
            },
            "nodes": {
                "total": node_count,
                "active": node_count, // Would track active separately in production
            },
            "performance": {
                "average_strength": 0.5, // Would calculate in production
                "pruning_rate": 0.1, // Would track in production
            },
            "configuration": {
                "plasticity_rate": 0.1,
                "pruning_threshold": 0.2,
                "learning_enabled": true,
            }
        }))
    }
    
    /// Get agent swarm context
    async fn get_agent_swarm_context(&self) -> Result<Value> {
        let agents = self.agents.read().await;
        
        let agent_list: Vec<Value> = agents.iter().map(|(id, agent)| {
            serde_json::json!({
                "id": id.to_string(),
                "name": agent.name(),
                "state": format!("{:?}", agent.state()),
                "capabilities": agent.capabilities().iter()
                    .map(|c| format!("{:?}", c))
                    .collect::<Vec<_>>(),
            })
        }).collect();
        
        Ok(serde_json::json!({
            "total_agents": agents.len(),
            "agents": agent_list,
            "swarm_state": "active", // Would be tracked in production
        }))
    }
    
    /// Get system metrics context
    async fn get_system_metrics_context(&self) -> Result<Value> {
        // In production, these would be real metrics
        Ok(serde_json::json!({
            "cpu_usage": 45.2,
            "memory_usage_mb": 512,
            "events_per_second": 120,
            "average_response_time_ms": 15,
            "uptime_seconds": 3600,
        }))
    }
    
    /// Get event history context
    async fn get_event_history_context(&self) -> Result<Value> {
        // In production, this would return actual event history
        Ok(serde_json::json!({
            "recent_events": [
                {
                    "timestamp": "2024-01-01T12:00:00Z",
                    "type": "agent_initialized",
                    "agent": "TrafficSeer",
                    "details": "Agent successfully initialized"
                },
                {
                    "timestamp": "2024-01-01T12:00:01Z",
                    "type": "pathway_created",
                    "source": "node_123",
                    "target": "node_456",
                    "strength": 0.75
                }
            ],
            "event_count": 2,
        }))
    }
    
    /// Add a custom context
    pub async fn add_context(&self, context: ContextItem) -> Result<()> {
        let mut contexts = self.contexts.write().await;
        
        if contexts.contains_key(&context.id) {
            return Err(anyhow!("Context '{}' already exists", context.id));
        }
        
        contexts.insert(context.id.clone(), context);
        Ok(())
    }
    
    /// Remove a context
    pub async fn remove_context(&self, context_id: &str) -> Result<()> {
        let mut contexts = self.contexts.write().await;
        
        if !contexts.contains_key(context_id) {
            return Err(anyhow!("Context '{}' not found", context_id));
        }
        
        // Don't allow removing default contexts
        let default_contexts = ["neural_network", "agent_swarm", "system_metrics", "event_history"];
        if default_contexts.contains(&context_id) {
            return Err(anyhow!("Cannot remove default context '{}'", context_id));
        }
        
        contexts.remove(context_id);
        Ok(())
    }
}

/// Context builder for creating custom contexts
pub struct ContextBuilder {
    id: String,
    name: String,
    description: String,
    content_type: String,
}

impl ContextBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            description: String::new(),
            content_type: "application/json".to_string(),
        }
    }
    
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
    
    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.content_type = content_type;
        self
    }
    
    pub fn build(self) -> ContextItem {
        ContextItem {
            id: self.id,
            name: self.name,
            description: self.description,
            content_type: self.content_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_context_builder() {
        let context = ContextBuilder::new("test".to_string())
            .with_name("Test Context".to_string())
            .with_description("A test context".to_string())
            .build();
        
        assert_eq!(context.id, "test");
        assert_eq!(context.name, "Test Context");
        assert_eq!(context.content_type, "application/json");
    }
    
    #[tokio::test]
    async fn test_context_provider() {
        let neural_network = Arc::new(ForgeNeuralNetwork::new());
        let agents = Arc::new(RwLock::new(HashMap::new()));
        
        let provider = ContextProvider::new(neural_network, agents);
        
        // Give time for default contexts to be initialized
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let contexts = provider.list_contexts().await;
        assert!(contexts.len() >= 4); // Should have at least 4 default contexts
    }
}