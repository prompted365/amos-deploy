use amos_agents::{
    traffic_seer::TrafficSeer, 
    pathway_sculptor::PathwaySculptor,
    memory_weaver::MemoryWeaver,
    cognition_alchemist::CognitionAlchemist,
    mesh_harmonizer::MeshHarmonizer,
};
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent};
use amos_swarm::{AmosSwarm, SwarmTopology, Task, TaskStrategy, CoordinationProtocol};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🤝 Starting AMOS Agent Coordination Demo");

    // Create shared neural network
    let neural_network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    
    info!("🧠 Neural network initialized with {} neurons", 1000);

    // Create hierarchical swarm for structured coordination
    let swarm = AmosSwarm::new(
        "Coordination Swarm".to_string(),
        SwarmTopology::Hierarchical { 
            levels: 3, 
            agents_per_level: 4 
        },
        neural_network.clone(),
    );
    info!("🏗️ Created hierarchical swarm structure");

    // Spawn coordinator (Mesh Harmonizer at top level)
    let mut harmonizer = MeshHarmonizer::new();
    harmonizer.initialize(neural_network.clone(), event_bus.clone()).await?;
    harmonizer.activate().await?;
    let harmonizer_id = swarm.spawn_agent(Arc::new(harmonizer)).await?;
    info!("👑 Mesh Harmonizer (coordinator) spawned: {}", harmonizer_id);

    // Spawn middle layer agents
    let mut traffic_seer = TrafficSeer::new();
    traffic_seer.initialize(neural_network.clone(), event_bus.clone()).await?;
    traffic_seer.activate().await?;
    let traffic_id = swarm.spawn_agent(Arc::new(traffic_seer)).await?;
    info!("🔍 Traffic Seer spawned: {}", traffic_id);

    let mut pathway_sculptor = PathwaySculptor::new();
    pathway_sculptor.initialize(neural_network.clone(), event_bus.clone()).await?;
    pathway_sculptor.activate().await?;
    let pathway_id = swarm.spawn_agent(Arc::new(pathway_sculptor)).await?;
    info!("🛤️ Pathway Sculptor spawned: {}", pathway_id);

    // Spawn worker agents
    let mut memory_weaver = MemoryWeaver::new();
    memory_weaver.initialize(neural_network.clone(), event_bus.clone()).await?;
    memory_weaver.activate().await?;
    let memory_id = swarm.spawn_agent(Arc::new(memory_weaver)).await?;
    info!("🧵 Memory Weaver spawned: {}", memory_id);

    let mut cognition_alchemist = CognitionAlchemist::new();
    cognition_alchemist.initialize(neural_network.clone(), event_bus.clone()).await?;
    cognition_alchemist.activate().await?;
    let cognition_id = swarm.spawn_agent(Arc::new(cognition_alchemist)).await?;
    info!("⚗️ Cognition Alchemist spawned: {}", cognition_id);

    // Display swarm hierarchy
    info!("📊 Swarm Hierarchy:");
    info!("   Level 1 (Coordinator): Mesh Harmonizer");
    info!("   Level 2 (Analyzers): Traffic Seer, Pathway Sculptor");
    info!("   Level 3 (Workers): Memory Weaver, Cognition Alchemist");

    // Simulate coordinated task execution
    info!("🎯 Starting coordinated task execution...");

    // Task 1: Pattern Recognition (requires coordination)
    let mut metadata = HashMap::new();
    metadata.insert("requires_coordination".to_string(), "true".to_string());
    metadata.insert("min_agents".to_string(), "3".to_string());

    let pattern_task = Task {
        id: uuid::Uuid::new_v4(),
        name: "Complex Pattern Recognition".to_string(),
        description: "Identify emergent patterns across neural regions".to_string(),
        priority: 0.9,
        metadata,
    };

    info!("📋 Task: {}", pattern_task.name);
    
    // Trigger coordination events
    event_bus.publish(SystemEvent::TaskScheduled {
        task_id: pattern_task.id,
        priority: pattern_task.priority,
    }).await;

    // Execute with adaptive strategy (agents coordinate automatically)
    let result = swarm.orchestrate(pattern_task.clone(), TaskStrategy::Adaptive).await?;
    
    info!("✅ Pattern recognition completed:");
    info!("   Agents coordinated: {}", result.agents_involved.len());
    info!("   Duration: {:?}", result.duration);
    
    // Simulate inter-agent communication
    info!("💬 Simulating agent communication...");
    
    // Traffic Seer detects anomaly
    event_bus.publish(SystemEvent::AnomalyDetected {
        agent_id: traffic_id,
        description: "Unusual neural spike pattern detected".to_string(),
        severity: 0.7,
    }).await;
    
    sleep(Duration::from_millis(500)).await;
    
    // Pathway Sculptor responds
    event_bus.publish(SystemEvent::PathwayOptimized {
        optimizer_id: pathway_id,
        improvement: 0.15,
        pathways_affected: 3,
    }).await;
    
    sleep(Duration::from_millis(500)).await;

    // Task 2: Memory Consolidation (sequential coordination)
    let memory_task = Task {
        id: uuid::Uuid::new_v4(),
        name: "Memory Consolidation".to_string(),
        description: "Coordinate memory storage across neural regions".to_string(),
        priority: 0.8,
        metadata: HashMap::new(),
    };

    info!("📋 Task: {}", memory_task.name);
    
    // Execute sequentially (agents pass results to each other)
    let memory_result = swarm.orchestrate(memory_task, TaskStrategy::Sequential).await?;
    
    info!("✅ Memory consolidation completed:");
    info!("   Sequential processing chain: {} agents", memory_result.agents_involved.len());

    // Demonstrate coordination protocol
    info!("🔄 Testing coordination protocol...");
    
    // Simulate workload distribution
    for i in 0..3 {
        let workload_task = Task {
            id: uuid::Uuid::new_v4(),
            name: format!("Workload {}", i + 1),
            description: "Distributed processing task".to_string(),
            priority: 0.5 + (i as f64 * 0.1),
            metadata: HashMap::new(),
        };
        
        event_bus.publish(SystemEvent::TaskScheduled {
            task_id: workload_task.id,
            priority: workload_task.priority,
        }).await;
    }

    sleep(Duration::from_secs(1)).await;

    // Check swarm coordination metrics
    let status = swarm.status().await;
    info!("📊 Coordination Metrics:");
    info!("   Total agents: {}", status.agent_count);
    info!("   Active tasks: {}", status.active_tasks);
    info!("   Swarm health: {:.2}%", status.health * 100.0);

    // Demonstrate agent failure and recovery
    info!("⚠️ Simulating agent failure and coordination recovery...");
    
    // Remove an agent to simulate failure
    swarm.remove_agent(memory_id).await?;
    warn!("Memory Weaver went offline!");
    
    sleep(Duration::from_millis(500)).await;
    
    // Other agents should adapt
    event_bus.publish(SystemEvent::AgentDeactivated {
        agent_id: memory_id,
    }).await;
    
    // Test coordination with reduced capacity
    let recovery_task = Task {
        id: uuid::Uuid::new_v4(),
        name: "Recovery Coordination".to_string(),
        description: "Test coordination with reduced agent capacity".to_string(),
        priority: 1.0,
        metadata: HashMap::new(),
    };
    
    match swarm.orchestrate(recovery_task, TaskStrategy::Adaptive).await {
        Ok(result) => {
            info!("✅ Swarm adapted successfully!");
            info!("   Remaining agents handled the task: {} agents", result.agents_involved.len());
        }
        Err(e) => {
            warn!("⚠️ Coordination challenge: {}", e);
        }
    }

    // Final coordination summary
    info!("🎉 Agent Coordination Demo Complete!");
    info!("📊 Coordination Summary:");
    info!("   - Demonstrated hierarchical coordination");
    info!("   - Agents communicated via event bus");
    info!("   - Adaptive task distribution");
    info!("   - Graceful handling of agent failures");

    Ok(())
}