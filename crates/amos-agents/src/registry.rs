use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent, Logger};
use crate::{CognitiveAgent, AgentState, AgentContext};

pub struct AgentRegistry {
    agents: Arc<RwLock<HashMap<Uuid, Box<dyn CognitiveAgent>>>>,
    context: AgentContext,
    logger: Logger,
}

impl AgentRegistry {
    pub fn new(neural_network: Arc<ForgeNeuralNetwork>, event_bus: Arc<EventBus>) -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            context: AgentContext::new(neural_network, event_bus),
            logger: Logger::new("agent_registry"),
        }
    }
    
    pub async fn spawn_agent(&self, mut agent: Box<dyn CognitiveAgent>) -> Result<Uuid> {
        let agent_id = agent.id();
        let agent_name = agent.name().to_string();
        
        // Initialize the agent
        agent.initialize(
            self.context.neural_network.clone(),
            self.context.event_bus.clone()
        ).await?;
        
        // Store in registry
        let mut agents = self.agents.write().await;
        agents.insert(agent_id, agent);
        
        self.logger.info(&format!("Spawned agent: {} ({})", agent_name, agent_id));
        
        // Publish spawn event
        self.context.event_bus.publish(SystemEvent::AgentActivated {
            agent_id,
            agent_type: agent_name,
        }).await;
        
        Ok(agent_id)
    }
    
    pub async fn get_agent(&self, agent_id: Uuid) -> Result<Option<AgentState>> {
        let agents = self.agents.read().await;
        Ok(agents.get(&agent_id).map(|agent| agent.state()))
    }
    
    pub async fn activate_agent(&self, agent_id: Uuid) -> Result<()> {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(&agent_id) {
            agent.activate().await?;
            Ok(())
        } else {
            Err(anyhow!("Agent not found: {}", agent_id))
        }
    }
    
    pub async fn suspend_agent(&self, agent_id: Uuid) -> Result<()> {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(&agent_id) {
            agent.suspend().await?;
            Ok(())
        } else {
            Err(anyhow!("Agent not found: {}", agent_id))
        }
    }
    
    pub async fn terminate_agent(&self, agent_id: Uuid) -> Result<()> {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(&agent_id) {
            agent.terminate().await?;
            agents.remove(&agent_id);
            
            self.logger.info(&format!("Terminated agent: {}", agent_id));
            Ok(())
        } else {
            Err(anyhow!("Agent not found: {}", agent_id))
        }
    }
    
    pub async fn process_all_agents(&self) -> Result<()> {
        let agent_ids: Vec<Uuid> = {
            let agents = self.agents.read().await;
            agents.keys().cloned().collect()
        };
        
        for agent_id in agent_ids {
            let mut agents = self.agents.write().await;
            if let Some(agent) = agents.get_mut(&agent_id) {
                if agent.state() == AgentState::Active {
                    agent.process().await?;
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn broadcast_event(&self, event: SystemEvent) -> Result<()> {
        let agent_ids: Vec<Uuid> = {
            let agents = self.agents.read().await;
            agents.keys().cloned().collect()
        };
        
        for agent_id in agent_ids {
            let mut agents = self.agents.write().await;
            if let Some(agent) = agents.get_mut(&agent_id) {
                agent.receive_event(event.clone()).await?;
            }
        }
        
        Ok(())
    }
    
    pub async fn get_active_agents(&self) -> Vec<(Uuid, String)> {
        let agents = self.agents.read().await;
        agents.iter()
            .filter(|(_, agent)| agent.state() == AgentState::Active)
            .map(|(id, agent)| (*id, agent.name().to_string()))
            .collect()
    }
    
    pub async fn shutdown(&self) -> Result<()> {
        self.logger.info("Shutting down agent registry");
        
        let agent_ids: Vec<Uuid> = {
            let agents = self.agents.read().await;
            agents.keys().cloned().collect()
        };
        
        // Terminate all agents
        for agent_id in agent_ids {
            self.terminate_agent(agent_id).await?;
        }
        
        // Send shutdown event
        self.context.event_bus.publish(SystemEvent::SystemShutdown).await;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_registry_creation() {
        let network = Arc::new(ForgeNeuralNetwork::new());
        let event_bus = Arc::new(EventBus::new());
        
        let registry = AgentRegistry::new(network, event_bus);
        let active_agents = registry.get_active_agents().await;
        
        assert_eq!(active_agents.len(), 0);
    }
}