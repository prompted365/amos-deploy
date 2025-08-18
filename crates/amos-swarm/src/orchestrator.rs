use crate::{
    task::{Task, TaskResult, TaskStatus, TaskStrategy, TaskOutput, TaskMetadata, AgentContribution, WorkItem, NeuralActivityMetrics},
    topology::{SwarmTopology, AgentPlacement},
};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;
use std::collections::HashMap;
use amos_core::neural::ForgeNeuralNetwork;
use amos_agents::CognitiveAgent;
use tracing::{info, debug, error};

/// Configuration for the swarm orchestrator
#[derive(Debug, Clone)]
pub struct SwarmConfig {
    pub max_concurrent_tasks: usize,
    pub task_retry_attempts: usize,
    pub coordination_interval_ms: u64,
    pub neural_sync_enabled: bool,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 10,
            task_retry_attempts: 3,
            coordination_interval_ms: 100,
            neural_sync_enabled: true,
        }
    }
}

/// Orchestrates task execution across the swarm
pub struct SwarmOrchestrator {
    topology: SwarmTopology,
    neural_network: Arc<ForgeNeuralNetwork>,
    config: SwarmConfig,
    agent_placements: Arc<RwLock<HashMap<Uuid, AgentPlacement>>>,
    active_tasks: Arc<RwLock<HashMap<Uuid, TaskExecution>>>,
    coordination_tx: mpsc::Sender<CoordinationMessage>,
    coordination_rx: Arc<RwLock<mpsc::Receiver<CoordinationMessage>>>,
}

struct TaskExecution {
    task: Task,
    strategy: TaskStrategy,
    assigned_agents: Vec<Uuid>,
    start_time: chrono::DateTime<chrono::Utc>,
    progress: f64,
}

enum CoordinationMessage {
    AgentProgress { agent_id: Uuid, task_id: Uuid, progress: f64 },
    AgentResult { agent_id: Uuid, task_id: Uuid, result: WorkItem },
    TaskComplete { task_id: Uuid },
    NeuralSync { pathway_updates: Vec<(Uuid, Uuid, f64)> },
}

impl SwarmOrchestrator {
    pub fn new(
        topology: SwarmTopology,
        neural_network: Arc<ForgeNeuralNetwork>,
    ) -> Self {
        let (tx, rx) = mpsc::channel(1000);
        
        Self {
            topology,
            neural_network,
            config: SwarmConfig::default(),
            agent_placements: Arc::new(RwLock::new(HashMap::new())),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
            coordination_tx: tx,
            coordination_rx: Arc::new(RwLock::new(rx)),
        }
    }
    
    pub fn with_config(mut self, config: SwarmConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Called when an agent joins the swarm
    pub async fn on_agent_joined(&self, agent_id: Uuid) {
        let mut placements = self.agent_placements.write().await;
        let placement = self.topology.calculate_placement(&placements);
        
        // Update existing agent placements
        for (existing_id, existing_placement) in placements.iter_mut() {
            existing_placement.on_agent_joined(agent_id, &placement);
        }
        
        placements.insert(agent_id, placement);
        
        info!("Agent {} joined swarm with {:?} topology", agent_id, self.topology);
    }
    
    /// Called when an agent leaves the swarm
    pub async fn on_agent_left(&self, agent_id: Uuid) {
        let mut placements = self.agent_placements.write().await;
        placements.remove(&agent_id);
        
        // Update remaining agent placements
        for placement in placements.values_mut() {
            placement.on_agent_left(agent_id);
        }
        
        info!("Agent {} left swarm", agent_id);
    }
    
    /// Execute a task across the swarm
    pub async fn execute_task(
        &self,
        task: Task,
        strategy: TaskStrategy,
        agents: HashMap<Uuid, Arc<dyn CognitiveAgent>>,
    ) -> Result<TaskResult, String> {
        info!("Executing task {} with {:?} strategy", task.id, strategy);
        
        // Select agents based on strategy and requirements
        let selected_agents = self.select_agents(&task, &strategy, &agents).await?;
        
        if selected_agents.len() < task.requirements.min_agents {
            return Err(format!(
                "Not enough agents available. Required: {}, Available: {}",
                task.requirements.min_agents,
                selected_agents.len()
            ));
        }
        
        // Create task execution record
        let execution = TaskExecution {
            task: task.clone(),
            strategy: strategy.clone(),
            assigned_agents: selected_agents.clone(),
            start_time: chrono::Utc::now(),
            progress: 0.0,
        };
        
        self.active_tasks.write().await.insert(task.id, execution);
        
        // Clone task_id before moving task in match arms
        let task_id = task.id;
        
        // Execute based on strategy
        let result = match strategy {
            TaskStrategy::Parallel => {
                self.execute_parallel(task, selected_agents, agents).await
            }
            TaskStrategy::Sequential => {
                self.execute_sequential(task, selected_agents, agents).await
            }
            TaskStrategy::Consensus { min_agreement } => {
                self.execute_consensus(task, selected_agents, agents, min_agreement).await
            }
            TaskStrategy::Distributed { max_subtasks } => {
                self.execute_distributed(task, selected_agents, agents, max_subtasks).await
            }
            TaskStrategy::Competitive => {
                self.execute_competitive(task, selected_agents, agents).await
            }
            TaskStrategy::Adaptive => {
                self.execute_adaptive(task, selected_agents, agents).await
            }
        };
        
        // Clean up
        self.active_tasks.write().await.remove(&task_id);
        
        result
    }
    
    /// Select agents for task execution
    async fn select_agents(
        &self,
        task: &Task,
        strategy: &TaskStrategy,
        available_agents: &HashMap<Uuid, Arc<dyn CognitiveAgent>>,
    ) -> Result<Vec<Uuid>, String> {
        let mut selected = Vec::new();
        
        // Filter by required capabilities
        let capable_agents: Vec<(Uuid, &Arc<dyn CognitiveAgent>)> = available_agents
            .iter()
            .filter(|(_, agent)| {
                // In production, check agent capabilities against requirements
                true // Placeholder
            })
            .map(|(id, agent)| (*id, agent))
            .collect();
        
        // Select based on strategy
        match strategy {
            TaskStrategy::Parallel | TaskStrategy::Competitive => {
                // Use all capable agents up to max
                let max = task.requirements.max_agents.unwrap_or(capable_agents.len());
                selected = capable_agents
                    .into_iter()
                    .take(max)
                    .map(|(id, _)| id)
                    .collect();
            }
            TaskStrategy::Sequential => {
                // Select agents in topology order
                let placements = self.agent_placements.read().await;
                selected = self.order_by_topology(capable_agents, &placements);
            }
            TaskStrategy::Consensus { .. } => {
                // Need odd number for voting
                let count = task.requirements.max_agents.unwrap_or(5).min(capable_agents.len());
                let count = if count % 2 == 0 { count - 1 } else { count };
                selected = capable_agents
                    .into_iter()
                    .take(count)
                    .map(|(id, _)| id)
                    .collect();
            }
            _ => {
                // Default selection
                let max_agents = task.requirements.max_agents.unwrap_or(capable_agents.len());
                selected = capable_agents
                    .into_iter()
                    .take(max_agents)
                    .map(|(id, _)| id)
                    .collect();
            }
        }
        
        Ok(selected)
    }
    
    /// Order agents by topology placement
    fn order_by_topology(
        &self,
        agents: Vec<(Uuid, &Arc<dyn CognitiveAgent>)>,
        placements: &HashMap<Uuid, AgentPlacement>,
    ) -> Vec<Uuid> {
        let mut ordered = Vec::new();
        
        match &self.topology {
            SwarmTopology::Hierarchical { .. } => {
                // Order by level
                let mut by_level: Vec<(usize, Uuid)> = agents
                    .iter()
                    .filter_map(|(id, _)| {
                        placements.get(id).and_then(|p| {
                            if let AgentPlacement::Hierarchical { level, .. } = p {
                                Some((*level, *id))
                            } else {
                                None
                            }
                        })
                    })
                    .collect();
                
                by_level.sort_by_key(|(level, _)| *level);
                ordered = by_level.into_iter().map(|(_, id)| id).collect();
            }
            SwarmTopology::Ring => {
                // Follow ring order
                if let Some((start_id, _)) = agents.first() {
                    ordered.push(*start_id);
                    let mut current = *start_id;
                    
                    while ordered.len() < agents.len() {
                        if let Some(placement) = placements.get(&current) {
                            if let AgentPlacement::Ring { next, .. } = placement {
                                if let Some(next_id) = next {
                                    if !ordered.contains(next_id) {
                                        ordered.push(*next_id);
                                        current = *next_id;
                                        continue;
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
            _ => {
                // Default order
                ordered = agents.into_iter().map(|(id, _)| id).collect();
            }
        }
        
        ordered
    }
    
    /// Execute task in parallel across all assigned agents
    async fn execute_parallel(
        &self,
        task: Task,
        agent_ids: Vec<Uuid>,
        agents: HashMap<Uuid, Arc<dyn CognitiveAgent>>,
    ) -> Result<TaskResult, String> {
        debug!("Executing task {} in parallel with {} agents", task.id, agent_ids.len());
        
        let mut handles = Vec::new();
        let start_time = chrono::Utc::now();
        
        // Spawn parallel tasks
        for agent_id in &agent_ids {
            if let Some(agent) = agents.get(agent_id) {
                let agent = agent.clone();
                let task_clone = task.clone();
                let neural_network = self.neural_network.clone();
                
                let handle = tokio::spawn(async move {
                    // Simulate agent processing
                    // In production, call actual agent process method
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    
                    WorkItem {
                        description: format!("Processed by {}", agent.name()),
                        result: Some(serde_json::json!({
                            "agent": agent.name(),
                            "confidence": 0.85,
                        })),
                        timestamp: chrono::Utc::now(),
                    }
                });
                
                handles.push((agent_id, handle));
            }
        }
        
        // Collect results
        let mut agent_contributions = HashMap::new();
        let mut all_results = Vec::new();
        
        for (agent_id, handle) in handles {
            match handle.await {
                Ok(work_item) => {
                    all_results.push(work_item.clone());
                    
                    let contribution = AgentContribution {
                        agent_id: *agent_id,
                        agent_type: agents.get(agent_id)
                            .map(|a| a.name().to_string())
                            .unwrap_or_default(),
                        work_items: vec![work_item],
                        confidence: 0.85,
                        neural_impact: 0.1,
                    };
                    
                    agent_contributions.insert(*agent_id, contribution);
                }
                Err(e) => {
                    error!("Agent {} failed: {}", agent_id, e);
                }
            }
        }
        
        let end_time = chrono::Utc::now();
        let duration_ms = (end_time - start_time).num_milliseconds() as u64;
        
        Ok(TaskResult {
            task_id: task.id,
            status: TaskStatus::Completed,
            output: Some(TaskOutput::Multiple(
                all_results.into_iter()
                    .filter_map(|w| w.result.map(|r| TaskOutput::Text(r.to_string())))
                    .collect()
            )),
            metadata: TaskMetadata {
                start_time,
                end_time: Some(end_time),
                duration_ms: Some(duration_ms),
                iterations: 1,
                neural_activity: NeuralActivityMetrics::default(),
            },
            agent_contributions,
        })
    }
    
    /// Execute task sequentially through assigned agents
    async fn execute_sequential(
        &self,
        task: Task,
        agent_ids: Vec<Uuid>,
        agents: HashMap<Uuid, Arc<dyn CognitiveAgent>>,
    ) -> Result<TaskResult, String> {
        debug!("Executing task {} sequentially through {} agents", task.id, agent_ids.len());
        
        let start_time = chrono::Utc::now();
        let mut agent_contributions = HashMap::new();
        let mut current_result = None;
        
        for agent_id in &agent_ids {
            if let Some(agent) = agents.get(agent_id) {
                // Process with current result as input
                let work_item = WorkItem {
                    description: format!("Sequential processing by {}", agent.name()),
                    result: Some(serde_json::json!({
                        "agent": agent.name(),
                        "input": current_result,
                        "output": format!("Processed by {}", agent.name()),
                    })),
                    timestamp: chrono::Utc::now(),
                };
                
                current_result = work_item.result.clone();
                
                let contribution = AgentContribution {
                    agent_id: *agent_id,
                    agent_type: agent.name().to_string(),
                    work_items: vec![work_item],
                    confidence: 0.9,
                    neural_impact: 0.15,
                };
                
                agent_contributions.insert(*agent_id, contribution);
            }
        }
        
        let end_time = chrono::Utc::now();
        
        Ok(TaskResult {
            task_id: task.id,
            status: TaskStatus::Completed,
            output: current_result.map(|r| TaskOutput::Text(r.to_string())),
            metadata: TaskMetadata {
                start_time,
                end_time: Some(end_time),
                duration_ms: Some((end_time - start_time).num_milliseconds() as u64),
                iterations: agent_ids.len(),
                neural_activity: NeuralActivityMetrics::default(),
            },
            agent_contributions,
        })
    }
    
    /// Execute with consensus voting
    async fn execute_consensus(
        &self,
        task: Task,
        agent_ids: Vec<Uuid>,
        agents: HashMap<Uuid, Arc<dyn CognitiveAgent>>,
        min_agreement: f64,
    ) -> Result<TaskResult, String> {
        // Similar to parallel but with voting mechanism
        self.execute_parallel(task, agent_ids, agents).await
    }
    
    /// Execute by distributing subtasks
    async fn execute_distributed(
        &self,
        task: Task,
        agent_ids: Vec<Uuid>,
        agents: HashMap<Uuid, Arc<dyn CognitiveAgent>>,
        max_subtasks: usize,
    ) -> Result<TaskResult, String> {
        // Break task into subtasks and distribute
        self.execute_parallel(task, agent_ids, agents).await
    }
    
    /// Execute competitively - best result wins
    async fn execute_competitive(
        &self,
        task: Task,
        agent_ids: Vec<Uuid>,
        agents: HashMap<Uuid, Arc<dyn CognitiveAgent>>,
    ) -> Result<TaskResult, String> {
        // Similar to parallel but select best result
        self.execute_parallel(task, agent_ids, agents).await
    }
    
    /// Adaptive execution - adjust strategy based on progress
    async fn execute_adaptive(
        &self,
        task: Task,
        agent_ids: Vec<Uuid>,
        agents: HashMap<Uuid, Arc<dyn CognitiveAgent>>,
    ) -> Result<TaskResult, String> {
        // Start with parallel, adapt if needed
        self.execute_parallel(task, agent_ids, agents).await
    }
    
    /// Get count of active tasks
    pub async fn active_task_count(&self) -> usize {
        self.active_tasks.read().await.len()
    }
}