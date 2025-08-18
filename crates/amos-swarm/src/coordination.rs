use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio::sync::{broadcast, mpsc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Message types for agent coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationMessage {
    /// Direct message between agents
    Direct {
        from: Uuid,
        to: Uuid,
        content: MessageContent,
    },
    
    /// Broadcast to all agents
    Broadcast {
        from: Uuid,
        content: MessageContent,
    },
    
    /// Multicast to specific group
    Multicast {
        from: Uuid,
        to: Vec<Uuid>,
        content: MessageContent,
    },
    
    /// System-level coordination
    System {
        content: SystemMessage,
    },
}

/// Content of coordination messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    /// Task-related coordination
    TaskCoordination {
        task_id: Uuid,
        action: TaskAction,
    },
    
    /// Knowledge sharing
    Knowledge {
        topic: String,
        data: serde_json::Value,
    },
    
    /// Request for help/resources
    Request {
        request_type: RequestType,
        details: String,
    },
    
    /// Response to request
    Response {
        request_id: Uuid,
        result: serde_json::Value,
    },
    
    /// Neural synchronization
    NeuralSync {
        pathways: Vec<PathwayUpdate>,
    },
    
    /// Custom message
    Custom(serde_json::Value),
}

/// Task coordination actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskAction {
    Claim,
    Release,
    Progress(f64),
    Complete(serde_json::Value),
    Failed(String),
    RequestHelp,
}

/// Types of requests agents can make
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    Computation,
    Memory,
    Expertise(String),
    Validation,
    Review,
}

/// System-level messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemMessage {
    AgentJoined(Uuid),
    AgentLeft(Uuid),
    TopologyChange,
    EmergencyStop,
    HealthCheck,
    ConfigUpdate(serde_json::Value),
}

/// Neural pathway update for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathwayUpdate {
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub strength_delta: f64,
    pub reason: String,
}

/// Coordination protocol for agent communication
pub trait CoordinationProtocol: Send + Sync {
    /// Send a message
    fn send(&self, message: CoordinationMessage) -> Result<(), String>;
    
    /// Subscribe to messages
    fn subscribe(&self) -> broadcast::Receiver<CoordinationMessage>;
    
    /// Get protocol capabilities
    fn capabilities(&self) -> Vec<String>;
}

/// Message bus for agent coordination
pub struct MessageBus {
    broadcast_tx: broadcast::Sender<CoordinationMessage>,
    direct_channels: Arc<RwLock<HashMap<Uuid, mpsc::Sender<CoordinationMessage>>>>,
    message_history: Arc<RwLock<Vec<CoordinationMessage>>>,
    max_history: usize,
}

impl MessageBus {
    pub fn new(channel_capacity: usize) -> Self {
        let (broadcast_tx, _) = broadcast::channel(channel_capacity);
        
        Self {
            broadcast_tx,
            direct_channels: Arc::new(RwLock::new(HashMap::new())),
            message_history: Arc::new(RwLock::new(Vec::new())),
            max_history: 1000,
        }
    }
    
    /// Register an agent's direct channel
    pub async fn register_agent(&self, agent_id: Uuid) -> mpsc::Receiver<CoordinationMessage> {
        let (tx, rx) = mpsc::channel(100);
        self.direct_channels.write().await.insert(agent_id, tx);
        rx
    }
    
    /// Unregister an agent
    pub async fn unregister_agent(&self, agent_id: Uuid) {
        self.direct_channels.write().await.remove(&agent_id);
    }
    
    /// Send a coordination message
    pub async fn send(&self, message: CoordinationMessage) -> Result<(), String> {
        // Store in history
        let mut history = self.message_history.write().await;
        history.push(message.clone());
        if history.len() > self.max_history {
            history.remove(0);
        }
        drop(history);
        
        match &message {
            CoordinationMessage::Direct { to, .. } => {
                let channels = self.direct_channels.read().await;
                if let Some(tx) = channels.get(to) {
                    tx.send(message).await
                        .map_err(|_| "Failed to send direct message".to_string())?;
                } else {
                    return Err(format!("Agent {} not found", to));
                }
            }
            
            CoordinationMessage::Broadcast { .. } => {
                self.broadcast_tx.send(message)
                    .map_err(|_| "Failed to broadcast message".to_string())?;
            }
            
            CoordinationMessage::Multicast { to, .. } => {
                let channels = self.direct_channels.read().await;
                for agent_id in to {
                    if let Some(tx) = channels.get(agent_id) {
                        let _ = tx.send(message.clone()).await;
                    }
                }
            }
            
            CoordinationMessage::System { .. } => {
                self.broadcast_tx.send(message)
                    .map_err(|_| "Failed to send system message".to_string())?;
            }
        }
        
        Ok(())
    }
    
    /// Subscribe to broadcast messages
    pub fn subscribe(&self) -> broadcast::Receiver<CoordinationMessage> {
        self.broadcast_tx.subscribe()
    }
    
    /// Get message history
    pub async fn get_history(&self, limit: Option<usize>) -> Vec<CoordinationMessage> {
        let history = self.message_history.read().await;
        let limit = limit.unwrap_or(history.len());
        
        history
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
}

impl CoordinationProtocol for MessageBus {
    fn send(&self, message: CoordinationMessage) -> Result<(), String> {
        // Use tokio::spawn to avoid blocking
        let bus = self.clone();
        tokio::spawn(async move {
            let _ = bus.send(message).await;
        });
        Ok(())
    }
    
    fn subscribe(&self) -> broadcast::Receiver<CoordinationMessage> {
        self.broadcast_tx.subscribe()
    }
    
    fn capabilities(&self) -> Vec<String> {
        vec![
            "direct_messaging".to_string(),
            "broadcast".to_string(),
            "multicast".to_string(),
            "message_history".to_string(),
            "neural_sync".to_string(),
        ]
    }
}

impl Clone for MessageBus {
    fn clone(&self) -> Self {
        Self {
            broadcast_tx: self.broadcast_tx.clone(),
            direct_channels: self.direct_channels.clone(),
            message_history: self.message_history.clone(),
            max_history: self.max_history,
        }
    }
}

/// Helper for creating coordination messages
pub struct MessageBuilder;

impl MessageBuilder {
    pub fn task_progress(from: Uuid, task_id: Uuid, progress: f64) -> CoordinationMessage {
        CoordinationMessage::Broadcast {
            from,
            content: MessageContent::TaskCoordination {
                task_id,
                action: TaskAction::Progress(progress),
            },
        }
    }
    
    pub fn request_help(from: Uuid, task_id: Uuid) -> CoordinationMessage {
        CoordinationMessage::Broadcast {
            from,
            content: MessageContent::TaskCoordination {
                task_id,
                action: TaskAction::RequestHelp,
            },
        }
    }
    
    pub fn share_knowledge(from: Uuid, topic: String, data: serde_json::Value) -> CoordinationMessage {
        CoordinationMessage::Broadcast {
            from,
            content: MessageContent::Knowledge { topic, data },
        }
    }
    
    pub fn neural_sync(from: Uuid, pathways: Vec<PathwayUpdate>) -> CoordinationMessage {
        CoordinationMessage::Broadcast {
            from,
            content: MessageContent::NeuralSync { pathways },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_message_bus() {
        let bus = MessageBus::new(100);
        
        // Register agents
        let agent1 = Uuid::new_v4();
        let agent2 = Uuid::new_v4();
        
        let mut rx1 = bus.register_agent(agent1).await;
        let _rx2 = bus.register_agent(agent2).await;
        
        // Send direct message
        let msg = CoordinationMessage::Direct {
            from: agent2,
            to: agent1,
            content: MessageContent::Custom(serde_json::json!({"test": "message"})),
        };
        
        bus.send(msg).await.unwrap();
        
        // Check reception
        let received = rx1.recv().await.unwrap();
        match received {
            CoordinationMessage::Direct { from, .. } => {
                assert_eq!(from, agent2);
            }
            _ => panic!("Wrong message type"),
        }
    }
    
    #[tokio::test]
    async fn test_broadcast() {
        let bus = MessageBus::new(100);
        
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        
        // Send broadcast
        let msg = CoordinationMessage::Broadcast {
            from: Uuid::new_v4(),
            content: MessageContent::System {
                content: SystemMessage::HealthCheck,
            },
        };
        
        bus.send(msg.clone()).await.unwrap();
        
        // Both should receive
        assert!(matches!(rx1.recv().await.unwrap(), CoordinationMessage::Broadcast { .. }));
        assert!(matches!(rx2.recv().await.unwrap(), CoordinationMessage::Broadcast { .. }));
    }
}