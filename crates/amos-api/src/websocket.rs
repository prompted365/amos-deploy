use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, State},
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio::sync::broadcast;
use tracing::{info, error};
use crate::{AppState, ApiError};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    // Client -> Server
    Subscribe { channels: Vec<String> },
    Unsubscribe { channels: Vec<String> },
    AgentCommand { agent_id: Uuid, command: String },
    SwarmOrchestrate { swarm_id: Uuid, task: String },
    
    // Server -> Client
    AgentUpdate { agent_id: Uuid, state: String },
    NeuralActivity { pathway_id: Uuid, strength: f64 },
    HormonalBurst { hormone: String, level: f64 },
    SwarmEvent { swarm_id: Uuid, event: String },
    TaskProgress { task_id: Uuid, progress: f64 },
    Error { message: String },
}

pub struct WsState {
    pub broadcast_tx: broadcast::Sender<WsMessage>,
}

impl WsState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { broadcast_tx: tx }
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Result<Response, ApiError> {
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state)))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let client_id = Uuid::new_v4();
    
    info!("WebSocket client connected: {}", client_id);
    
    // Create broadcast receiver for this client
    let mut broadcast_rx = state.ws_state.broadcast_tx.subscribe();
    
    // Spawn task to forward broadcast messages to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = broadcast_rx.recv().await {
            if let Ok(text) = serde_json::to_string(&msg) {
                if sender.send(axum::extract::ws::Message::Text(text)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Handle incoming messages
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                axum::extract::ws::Message::Text(text) => {
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        handle_ws_message(ws_msg, &state_clone, client_id).await;
                    }
                }
                axum::extract::ws::Message::Close(_) => break,
                _ => {}
            }
        }
    });
    
    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
    
    info!("WebSocket client disconnected: {}", client_id);
}

async fn handle_ws_message(msg: WsMessage, state: &AppState, client_id: Uuid) {
    match msg {
        WsMessage::Subscribe { channels } => {
            info!("Client {} subscribing to channels: {:?}", client_id, channels);
            // In production, implement channel-based filtering
        }
        
        WsMessage::AgentCommand { agent_id, command } => {
            let agents = state.agents.read().await;
            if let Some(agent) = agents.get(&agent_id) {
                info!("Executing command '{}' on agent {}", command, agent_id);
                
                // Broadcast agent state update
                let update = WsMessage::AgentUpdate {
                    agent_id,
                    state: format!("{:?}", agent.state()),
                };
                let _ = state.ws_state.broadcast_tx.send(update);
            }
        }
        
        WsMessage::SwarmOrchestrate { swarm_id, task } => {
            info!("Orchestrating task for swarm {}: {}", swarm_id, task);
            
            // Simulate task progress updates
            let task_id = Uuid::new_v4();
            let tx = state.ws_state.broadcast_tx.clone();
            
            tokio::spawn(async move {
                for progress in [0.0, 0.25, 0.5, 0.75, 1.0] {
                    let _ = tx.send(WsMessage::TaskProgress { task_id, progress });
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            });
        }
        
        _ => {
            error!("Unexpected client message: {:?}", msg);
        }
    }
}

// Neural activity broadcaster
pub fn start_neural_activity_broadcaster(state: AppState) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
        
        loop {
            interval.tick().await;
            
            // Simulate neural activity
            let activity = WsMessage::NeuralActivity {
                pathway_id: Uuid::new_v4(),
                strength: rand::random::<f64>(),
            };
            
            let _ = state.ws_state.broadcast_tx.send(activity);
            
            // Occasionally send hormonal bursts
            if rand::random::<f64>() > 0.7 {
                let hormones = ["dopamine", "serotonin", "cortisol", "oxytocin"];
                let hormone = hormones[rand::random::<usize>() % hormones.len()];
                
                let burst = WsMessage::HormonalBurst {
                    hormone: hormone.to_string(),
                    level: rand::random::<f64>(),
                };
                
                let _ = state.ws_state.broadcast_tx.send(burst);
            }
        }
    });
}