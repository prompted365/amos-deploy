use amos_agents::*;
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_registry_spawn_agent() {
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let registry = AgentRegistry::new(network, event_bus);
    
    // Spawn a TrafficSeer
    let seer = Box::new(TrafficSeer::new());
    let agent_id = registry.spawn_agent(seer).await.unwrap();
    
    // Check agent exists and is active
    let state = registry.get_agent(agent_id).await.unwrap();
    assert_eq!(state, Some(AgentState::Active));
}

#[tokio::test]
async fn test_registry_multiple_agents() {
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let registry = AgentRegistry::new(network, event_bus);
    
    // Spawn multiple agents
    let seer = Box::new(TrafficSeer::new());
    let sculptor = Box::new(PathwaySculptor::new());
    
    let _seer_id = registry.spawn_agent(seer).await.unwrap();
    let _sculptor_id = registry.spawn_agent(sculptor).await.unwrap();
    
    // Check both are active
    let active_agents = registry.get_active_agents().await;
    assert_eq!(active_agents.len(), 2);
    
    let agent_names: Vec<String> = active_agents.iter().map(|(_, name)| name.clone()).collect();
    assert!(agent_names.contains(&"TrafficSeer".to_string()));
    assert!(agent_names.contains(&"PathwaySculptor".to_string()));
}

#[tokio::test]
async fn test_registry_agent_lifecycle() {
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let registry = AgentRegistry::new(network, event_bus);
    
    // Spawn agent
    let seer = Box::new(TrafficSeer::new());
    let agent_id = registry.spawn_agent(seer).await.unwrap();
    
    // Suspend agent
    registry.suspend_agent(agent_id).await.unwrap();
    let state = registry.get_agent(agent_id).await.unwrap();
    assert_eq!(state, Some(AgentState::Suspended));
    
    // Reactivate agent
    registry.activate_agent(agent_id).await.unwrap();
    let state = registry.get_agent(agent_id).await.unwrap();
    assert_eq!(state, Some(AgentState::Active));
    
    // Terminate agent
    registry.terminate_agent(agent_id).await.unwrap();
    let state = registry.get_agent(agent_id).await.unwrap();
    assert_eq!(state, None);
}

#[tokio::test]
async fn test_registry_process_all_agents() {
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let registry = AgentRegistry::new(network, event_bus);
    
    // Spawn multiple agents
    let seer1 = Box::new(TrafficSeer::new());
    let seer2 = Box::new(TrafficSeer::new());
    
    registry.spawn_agent(seer1).await.unwrap();
    registry.spawn_agent(seer2).await.unwrap();
    
    // Process all agents
    registry.process_all_agents().await.unwrap();
    
    // Agents should still be active
    let active_agents = registry.get_active_agents().await;
    assert_eq!(active_agents.len(), 2);
}

#[tokio::test]
async fn test_registry_broadcast_event() {
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let registry = AgentRegistry::new(network, event_bus);
    
    // Spawn agents
    let seer = Box::new(TrafficSeer::new());
    let sculptor = Box::new(PathwaySculptor::new());
    
    registry.spawn_agent(seer).await.unwrap();
    registry.spawn_agent(sculptor).await.unwrap();
    
    // Broadcast event
    let event = SystemEvent::NeuralFired { node_id: Uuid::new_v4() };
    registry.broadcast_event(event).await.unwrap();
    
    // Both agents should have received the event
    // (They're still active, which means event handling didn't crash them)
    let active_agents = registry.get_active_agents().await;
    assert_eq!(active_agents.len(), 2);
}

#[tokio::test]
async fn test_registry_shutdown() {
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let registry = AgentRegistry::new(network, event_bus);
    
    // Spawn multiple agents
    let seer = Box::new(TrafficSeer::new());
    let sculptor = Box::new(PathwaySculptor::new());
    
    registry.spawn_agent(seer).await.unwrap();
    registry.spawn_agent(sculptor).await.unwrap();
    
    // Shutdown registry
    registry.shutdown().await.unwrap();
    
    // No agents should be active
    let active_agents = registry.get_active_agents().await;
    assert_eq!(active_agents.len(), 0);
}

#[tokio::test]
async fn test_registry_invalid_agent_id() {
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let registry = AgentRegistry::new(network, event_bus);
    
    let fake_id = Uuid::new_v4();
    
    // Try to activate non-existent agent
    let result = registry.activate_agent(fake_id).await;
    assert!(result.is_err());
    
    // Try to suspend non-existent agent
    let result = registry.suspend_agent(fake_id).await;
    assert!(result.is_err());
    
    // Try to terminate non-existent agent
    let result = registry.terminate_agent(fake_id).await;
    assert!(result.is_err());
}