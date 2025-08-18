//! Example WebSocket client for AMOS API
//! 
//! Run with: cargo run --example websocket_client

use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to AMOS WebSocket server...");
    
    let url = "ws://localhost:3000/ws";
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to {}", url);
    
    let (mut write, mut read) = ws_stream.split();
    
    // Subscribe to all channels
    let subscribe_msg = json!({
        "type": "Subscribe",
        "data": {
            "channels": ["neural", "agents", "swarms", "hormones"]
        }
    });
    
    write.send(Message::Text(subscribe_msg.to_string())).await?;
    println!("Subscribed to channels");
    
    // Spawn task to send periodic commands
    let write_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            
            // Send a test command
            let cmd = json!({
                "type": "AgentCommand",
                "data": {
                    "agent_id": "00000000-0000-0000-0000-000000000000",
                    "command": "status"
                }
            });
            
            if write.send(Message::Text(cmd.to_string())).await.is_err() {
                break;
            }
            
            println!("Sent agent command");
        }
    });
    
    // Read messages
    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
                if let Ok(json_msg) = serde_json::from_str::<serde_json::Value>(&text) {
                    match json_msg["type"].as_str() {
                        Some("NeuralActivity") => {
                            let strength = json_msg["data"]["strength"].as_f64().unwrap_or(0.0);
                            println!("🧠 Neural activity: strength={:.3}", strength);
                        }
                        Some("HormonalBurst") => {
                            let hormone = json_msg["data"]["hormone"].as_str().unwrap_or("unknown");
                            let level = json_msg["data"]["level"].as_f64().unwrap_or(0.0);
                            println!("💊 Hormonal burst: {} level={:.3}", hormone, level);
                        }
                        Some("AgentUpdate") => {
                            let agent_id = json_msg["data"]["agent_id"].as_str().unwrap_or("unknown");
                            let state = json_msg["data"]["state"].as_str().unwrap_or("unknown");
                            println!("🤖 Agent update: {} -> {}", agent_id, state);
                        }
                        Some("TaskProgress") => {
                            let progress = json_msg["data"]["progress"].as_f64().unwrap_or(0.0);
                            println!("📊 Task progress: {:.0}%", progress * 100.0);
                        }
                        _ => println!("📨 Message: {}", text),
                    }
                } else {
                    println!("📨 Raw message: {}", text);
                }
            }
            Message::Close(_) => {
                println!("Connection closed");
                break;
            }
            _ => {}
        }
    }
    
    write_handle.abort();
    Ok(())
}