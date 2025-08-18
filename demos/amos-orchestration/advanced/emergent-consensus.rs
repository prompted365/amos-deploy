use amos_agents::{
    traffic_seer::TrafficSeer,
    pathway_sculptor::PathwaySculptor,
    memory_weaver::MemoryWeaver,
    cognition_alchemist::CognitionAlchemist,
    learning_oracle::LearningOracle,
    mesh_harmonizer::MeshHarmonizer,
    consciousness_emergent::ConsciousnessEmergent,
};
use amos_core::{ForgeNeuralNetwork, EventBus, SystemEvent, HormonalState};
use amos_swarm::{AmosSwarm, SwarmTopology, Task, TaskStrategy};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;
use tokio::time::{sleep, Duration};
use rand::Rng;
use dashmap::DashMap;

// Consensus tracking structure
#[derive(Debug, Clone)]
struct ConsensusState {
    proposals: HashMap<String, f64>,
    votes: HashMap<uuid::Uuid, HashMap<String, f64>>,
    convergence_threshold: f64,
    rounds: usize,
}

impl ConsensusState {
    fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            votes: HashMap::new(),
            convergence_threshold: 0.8,
            rounds: 0,
        }
    }

    fn add_vote(&mut self, agent_id: uuid::Uuid, proposal: String, confidence: f64) {
        self.votes.entry(agent_id)
            .or_insert_with(HashMap::new)
            .insert(proposal, confidence);
    }

    fn calculate_consensus(&self) -> Option<(String, f64)> {
        let mut proposal_scores: HashMap<String, f64> = HashMap::new();
        
        // Aggregate votes
        for agent_votes in self.votes.values() {
            for (proposal, confidence) in agent_votes {
                *proposal_scores.entry(proposal.clone()).or_insert(0.0) += confidence;
            }
        }
        
        // Normalize by number of agents
        let agent_count = self.votes.len() as f64;
        for score in proposal_scores.values_mut() {
            *score /= agent_count;
        }
        
        // Find highest scoring proposal
        proposal_scores.into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .filter(|(_, score)| *score >= self.convergence_threshold)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🧠 Starting AMOS Emergent Consensus Demo");
    info!("📊 This demo shows how multiple agents reach consensus through neural synchronization");

    // Create shared neural network with enhanced capacity
    let mut neural_network = ForgeNeuralNetwork::new();
    neural_network.add_neurons(2000); // More neurons for complex decision making
    let neural_network = Arc::new(neural_network);
    
    let event_bus = Arc::new(EventBus::new());
    
    // Create swarm with mesh topology for peer-to-peer consensus
    let swarm = AmosSwarm::new(
        "Consensus Swarm".to_string(),
        SwarmTopology::Mesh { max_connections: 10 },
        neural_network.clone(),
    );
    
    info!("🔗 Created mesh swarm for distributed consensus");

    // Shared consensus state
    let consensus_state = Arc::new(tokio::sync::RwLock::new(ConsensusState::new()));
    
    // Spawn diverse agents for different perspectives
    info!("🚀 Spawning diverse cognitive agents...");
    
    let mut agents = Vec::new();
    
    // Traffic Seer - Pattern analysis perspective
    let mut traffic_seer = TrafficSeer::new();
    traffic_seer.initialize(neural_network.clone(), event_bus.clone()).await?;
    traffic_seer.activate().await?;
    let traffic_id = swarm.spawn_agent(Arc::new(traffic_seer)).await?;
    agents.push(("Traffic Seer", traffic_id));
    
    // Memory Weaver - Historical perspective
    let mut memory_weaver = MemoryWeaver::new();
    memory_weaver.initialize(neural_network.clone(), event_bus.clone()).await?;
    memory_weaver.activate().await?;
    let memory_id = swarm.spawn_agent(Arc::new(memory_weaver)).await?;
    agents.push(("Memory Weaver", memory_id));
    
    // Learning Oracle - Predictive perspective
    let mut learning_oracle = LearningOracle::new();
    learning_oracle.initialize(neural_network.clone(), event_bus.clone()).await?;
    learning_oracle.activate().await?;
    let learning_id = swarm.spawn_agent(Arc::new(learning_oracle)).await?;
    agents.push(("Learning Oracle", learning_id));
    
    // Cognition Alchemist - Creative perspective
    let mut cognition_alchemist = CognitionAlchemist::new();
    cognition_alchemist.initialize(neural_network.clone(), event_bus.clone()).await?;
    cognition_alchemist.activate().await?;
    let cognition_id = swarm.spawn_agent(Arc::new(cognition_alchemist)).await?;
    agents.push(("Cognition Alchemist", cognition_id));
    
    // Consciousness Emergent - Meta-cognitive perspective
    let mut consciousness = ConsciousnessEmergent::new();
    consciousness.initialize(neural_network.clone(), event_bus.clone()).await?;
    consciousness.activate().await?;
    let consciousness_id = swarm.spawn_agent(Arc::new(consciousness)).await?;
    agents.push(("Consciousness Emergent", consciousness_id));
    
    // Mesh Harmonizer - Coordination perspective
    let mut harmonizer = MeshHarmonizer::new();
    harmonizer.initialize(neural_network.clone(), event_bus.clone()).await?;
    harmonizer.activate().await?;
    let harmonizer_id = swarm.spawn_agent(Arc::new(harmonizer)).await?;
    agents.push(("Mesh Harmonizer", harmonizer_id));

    info!("✅ Spawned {} diverse agents", agents.len());

    // Define decision scenarios
    let scenarios = vec![
        (
            "Resource Allocation",
            vec![
                "Optimize for efficiency",
                "Maximize redundancy",
                "Balance load equally",
                "Prioritize critical paths"
            ]
        ),
        (
            "Neural Architecture",
            vec![
                "Deep hierarchical layers",
                "Flat distributed mesh",
                "Hybrid adaptive topology",
                "Dynamic reconfigurable"
            ]
        ),
        (
            "Learning Strategy",
            vec![
                "Supervised training",
                "Reinforcement learning",
                "Unsupervised clustering",
                "Meta-learning approach"
            ]
        ),
    ];

    // Process each scenario
    for (scenario_name, proposals) in scenarios {
        info!("🎯 Scenario: {}", scenario_name);
        info!("📋 Proposals: {:?}", proposals);
        
        // Reset consensus state
        {
            let mut state = consensus_state.write().await;
            *state = ConsensusState::new();
            for proposal in &proposals {
                state.proposals.insert(proposal.to_string(), 0.0);
            }
        }
        
        // Create consensus task
        let mut metadata = HashMap::new();
        metadata.insert("scenario".to_string(), scenario_name.to_string());
        metadata.insert("proposal_count".to_string(), proposals.len().to_string());
        
        let consensus_task = Task {
            id: uuid::Uuid::new_v4(),
            name: format!("{} Consensus", scenario_name),
            description: "Reach distributed consensus through neural synchronization".to_string(),
            priority: 0.9,
            metadata,
        };
        
        // Trigger consensus building
        event_bus.publish(SystemEvent::TaskScheduled {
            task_id: consensus_task.id,
            priority: consensus_task.priority,
        }).await;
        
        // Simulate multiple rounds of voting
        let mut consensus_reached = false;
        let max_rounds = 5;
        
        for round in 1..=max_rounds {
            info!("🔄 Consensus Round {}/{}", round, max_rounds);
            
            // Each agent evaluates proposals
            for (agent_name, agent_id) in &agents {
                let mut rng = rand::thread_rng();
                
                // Simulate agent decision making with neural influence
                let neural_activity = neural_network.get_average_activity();
                
                for proposal in &proposals {
                    // Each agent has different evaluation criteria
                    let base_score = match agent_name {
                        &"Traffic Seer" => {
                            // Prefers efficiency
                            if proposal.contains("efficiency") || proposal.contains("Optimize") {
                                0.8
                            } else {
                                rng.gen_range(0.3..0.6)
                            }
                        },
                        &"Memory Weaver" => {
                            // Values stability and redundancy
                            if proposal.contains("redundancy") || proposal.contains("hierarchical") {
                                0.75
                            } else {
                                rng.gen_range(0.4..0.6)
                            }
                        },
                        &"Learning Oracle" => {
                            // Prefers adaptive approaches
                            if proposal.contains("adaptive") || proposal.contains("Meta") {
                                0.85
                            } else {
                                rng.gen_range(0.3..0.5)
                            }
                        },
                        &"Cognition Alchemist" => {
                            // Likes creative solutions
                            if proposal.contains("Hybrid") || proposal.contains("Dynamic") {
                                0.8
                            } else {
                                rng.gen_range(0.4..0.7)
                            }
                        },
                        &"Consciousness Emergent" => {
                            // Meta-cognitive preference
                            if proposal.contains("Meta") || proposal.contains("Unsupervised") {
                                0.9
                            } else {
                                rng.gen_range(0.3..0.6)
                            }
                        },
                        _ => rng.gen_range(0.4..0.6)
                    };
                    
                    // Adjust score based on neural synchronization
                    let synchronized_score = base_score * (0.7 + neural_activity * 0.3);
                    
                    // Add vote
                    consensus_state.write().await.add_vote(
                        *agent_id,
                        proposal.to_string(),
                        synchronized_score
                    );
                }
            }
            
            // Check for consensus
            let state = consensus_state.read().await;
            if let Some((winning_proposal, confidence)) = state.calculate_consensus() {
                info!("✅ Consensus reached! Proposal: '{}' with {:.2}% agreement", 
                      winning_proposal, confidence * 100.0);
                consensus_reached = true;
                
                // Trigger neural synchronization event
                event_bus.publish(SystemEvent::NeuralSynchronization {
                    sync_level: confidence,
                    participating_agents: agents.len(),
                }).await;
                
                break;
            } else {
                info!("❌ No consensus yet. Continuing deliberation...");
                
                // Show current standings
                let mut proposal_scores: HashMap<String, f64> = HashMap::new();
                for agent_votes in state.votes.values() {
                    for (proposal, confidence) in agent_votes {
                        *proposal_scores.entry(proposal.clone()).or_insert(0.0) += confidence;
                    }
                }
                
                let agent_count = state.votes.len() as f64;
                let mut sorted_proposals: Vec<_> = proposal_scores.into_iter()
                    .map(|(p, s)| (p, s / agent_count))
                    .collect();
                sorted_proposals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                
                for (proposal, score) in sorted_proposals.iter().take(3) {
                    info!("   {} - {:.2}%", proposal, score * 100.0);
                }
            }
            
            // Neural adaptation between rounds
            sleep(Duration::from_millis(500)).await;
            
            // Stimulate neural network for better synchronization
            neural_network.stimulate_region("consensus_cortex", round as f64 * 0.2);
        }
        
        if !consensus_reached {
            warn!("⚠️ Failed to reach consensus after {} rounds", max_rounds);
        }
        
        info!("📊 Scenario '{}' completed\n", scenario_name);
        sleep(Duration::from_secs(1)).await;
    }

    // Demonstrate emergent behavior
    info!("🌟 Demonstrating emergent consensus properties...");
    
    // Create a complex multi-faceted decision
    let complex_task = Task {
        id: uuid::Uuid::new_v4(),
        name: "System Evolution Strategy".to_string(),
        description: "Determine optimal evolution path for the entire system".to_string(),
        priority: 1.0,
        metadata: HashMap::new(),
    };
    
    info!("🎯 Complex decision: {}", complex_task.name);
    
    // Execute with adaptive strategy to see emergent consensus
    match swarm.orchestrate(complex_task, TaskStrategy::Adaptive).await {
        Ok(result) => {
            info!("✨ Emergent consensus achieved!");
            info!("   Agents involved: {}", result.agents_involved.len());
            info!("   Time to consensus: {:?}", result.duration);
            info!("   Neural synchronization level: {:.2}%", 
                  neural_network.get_average_activity() * 100.0);
        }
        Err(e) => {
            warn!("❌ Complex consensus failed: {}", e);
        }
    }
    
    // Final analysis
    let final_status = swarm.status().await;
    info!("📊 Final Swarm Analysis:");
    info!("   Total agents: {}", final_status.agent_count);
    info!("   Swarm health: {:.2}%", final_status.health * 100.0);
    info!("   Neural coherence: {:.2}%", neural_network.get_coherence() * 100.0);
    
    info!("🎉 Emergent Consensus Demo Complete!");
    info!("💡 Key Insights:");
    info!("   - Agents with diverse perspectives can reach consensus");
    info!("   - Neural synchronization facilitates agreement");
    info!("   - Emergent behavior arises from simple voting rules");
    info!("   - System adapts and improves consensus efficiency over time");

    Ok(())
}