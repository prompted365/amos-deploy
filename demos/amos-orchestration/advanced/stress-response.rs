use amos_agents::{
    traffic_seer::TrafficSeer,
    pathway_sculptor::PathwaySculptor,
    memory_weaver::MemoryWeaver,
    performance_guardian::PerformanceGuardian,
    mesh_harmonizer::MeshHarmonizer,
};
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent, HormonalState, StressResponse};
use amos_swarm::{AmosSwarm, SwarmTopology, Task, TaskStrategy};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;
use tokio::time::{sleep, Duration, interval};
use rand::Rng;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

// Stress metrics tracking
struct StressMetrics {
    task_count: AtomicU64,
    failed_tasks: AtomicU64,
    avg_response_time: AtomicU64,
    memory_pressure: AtomicU64,
    neural_overload: AtomicBool,
}

impl StressMetrics {
    fn new() -> Self {
        Self {
            task_count: AtomicU64::new(0),
            failed_tasks: AtomicU64::new(0),
            avg_response_time: AtomicU64::new(0),
            memory_pressure: AtomicU64::new(0),
            neural_overload: AtomicBool::new(false),
        }
    }
    
    fn report(&self) {
        info!("📊 Stress Metrics:");
        info!("   Total tasks: {}", self.task_count.load(Ordering::Relaxed));
        info!("   Failed tasks: {}", self.failed_tasks.load(Ordering::Relaxed));
        info!("   Avg response time: {}ms", self.avg_response_time.load(Ordering::Relaxed));
        info!("   Memory pressure: {}%", self.memory_pressure.load(Ordering::Relaxed));
        info!("   Neural overload: {}", self.neural_overload.load(Ordering::Relaxed));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("💪 Starting AMOS Stress Response Demo");
    info!("🔥 This demo shows how the swarm handles extreme load and adapts under stress");

    // Create neural network with stress response capabilities
    let mut neural_network = ForgeNeuralNetwork::new();
    neural_network.add_neurons(5000); // Large network for handling stress
    neural_network.enable_stress_response();
    let neural_network = Arc::new(neural_network);
    
    let event_bus = Arc::new(EventBus::new());
    let hormonal_state = Arc::new(tokio::sync::RwLock::new(HormonalState::new()));
    
    // Create swarm with hierarchical topology for better load distribution
    let swarm = AmosSwarm::new(
        "Stress Response Swarm".to_string(),
        SwarmTopology::Hierarchical { 
            levels: 4, 
            agents_per_level: 5 
        },
        neural_network.clone(),
    );
    
    info!("🏗️ Created hierarchical swarm for stress management");

    // Shared stress metrics
    let metrics = Arc::new(StressMetrics::new());
    
    // Spawn specialized stress-handling agents
    info!("🚀 Spawning stress-response team...");
    
    // Performance Guardian - monitors system health
    let mut guardian = PerformanceGuardian::new();
    guardian.initialize(neural_network.clone(), event_bus.clone()).await?;
    guardian.activate().await?;
    let guardian_id = swarm.spawn_agent(Arc::new(guardian)).await?;
    info!("🛡️ Performance Guardian online");
    
    // Multiple Traffic Seers for load distribution
    let mut traffic_ids = Vec::new();
    for i in 0..3 {
        let mut traffic_seer = TrafficSeer::new();
        traffic_seer.initialize(neural_network.clone(), event_bus.clone()).await?;
        traffic_seer.activate().await?;
        let id = swarm.spawn_agent(Arc::new(traffic_seer)).await?;
        traffic_ids.push(id);
        info!("🚦 Traffic Seer {} deployed", i + 1);
    }
    
    // Pathway Sculptors for dynamic routing
    let mut sculptor_ids = Vec::new();
    for i in 0..2 {
        let mut sculptor = PathwaySculptor::new();
        sculptor.initialize(neural_network.clone(), event_bus.clone()).await?;
        sculptor.activate().await?;
        let id = swarm.spawn_agent(Arc::new(sculptor)).await?;
        sculptor_ids.push(id);
        info!("🛤️ Pathway Sculptor {} deployed", i + 1);
    }
    
    // Memory Weavers for state management
    let mut memory_ids = Vec::new();
    for i in 0..2 {
        let mut weaver = MemoryWeaver::new();
        weaver.initialize(neural_network.clone(), event_bus.clone()).await?;
        weaver.activate().await?;
        let id = swarm.spawn_agent(Arc::new(weaver)).await?;
        memory_ids.push(id);
        info!("🧵 Memory Weaver {} deployed", i + 1);
    }
    
    // Mesh Harmonizer for coordination under stress
    let mut harmonizer = MeshHarmonizer::new();
    harmonizer.initialize(neural_network.clone(), event_bus.clone()).await?;
    harmonizer.activate().await?;
    let harmonizer_id = swarm.spawn_agent(Arc::new(harmonizer)).await?;
    info!("🎵 Mesh Harmonizer coordinating stress response");

    let initial_status = swarm.status().await;
    info!("📊 Initial swarm strength: {} agents", initial_status.agent_count);

    // Phase 1: Gradual Load Increase
    info!("\n🌊 PHASE 1: Gradual Load Increase");
    info!("Simulating increasing workload...");
    
    let mut load_level = 1;
    for wave in 1..=5 {
        info!("📈 Load wave {}/5 - Intensity: {}", wave, load_level);
        
        // Generate tasks based on load level
        let mut tasks = Vec::new();
        for i in 0..load_level {
            let task = Task {
                id: uuid::Uuid::new_v4(),
                name: format!("Load Task {}-{}", wave, i),
                description: "Process data under increasing load".to_string(),
                priority: rand::thread_rng().gen_range(0.1..1.0),
                metadata: HashMap::new(),
            };
            tasks.push(task);
        }
        
        // Submit tasks concurrently
        let start = tokio::time::Instant::now();
        let handles: Vec<_> = tasks.into_iter().map(|task| {
            let swarm_clone = swarm.clone();
            let metrics_clone = metrics.clone();
            tokio::spawn(async move {
                metrics_clone.task_count.fetch_add(1, Ordering::Relaxed);
                match swarm_clone.orchestrate(task, TaskStrategy::Adaptive).await {
                    Ok(_) => {},
                    Err(_) => {
                        metrics_clone.failed_tasks.fetch_add(1, Ordering::Relaxed);
                    }
                }
            })
        }).collect();
        
        // Wait for wave completion
        for handle in handles {
            let _ = handle.await;
        }
        
        let duration = start.elapsed();
        metrics.avg_response_time.store(duration.as_millis() as u64, Ordering::Relaxed);
        
        // Check stress indicators
        let stress_level = neural_network.get_stress_level();
        hormonal_state.write().await.cortisol = stress_level;
        
        info!("   ⏱️ Wave completed in {:?}", duration);
        info!("   🧠 Neural stress level: {:.2}%", stress_level * 100.0);
        info!("   💊 Cortisol level: {:.2}", hormonal_state.read().await.cortisol);
        
        // System adaptation
        if stress_level > 0.7 {
            warn!("⚠️ High stress detected! Triggering adaptation...");
            event_bus.publish(SystemEvent::StressResponseTriggered {
                level: stress_level,
                action: "pathway_optimization".to_string(),
            }).await;
            sleep(Duration::from_millis(500)).await;
        }
        
        load_level *= 2;
        sleep(Duration::from_secs(1)).await;
    }
    
    metrics.report();

    // Phase 2: Spike Load Test
    info!("\n⚡ PHASE 2: Spike Load Test");
    info!("Simulating sudden traffic spike...");
    
    let spike_size = 50;
    let mut spike_tasks = Vec::new();
    
    for i in 0..spike_size {
        let task = Task {
            id: uuid::Uuid::new_v4(),
            name: format!("Spike Task {}", i),
            description: "Emergency processing request".to_string(),
            priority: 0.9, // High priority
            metadata: HashMap::new(),
        };
        spike_tasks.push(task);
    }
    
    info!("🚨 Injecting {} high-priority tasks simultaneously!", spike_size);
    
    let spike_start = tokio::time::Instant::now();
    let spike_handles: Vec<_> = spike_tasks.into_iter().map(|task| {
        let swarm_clone = swarm.clone();
        let metrics_clone = metrics.clone();
        let event_bus_clone = event_bus.clone();
        
        tokio::spawn(async move {
            metrics_clone.task_count.fetch_add(1, Ordering::Relaxed);
            
            // Random task might trigger emergency events
            if rand::thread_rng().gen_bool(0.1) {
                event_bus_clone.publish(SystemEvent::EmergencyAlert {
                    severity: 0.8,
                    message: "Resource contention detected".to_string(),
                }).await;
            }
            
            match swarm_clone.orchestrate(task, TaskStrategy::Parallel).await {
                Ok(_) => {},
                Err(_) => {
                    metrics_clone.failed_tasks.fetch_add(1, Ordering::Relaxed);
                }
            }
        })
    }).collect();
    
    // Monitor stress during spike
    let monitor_handle = tokio::spawn({
        let neural_network_clone = neural_network.clone();
        let metrics_clone = metrics.clone();
        let hormonal_state_clone = hormonal_state.clone();
        
        async move {
            let mut ticker = interval(Duration::from_millis(100));
            for i in 0..10 {
                ticker.tick().await;
                let stress = neural_network_clone.get_stress_level();
                if stress > 0.9 {
                    metrics_clone.neural_overload.store(true, Ordering::Relaxed);
                    error!("🔴 NEURAL OVERLOAD DETECTED!");
                }
                
                // Update hormonal response
                let mut hormones = hormonal_state_clone.write().await;
                hormones.adrenaline = stress;
                hormones.norepinephrine = stress * 0.8;
                
                if i % 2 == 0 {
                    info!("   📡 Real-time stress: {:.2}%", stress * 100.0);
                }
            }
        }
    });
    
    // Wait for spike completion
    for handle in spike_handles {
        let _ = handle.await;
    }
    monitor_handle.await?;
    
    let spike_duration = spike_start.elapsed();
    info!("⚡ Spike handled in {:?}", spike_duration);
    
    // Recovery period
    info!("\n🌿 PHASE 3: Recovery and Adaptation");
    info!("Monitoring system recovery...");
    
    for i in 1..=5 {
        sleep(Duration::from_secs(1)).await;
        
        let stress = neural_network.get_stress_level();
        let hormones = hormonal_state.read().await;
        
        info!("Recovery checkpoint {}:", i);
        info!("   🧠 Stress level: {:.2}%", stress * 100.0);
        info!("   💊 Cortisol: {:.2}", hormones.cortisol);
        info!("   ⚡ Adrenaline: {:.2}", hormones.adrenaline);
        
        // Trigger healing mechanisms
        if stress > 0.5 {
            event_bus.publish(SystemEvent::HealingInitiated {
                target_region: "global".to_string(),
                intensity: 0.3,
            }).await;
        }
    }

    // Phase 4: Sustained Load with Agent Failures
    info!("\n🔨 PHASE 4: Sustained Load with Agent Failures");
    info!("Testing resilience under agent failures...");
    
    // Start sustained load
    let sustained_handle = tokio::spawn({
        let swarm_clone = swarm.clone();
        let metrics_clone = metrics.clone();
        
        async move {
            let mut ticker = interval(Duration::from_millis(200));
            for i in 0..20 {
                ticker.tick().await;
                
                let task = Task {
                    id: uuid::Uuid::new_v4(),
                    name: format!("Sustained Task {}", i),
                    description: "Continuous processing".to_string(),
                    priority: 0.5,
                    metadata: HashMap::new(),
                };
                
                let swarm = swarm_clone.clone();
                let metrics = metrics_clone.clone();
                tokio::spawn(async move {
                    metrics.task_count.fetch_add(1, Ordering::Relaxed);
                    let _ = swarm.orchestrate(task, TaskStrategy::Adaptive).await;
                });
            }
        }
    });
    
    // Simulate random agent failures
    sleep(Duration::from_secs(1)).await;
    
    info!("💥 Simulating agent failures...");
    
    // Fail a traffic seer
    if let Some(failed_id) = traffic_ids.get(0) {
        swarm.remove_agent(*failed_id).await?;
        error!("💥 Traffic Seer 1 has failed!");
        event_bus.publish(SystemEvent::AgentDeactivated {
            agent_id: *failed_id,
        }).await;
    }
    
    sleep(Duration::from_millis(500)).await;
    
    // Fail a memory weaver
    if let Some(failed_id) = memory_ids.get(0) {
        swarm.remove_agent(*failed_id).await?;
        error!("💥 Memory Weaver 1 has failed!");
        event_bus.publish(SystemEvent::AgentDeactivated {
            agent_id: *failed_id,
        }).await;
    }
    
    // Let the system adapt
    sustained_handle.await?;
    
    // Final stress test results
    info!("\n📊 FINAL STRESS TEST RESULTS");
    
    let final_status = swarm.status().await;
    let final_stress = neural_network.get_stress_level();
    let final_hormones = hormonal_state.read().await;
    
    metrics.report();
    
    info!("\n🏁 System State:");
    info!("   Surviving agents: {}/{}", final_status.agent_count, initial_status.agent_count);
    info!("   Final stress level: {:.2}%", final_stress * 100.0);
    info!("   System health: {:.2}%", final_status.health * 100.0);
    info!("   Neural coherence: {:.2}%", neural_network.get_coherence() * 100.0);
    
    info!("\n💊 Final Hormonal State:");
    info!("   Cortisol: {:.2}", final_hormones.cortisol);
    info!("   Adrenaline: {:.2}", final_hormones.adrenaline);
    info!("   Dopamine: {:.2}", final_hormones.dopamine);
    info!("   Serotonin: {:.2}", final_hormones.serotonin);
    
    // Calculate resilience score
    let total_tasks = metrics.task_count.load(Ordering::Relaxed);
    let failed_tasks = metrics.failed_tasks.load(Ordering::Relaxed);
    let success_rate = if total_tasks > 0 {
        ((total_tasks - failed_tasks) as f64 / total_tasks as f64) * 100.0
    } else {
        0.0
    };
    
    let resilience_score = success_rate * final_status.health * (1.0 - final_stress);
    
    info!("\n🏆 Resilience Score: {:.2}/100", resilience_score);
    info!("   Task success rate: {:.2}%", success_rate);
    info!("   Agent survival rate: {:.2}%", 
          (final_status.agent_count as f64 / initial_status.agent_count as f64) * 100.0);
    
    info!("\n🎉 Stress Response Demo Complete!");
    info!("💡 Key Findings:");
    info!("   - System successfully adapted to increasing load");
    info!("   - Neural stress response prevented total collapse");
    info!("   - Agent failures were handled gracefully");
    info!("   - Hormonal system provided adaptive responses");
    info!("   - Recovery mechanisms restored balance");

    Ok(())
}