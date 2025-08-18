use amos_agents::{traffic_seer::TrafficSeer, memory_weaver::MemoryWeaver, learning_oracle::LearningOracle};
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent};
use amos_swarm::{AmosSwarm, SwarmTopology, Task, TaskStrategy};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🐝 Starting AMOS Hello Swarm Demo");

    // Create the neural network
    let neural_network = Arc::new(ForgeNeuralNetwork::new());
    info!("🧠 Neural network initialized");

    // Create event bus
    let event_bus = Arc::new(EventBus::new());
    
    // Create a swarm with mesh topology
    let swarm = AmosSwarm::new(
        "Hello Swarm".to_string(),
        SwarmTopology::Mesh { max_connections: 6 },
        neural_network.clone(),
    );
    info!("🔗 Swarm created with mesh topology");

    // Spawn different types of agents
    info!("🚀 Spawning agents...");
    
    // Create and spawn Traffic Seer
    let mut traffic_seer = TrafficSeer::new();
    traffic_seer.initialize(neural_network.clone(), event_bus.clone()).await?;
    traffic_seer.activate().await?;
    let traffic_id = swarm.spawn_agent(Arc::new(traffic_seer)).await?;
    info!("✅ Traffic Seer spawned: {}", traffic_id);

    // Create and spawn Memory Weaver
    let mut memory_weaver = MemoryWeaver::new();
    memory_weaver.initialize(neural_network.clone(), event_bus.clone()).await?;
    memory_weaver.activate().await?;
    let memory_id = swarm.spawn_agent(Arc::new(memory_weaver)).await?;
    info!("✅ Memory Weaver spawned: {}", memory_id);

    // Create and spawn Learning Oracle
    let mut learning_oracle = LearningOracle::new();
    learning_oracle.initialize(neural_network.clone(), event_bus.clone()).await?;
    learning_oracle.activate().await?;
    let learning_id = swarm.spawn_agent(Arc::new(learning_oracle)).await?;
    info!("✅ Learning Oracle spawned: {}", learning_id);

    // Get swarm status
    let status = swarm.status().await;
    info!("📊 Swarm Status:");
    info!("   Name: {}", status.name);
    info!("   Agents: {}", status.agent_count);
    info!("   Health: {:.2}%", status.health * 100.0);
    info!("   Topology: {:?}", status.topology);

    // Create a simple task
    let task = Task {
        id: uuid::Uuid::new_v4(),
        name: "Hello World Analysis".to_string(),
        description: "Analyze neural patterns and optimize pathways".to_string(),
        priority: 1.0,
        metadata: std::collections::HashMap::new(),
    };

    info!("🎯 Orchestrating task: {}", task.name);
    
    // Execute task with parallel strategy
    match swarm.orchestrate(task, TaskStrategy::Parallel).await {
        Ok(result) => {
            info!("✨ Task completed successfully!");
            info!("   Duration: {:?}", result.duration);
            info!("   Agents involved: {}", result.agents_involved.len());
        }
        Err(e) => {
            info!("❌ Task failed: {}", e);
        }
    }

    // Simulate some neural activity
    info!("🔄 Simulating neural activity...");
    event_bus.publish(SystemEvent::NeuralActivityDetected {
        region: "cortex".to_string(),
        intensity: 0.8,
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Final status
    let final_status = swarm.status().await;
    info!("📊 Final Swarm Status:");
    info!("   Active tasks: {}", final_status.active_tasks);
    info!("   Health: {:.2}%", final_status.health * 100.0);

    info!("🎉 Hello Swarm demo completed!");

    Ok(())
}