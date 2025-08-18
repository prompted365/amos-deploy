use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use async_trait::async_trait;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SystemEvent {
    NeuralFired { node_id: Uuid },
    PathwayStrengthened { pathway_id: Uuid, new_strength: f64 },
    HormonalBurst { hormone_type: String, intensity: f64 },
    ThreatDetected { threat_id: Uuid, level: String },
    AgentActivated { agent_id: Uuid, agent_type: String },
    MemoryStored { memory_id: Uuid, content_size: usize },
    SystemShutdown,
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: SystemEvent);
    fn event_types(&self) -> Vec<TypeId>;
}

type HandlerId = Uuid;
type EventHandlers = HashMap<TypeId, Vec<(HandlerId, Arc<dyn EventHandler>)>>;

pub struct EventBus {
    handlers: Arc<RwLock<EventHandlers>>,
    event_tx: mpsc::UnboundedSender<SystemEvent>,
    event_rx: Arc<RwLock<mpsc::UnboundedReceiver<SystemEvent>>>,
}

impl EventBus {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            event_rx: Arc::new(RwLock::new(event_rx)),
        }
    }
    
    pub async fn subscribe(&self, handler: Arc<dyn EventHandler>) -> HandlerId {
        let handler_id = Uuid::new_v4();
        let mut handlers = self.handlers.write().await;
        
        for event_type in handler.event_types() {
            handlers
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push((handler_id, handler.clone()));
        }
        
        handler_id
    }
    
    pub async fn unsubscribe(&self, handler_id: HandlerId) {
        let mut handlers = self.handlers.write().await;
        
        for (_, handler_list) in handlers.iter_mut() {
            handler_list.retain(|(id, _)| *id != handler_id);
        }
    }
    
    pub async fn publish(&self, event: SystemEvent) {
        let _ = self.event_tx.send(event);
    }
    
    pub async fn start_processing(self: Arc<Self>) {
        let handlers = self.handlers.clone();
        let event_rx = self.event_rx.clone();
        
        tokio::spawn(async move {
            let mut rx = event_rx.write().await;
            
            while let Some(event) = rx.recv().await {
                let type_id = TypeId::of::<SystemEvent>();
                let handlers_guard = handlers.read().await;
                
                if let Some(handler_list) = handlers_guard.get(&type_id) {
                    for (_, handler) in handler_list {
                        let event_clone = event.clone();
                        let handler_clone = handler.clone();
                        
                        tokio::spawn(async move {
                            handler_clone.handle(event_clone).await;
                        });
                    }
                }
                
                if event == SystemEvent::SystemShutdown {
                    break;
                }
            }
        });
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LoggingHandler;

#[async_trait]
impl EventHandler for LoggingHandler {
    async fn handle(&self, event: SystemEvent) {
        println!("[EVENT] {:?}", event);
    }
    
    fn event_types(&self) -> Vec<TypeId> {
        vec![TypeId::of::<SystemEvent>()]
    }
}

pub struct MessageRouter {
    routes: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<SystemEvent>>>>,
}

impl MessageRouter {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn register_route(&self, route_name: String) -> mpsc::UnboundedReceiver<SystemEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.routes.write().await.insert(route_name, tx);
        rx
    }
    
    pub async fn route_message(&self, route_name: &str, event: SystemEvent) -> Result<(), String> {
        let routes = self.routes.read().await;
        
        if let Some(tx) = routes.get(route_name) {
            tx.send(event).map_err(|_| "Failed to send message".to_string())
        } else {
            Err(format!("Route '{}' not found", route_name))
        }
    }
}

impl Default for MessageRouter {
    fn default() -> Self {
        Self::new()
    }
}