use amos_core::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use async_trait::async_trait;
use std::any::TypeId;

struct NeuralEventLogger {
    logger: Logger,
    event_count: Arc<Mutex<usize>>,
}

#[async_trait]
impl EventHandler for NeuralEventLogger {
    async fn handle(&self, event: SystemEvent) {
        let mut count = self.event_count.lock().await;
        *count += 1;
        
        match event {
            SystemEvent::NeuralFired { node_id } => {
                log_context!(
                    self.logger,
                    info,
                    "Neural node fired",
                    "node_id" => node_id,
                    "event_number" => *count
                );
            }
            SystemEvent::PathwayStrengthened { pathway_id, new_strength } => {
                log_context!(
                    self.logger,
                    info,
                    "Pathway strengthened",
                    "pathway_id" => pathway_id,
                    "new_strength" => new_strength
                );
            }
            _ => {}
        }
    }
    
    fn event_types(&self) -> Vec<TypeId> {
        vec![TypeId::of::<SystemEvent>()]
    }
}

#[tokio::test]
async fn test_neural_to_event_bus_integration() {
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    bus_clone.start_processing().await;
    
    // Set up neural event logger
    let logger = Arc::new(NeuralEventLogger {
        logger: Logger::new("neural_integration"),
        event_count: Arc::new(Mutex::new(0)),
    });
    
    let count_clone = logger.event_count.clone();
    event_bus.subscribe(logger).await;
    
    // Simulate neural activity without creating actual network (to avoid block_on issues)
    let node1 = Uuid::new_v4();
    let node2 = Uuid::new_v4();
    let pathway_id = Uuid::new_v4();
    
    event_bus.publish(SystemEvent::NeuralFired { node_id: node1 }).await;
    event_bus.publish(SystemEvent::NeuralFired { node_id: node2 }).await;
    event_bus.publish(SystemEvent::PathwayStrengthened { 
        pathway_id, 
        new_strength: 0.7 
    }).await;
    
    sleep(Duration::from_millis(100)).await;
    
    // Verify events were logged
    let count = count_clone.lock().await;
    assert_eq!(*count, 3);
}

#[tokio::test]
async fn test_hormonal_immune_event_integration() {
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    bus_clone.start_processing().await;
    
    // Create message router for specialized routing
    let router = Arc::new(MessageRouter::new());
    let mut hormonal_rx = router.register_route("hormonal".to_string()).await;
    let mut immune_rx = router.register_route("immune".to_string()).await;
    
    // Simulate hormonal burst
    let burst_event = SystemEvent::HormonalBurst {
        hormone_type: "Cortisol".to_string(),
        intensity: 0.8,
    };
    
    router.route_message("hormonal", burst_event.clone()).await.unwrap();
    
    // Simulate threat detection
    let threat_event = SystemEvent::ThreatDetected {
        threat_id: Uuid::new_v4(),
        level: "High".to_string(),
    };
    
    router.route_message("immune", threat_event.clone()).await.unwrap();
    
    // Verify routing
    if let Some(received) = hormonal_rx.recv().await {
        match received {
            SystemEvent::HormonalBurst { hormone_type, intensity } => {
                assert_eq!(hormone_type, "Cortisol");
                assert_eq!(intensity, 0.8);
            }
            _ => panic!("Wrong event type in hormonal route"),
        }
    }
    
    if let Some(received) = immune_rx.recv().await {
        match received {
            SystemEvent::ThreatDetected { level, .. } => {
                assert_eq!(level, "High");
            }
            _ => panic!("Wrong event type in immune route"),
        }
    }
}

#[tokio::test]
async fn test_full_system_integration() {
    // Create all components
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    bus_clone.start_processing().await;
    
    let mut hormonal_state = HormonalState::new();
    let immune_system = ForgeImmuneSystem::new();
    
    // Set up comprehensive logging
    let system_logger = Arc::new(NeuralEventLogger {
        logger: Logger::new("amos_system").with_level(LogLevel::Debug),
        event_count: Arc::new(Mutex::new(0)),
    });
    
    let count_clone = system_logger.event_count.clone();
    event_bus.subscribe(system_logger).await;
    
    // Simulate complex system interaction
    // 1. Neural activity (simulated without actual network)
    let memory_node = Uuid::new_v4();
    event_bus.publish(SystemEvent::NeuralFired { node_id: memory_node }).await;
    
    // 2. Hormonal response
    let dopamine_burst = HormonalBurst {
        id: Uuid::new_v4(),
        hormone: HormoneType::Dopamine,
        intensity: 0.6,
        triggered_at: chrono::Utc::now(),
        duration_ms: 5000,
    };
    
    hormonal_state.apply_burst(&dopamine_burst);
    event_bus.publish(SystemEvent::HormonalBurst {
        hormone_type: "Dopamine".to_string(),
        intensity: 0.6,
    }).await;
    
    // 3. Pattern detection
    let pattern = Pattern {
        id: Uuid::new_v4(),
        data: vec![1.0, 2.0, 3.0],
        pattern_type: PatternType::Normal,
    };
    
    let threat_level = immune_system.detect_anomaly(&pattern).await;
    if threat_level.is_some() {
        event_bus.publish(SystemEvent::ThreatDetected {
            threat_id: Uuid::new_v4(),
            level: format!("{:?}", threat_level.unwrap()),
        }).await;
    }
    
    // 4. Agent activation
    event_bus.publish(SystemEvent::AgentActivated {
        agent_id: Uuid::new_v4(),
        agent_type: "MemoryWeaver".to_string(),
    }).await;
    
    // 5. Memory storage
    event_bus.publish(SystemEvent::MemoryStored {
        memory_id: Uuid::new_v4(),
        content_size: 2048,
    }).await;
    
    sleep(Duration::from_millis(200)).await;
    
    // Verify system processed all events
    let count = count_clone.lock().await;
    assert!(*count >= 4); // At least the 4 events we published (no threat detected since no detectors)
}

#[tokio::test]
async fn test_system_shutdown_propagation() {
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    bus_clone.start_processing().await;
    
    // Create multiple handlers
    let handler1 = Arc::new(NeuralEventLogger {
        logger: Logger::new("handler1"),
        event_count: Arc::new(Mutex::new(0)),
    });
    
    let handler2 = Arc::new(NeuralEventLogger {
        logger: Logger::new("handler2"),
        event_count: Arc::new(Mutex::new(0)),
    });
    
    let count1 = handler1.event_count.clone();
    let count2 = handler2.event_count.clone();
    
    event_bus.subscribe(handler1).await;
    event_bus.subscribe(handler2).await;
    
    // Send some events
    event_bus.publish(SystemEvent::NeuralFired { node_id: Uuid::new_v4() }).await;
    event_bus.publish(SystemEvent::SystemShutdown).await;
    
    sleep(Duration::from_millis(100)).await;
    
    // Both handlers should have processed events before shutdown
    assert_eq!(*count1.lock().await, 2);
    assert_eq!(*count2.lock().await, 2);
}