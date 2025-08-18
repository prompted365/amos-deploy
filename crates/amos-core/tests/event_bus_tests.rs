use amos_core::event_bus::*;
use std::sync::Arc;
use std::any::TypeId;
use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

struct TestEventHandler {
    received_events: Arc<Mutex<Vec<SystemEvent>>>,
}

#[async_trait]
impl EventHandler for TestEventHandler {
    async fn handle(&self, event: SystemEvent) {
        self.received_events.lock().await.push(event);
    }
    
    fn event_types(&self) -> Vec<TypeId> {
        vec![TypeId::of::<SystemEvent>()]
    }
}

#[tokio::test]
async fn test_event_bus_creation() {
    let event_bus = EventBus::new();
    
    // Should be able to create without errors
    let _bus_arc = Arc::new(event_bus);
}

#[tokio::test]
async fn test_event_subscription() {
    let event_bus = Arc::new(EventBus::new());
    
    let handler = Arc::new(TestEventHandler {
        received_events: Arc::new(Mutex::new(Vec::new())),
    });
    
    let handler_id = event_bus.subscribe(handler).await;
    
    // Handler ID should be valid UUID
    assert_ne!(handler_id, Uuid::nil());
}

#[tokio::test]
async fn test_event_publishing() {
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    
    // Start processing in background
    bus_clone.start_processing().await;
    
    let handler = Arc::new(TestEventHandler {
        received_events: Arc::new(Mutex::new(Vec::new())),
    });
    
    let events_clone = handler.received_events.clone();
    event_bus.subscribe(handler).await;
    
    // Publish an event
    let node_id = Uuid::new_v4();
    event_bus.publish(SystemEvent::NeuralFired { node_id }).await;
    
    // Give time for async processing
    sleep(Duration::from_millis(100)).await;
    
    // Check event was received
    let events = events_clone.lock().await;
    assert_eq!(events.len(), 1);
    match &events[0] {
        SystemEvent::NeuralFired { node_id: received_id } => {
            assert_eq!(*received_id, node_id);
        }
        _ => panic!("Wrong event type received"),
    }
}

#[tokio::test]
async fn test_multiple_handlers() {
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    
    bus_clone.start_processing().await;
    
    let handler1 = Arc::new(TestEventHandler {
        received_events: Arc::new(Mutex::new(Vec::new())),
    });
    let handler2 = Arc::new(TestEventHandler {
        received_events: Arc::new(Mutex::new(Vec::new())),
    });
    
    let events1 = handler1.received_events.clone();
    let events2 = handler2.received_events.clone();
    
    event_bus.subscribe(handler1).await;
    event_bus.subscribe(handler2).await;
    
    // Publish an event
    event_bus.publish(SystemEvent::HormonalBurst {
        hormone_type: "Dopamine".to_string(),
        intensity: 0.7,
    }).await;
    
    sleep(Duration::from_millis(100)).await;
    
    // Both handlers should receive the event
    assert_eq!(events1.lock().await.len(), 1);
    assert_eq!(events2.lock().await.len(), 1);
}

#[tokio::test]
async fn test_unsubscribe() {
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    
    bus_clone.start_processing().await;
    
    let handler = Arc::new(TestEventHandler {
        received_events: Arc::new(Mutex::new(Vec::new())),
    });
    
    let events_clone = handler.received_events.clone();
    let handler_id = event_bus.subscribe(handler).await;
    
    // Unsubscribe
    event_bus.unsubscribe(handler_id).await;
    
    // Publish an event
    event_bus.publish(SystemEvent::ThreatDetected {
        threat_id: Uuid::new_v4(),
        level: "High".to_string(),
    }).await;
    
    sleep(Duration::from_millis(100)).await;
    
    // Handler should not receive the event
    assert_eq!(events_clone.lock().await.len(), 0);
}

#[tokio::test]
async fn test_message_router() {
    let router = MessageRouter::new();
    
    // Register a route
    let mut receiver = router.register_route("neural_events".to_string()).await;
    
    // Route a message
    let event = SystemEvent::NeuralFired { node_id: Uuid::new_v4() };
    let result = router.route_message("neural_events", event.clone()).await;
    
    assert!(result.is_ok());
    
    // Check message was received
    if let Some(received) = receiver.recv().await {
        match (received, event) {
            (SystemEvent::NeuralFired { node_id: id1 }, SystemEvent::NeuralFired { node_id: id2 }) => {
                assert_eq!(id1, id2);
            }
            _ => panic!("Event mismatch"),
        }
    } else {
        panic!("No message received");
    }
}

#[tokio::test]
async fn test_invalid_route() {
    let router = MessageRouter::new();
    
    let event = SystemEvent::SystemShutdown;
    let result = router.route_message("non_existent", event).await;
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Route 'non_existent' not found");
}

#[tokio::test]
async fn test_system_shutdown() {
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    
    bus_clone.start_processing().await;
    
    let handler = Arc::new(TestEventHandler {
        received_events: Arc::new(Mutex::new(Vec::new())),
    });
    
    let events_clone = handler.received_events.clone();
    event_bus.subscribe(handler).await;
    
    // Send shutdown event
    event_bus.publish(SystemEvent::SystemShutdown).await;
    
    sleep(Duration::from_millis(100)).await;
    
    // Handler should receive shutdown
    let events = events_clone.lock().await;
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], SystemEvent::SystemShutdown));
}

#[tokio::test]
async fn test_different_event_types() {
    let event_bus = Arc::new(EventBus::new());
    let bus_clone = event_bus.clone();
    
    bus_clone.start_processing().await;
    
    let handler = Arc::new(TestEventHandler {
        received_events: Arc::new(Mutex::new(Vec::new())),
    });
    
    let events_clone = handler.received_events.clone();
    event_bus.subscribe(handler).await;
    
    // Publish different event types
    let events = vec![
        SystemEvent::NeuralFired { node_id: Uuid::new_v4() },
        SystemEvent::PathwayStrengthened { pathway_id: Uuid::new_v4(), new_strength: 0.8 },
        SystemEvent::HormonalBurst { hormone_type: "Cortisol".to_string(), intensity: 0.5 },
        SystemEvent::ThreatDetected { threat_id: Uuid::new_v4(), level: "Medium".to_string() },
        SystemEvent::AgentActivated { agent_id: Uuid::new_v4(), agent_type: "Memory".to_string() },
        SystemEvent::MemoryStored { memory_id: Uuid::new_v4(), content_size: 1024 },
    ];
    
    for event in &events {
        event_bus.publish(event.clone()).await;
    }
    
    sleep(Duration::from_millis(200)).await;
    
    // All events should be received
    let received = events_clone.lock().await;
    assert_eq!(received.len(), events.len());
}