pub mod orchestrator;
pub mod topology;
pub mod task;
pub mod coordination;

pub use orchestrator::{SwarmOrchestrator, SwarmConfig};
pub use topology::{SwarmTopology, AgentPlacement};
pub use task::{Task, TaskResult, TaskStrategy};
pub use coordination::{CoordinationProtocol, MessageBus};

use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use amos_core::neural::ForgeNeuralNetwork;
use amos_agents::CognitiveAgent;
use std::collections::HashMap;

/// AMOS Swarm - Biological intelligence orchestration inspired by ruv-swarm
/// 
/// This module provides swarm orchestration capabilities for AMOS agents,
/// allowing them to work together in various topologies to solve complex tasks.
#[derive(Clone)]
pub struct AmosSwarm {
    pub id: Uuid,
    pub name: String,
    pub topology: SwarmTopology,
    pub agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>,
    pub neural_network: Arc<ForgeNeuralNetwork>,
    pub orchestrator: Arc<SwarmOrchestrator>,
}

impl AmosSwarm {
    pub fn new(
        name: String,
        topology: SwarmTopology,
        neural_network: Arc<ForgeNeuralNetwork>,
    ) -> Self {
        let orchestrator = Arc::new(SwarmOrchestrator::new(
            topology.clone(),
            neural_network.clone(),
        ));
        
        Self {
            id: Uuid::new_v4(),
            name,
            topology,
            agents: Arc::new(RwLock::new(HashMap::new())),
            neural_network,
            orchestrator,
        }
    }
    
    /// Spawn a new agent into the swarm
    pub async fn spawn_agent(
        &self,
        agent: Arc<dyn CognitiveAgent>,
    ) -> Result<Uuid, String> {
        let agent_id = agent.id();
        let mut agents = self.agents.write().await;
        
        // Check swarm capacity based on topology
        let max_agents = match &self.topology {
            SwarmTopology::Mesh { max_connections } => max_connections * 10,
            SwarmTopology::Hierarchical { levels, agents_per_level } => levels * agents_per_level,
            SwarmTopology::Ring => 100,
            SwarmTopology::Star { max_satellites } => max_satellites + 1,
        };
        
        if agents.len() >= max_agents {
            return Err("Swarm at maximum capacity".to_string());
        }
        
        agents.insert(agent_id, agent);
        
        // Notify orchestrator of new agent
        self.orchestrator.on_agent_joined(agent_id).await;
        
        Ok(agent_id)
    }
    
    /// Remove an agent from the swarm
    pub async fn remove_agent(&self, agent_id: Uuid) -> Result<(), String> {
        let mut agents = self.agents.write().await;
        
        if agents.remove(&agent_id).is_none() {
            return Err(format!("Agent {} not found in swarm", agent_id));
        }
        
        // Notify orchestrator of agent departure
        self.orchestrator.on_agent_left(agent_id).await;
        
        Ok(())
    }
    
    /// Orchestrate a task across the swarm
    pub async fn orchestrate(
        &self,
        task: Task,
        strategy: TaskStrategy,
    ) -> Result<TaskResult, String> {
        let agents = self.agents.read().await;
        
        if agents.is_empty() {
            return Err("No agents available in swarm".to_string());
        }
        
        // Delegate to orchestrator
        self.orchestrator.execute_task(
            task,
            strategy,
            agents.clone(),
        ).await
    }
    
    /// Get swarm status
    pub async fn status(&self) -> SwarmStatus {
        let agents = self.agents.read().await;
        
        SwarmStatus {
            id: self.id,
            name: self.name.clone(),
            topology: self.topology.clone(),
            agent_count: agents.len(),
            active_tasks: self.orchestrator.active_task_count().await,
            health: self.calculate_health(&agents).await,
        }
    }
    
    async fn calculate_health(
        &self,
        agents: &HashMap<Uuid, Arc<dyn CognitiveAgent>>,
    ) -> f64 {
        // Calculate swarm health based on agent states and neural activity
        let mut total_health = 0.0;
        
        for agent in agents.values() {
            // In production, query actual agent health
            total_health += 0.9; // Placeholder
        }
        
        if agents.is_empty() {
            0.0
        } else {
            total_health / agents.len() as f64
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwarmStatus {
    pub id: Uuid,
    pub name: String,
    pub topology: SwarmTopology,
    pub agent_count: usize,
    pub active_tasks: usize,
    pub health: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use amos_agents::ArchitectAgent;
    
    #[tokio::test]
    async fn test_swarm_creation() {
        let neural_network = Arc::new(ForgeNeuralNetwork::new());
        let swarm = AmosSwarm::new(
            "Test Swarm".to_string(),
            SwarmTopology::Mesh { max_connections: 6 },
            neural_network.clone(),
        );
        
        let status = swarm.status().await;
        assert_eq!(status.name, "Test Swarm");
        assert_eq!(status.agent_count, 0);
    }
    
    #[tokio::test]
    async fn test_agent_spawning() {
        let neural_network = Arc::new(ForgeNeuralNetwork::new());
        let swarm = AmosSwarm::new(
            "Test Swarm".to_string(),
            SwarmTopology::Mesh { max_connections: 6 },
            neural_network.clone(),
        );
        
        let agent = Arc::new(ArchitectAgent::new(
            Uuid::new_v4(),
            "Test Architect",
            neural_network,
            false,
        ));
        
        let agent_id = swarm.spawn_agent(agent).await.unwrap();
        
        let status = swarm.status().await;
        assert_eq!(status.agent_count, 1);
        
        // Remove agent
        swarm.remove_agent(agent_id).await.unwrap();
        
        let status = swarm.status().await;
        assert_eq!(status.agent_count, 0);
    }
}