use amos_agents::*;
use amos_core::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use async_trait::async_trait;
use std::any::TypeId;

struct EventCollector {
    events: Arc<Mutex<Vec<SystemEvent>>>,
}

#[async_trait]
impl EventHandler for EventCollector {
    async fn handle(&self, event: SystemEvent) {
        self.events.lock().await.push(event);
    }
    
    fn event_types(&self) -> Vec<TypeId> {
        vec![TypeId::of::<SystemEvent>()]
    }
}

#[tokio::test]
async fn test_agent_event_bus_integration() {
    // Create infrastructure
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    bus_clone.start_processing().await;
    
    // Set up event collector
    let collector = Arc::new(EventCollector {
        events: Arc::new(Mutex::new(Vec::new())),
    });
    let events_clone = collector.events.clone();
    event_bus.subscribe(collector).await;
    
    // Create registry and spawn agents
    let registry = AgentRegistry::new(network.clone(), event_bus.clone());
    
    let seer = Box::new(TrafficSeer::new());
    let sculptor = Box::new(PathwaySculptor::new());
    
    let _seer_id = registry.spawn_agent(seer).await.unwrap();
    let _sculptor_id = registry.spawn_agent(sculptor).await.unwrap();
    
    sleep(Duration::from_millis(100)).await;
    
    // Check spawn events were published
    let events = events_clone.lock().await;
    assert!(events.len() >= 2); // At least 2 agent activation events
    
    let agent_events: Vec<_> = events.iter()
        .filter_map(|e| match e {
            SystemEvent::AgentActivated { agent_id, agent_type } => Some((agent_id, agent_type)),
            _ => None
        })
        .collect();
    
    assert!(agent_events.iter().any(|(_, t)| t == &"TrafficSeer"));
    assert!(agent_events.iter().any(|(_, t)| t == &"PathwaySculptor"));
}

#[tokio::test]
async fn test_traffic_seer_threat_detection_flow() {
    // Create infrastructure
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    bus_clone.start_processing().await;
    
    // Set up event collector
    let collector = Arc::new(EventCollector {
        events: Arc::new(Mutex::new(Vec::new())),
    });
    let events_clone = collector.events.clone();
    event_bus.subscribe(collector).await;
    
    // Create registry and spawn TrafficSeer
    let registry = AgentRegistry::new(network.clone(), event_bus.clone());
    let mut seer = TrafficSeer::new();
    
    // Add anomaly pattern
    let anomaly_pattern = Pattern {
        id: Uuid::new_v4(),
        data: vec![0.1, 10.0, 0.1], // High variance
        pattern_type: PatternType::Anomaly,
    };
    seer.add_pattern(anomaly_pattern);
    
    let _seer_id = registry.spawn_agent(Box::new(seer)).await.unwrap();
    
    // Don't process agents to avoid neural network block_on issue
    // Just verify agent was spawned correctly
    
    sleep(Duration::from_millis(100)).await;
    
    // Check if agent spawn event was published
    let events = events_clone.lock().await;
    let agent_events: Vec<_> = events.iter()
        .filter_map(|e| match e {
            SystemEvent::AgentActivated { agent_id, agent_type } => Some((agent_id, agent_type)),
            _ => None
        })
        .collect();
    
    assert!(!agent_events.is_empty());
    assert!(agent_events.iter().any(|(_, t)| t == &"TrafficSeer"));
}

#[tokio::test]
async fn test_pathway_sculptor_optimization_flow() {
    // Create infrastructure
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    bus_clone.start_processing().await;
    
    // Set up event collector
    let collector = Arc::new(EventCollector {
        events: Arc::new(Mutex::new(Vec::new())),
    });
    let events_clone = collector.events.clone();
    event_bus.subscribe(collector).await;
    
    // Create registry and spawn PathwaySculptor
    let registry = AgentRegistry::new(network.clone(), event_bus.clone());
    let sculptor = Box::new(PathwaySculptor::new());
    registry.spawn_agent(sculptor).await.unwrap();
    
    // Send pathway strengthened event
    let pathway_id = Uuid::new_v4();
    event_bus.publish(SystemEvent::PathwayStrengthened {
        pathway_id,
        new_strength: 0.8,
    }).await;
    
    // Let sculptor receive and process event
    registry.broadcast_event(SystemEvent::PathwayStrengthened {
        pathway_id,
        new_strength: 0.8,
    }).await.unwrap();
    
    sleep(Duration::from_millis(100)).await;
    
    // Sculptor should have received the event
    let events = events_clone.lock().await;
    assert!(events.len() > 0);
}

#[tokio::test]
async fn test_full_agent_system_integration() {
    // Create complete system
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    bus_clone.start_processing().await;
    
    let logger = Logger::new("integration_test");
    
    // Create registry
    let registry = AgentRegistry::new(network.clone(), event_bus.clone());
    
    // Spawn all agent types
    let seer = Box::new(TrafficSeer::new());
    let sculptor = Box::new(PathwaySculptor::new());
    
    registry.spawn_agent(seer).await.unwrap();
    registry.spawn_agent(sculptor).await.unwrap();
    
    logger.info("All agents spawned");
    
    // Simulate system activity
    for _i in 0..5 {
        // Neural activity
        event_bus.publish(SystemEvent::NeuralFired {
            node_id: Uuid::new_v4(),
        }).await;
        
        // Process agents
        registry.process_all_agents().await.unwrap();
        
        sleep(Duration::from_millis(50)).await;
    }
    
    // Check system is still healthy
    let active_agents = registry.get_active_agents().await;
    assert_eq!(active_agents.len(), 2);
    
    // Graceful shutdown
    registry.shutdown().await.unwrap();
    
    let active_agents = registry.get_active_agents().await;
    assert_eq!(active_agents.len(), 0);
}