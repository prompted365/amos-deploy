use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// A task to be executed by the swarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub input: TaskInput,
    pub requirements: TaskRequirements,
    pub priority: TaskPriority,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Task {
    pub fn new(description: String, input: TaskInput) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            input,
            requirements: TaskRequirements::default(),
            priority: TaskPriority::Medium,
            created_at: chrono::Utc::now(),
        }
    }
    
    pub fn with_requirements(mut self, requirements: TaskRequirements) -> Self {
        self.requirements = requirements;
        self
    }
    
    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }
}

/// Task input data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskInput {
    Text(String),
    Code { language: String, content: String },
    Analysis { target: String, metrics: Vec<String> },
    Research { topic: String, depth: ResearchDepth },
    Custom(serde_json::Value),
}

/// Task execution requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequirements {
    pub min_agents: usize,
    pub max_agents: Option<usize>,
    pub required_capabilities: Vec<String>,
    pub timeout: Option<Duration>,
    pub max_iterations: Option<usize>,
}

impl Default for TaskRequirements {
    fn default() -> Self {
        Self {
            min_agents: 1,
            max_agents: None,
            required_capabilities: Vec::new(),
            timeout: Some(Duration::from_secs(300)), // 5 minutes
            max_iterations: Some(100),
        }
    }
}

/// Task priority levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Research depth for research tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchDepth {
    Surface,
    Moderate,
    Deep,
    Exhaustive,
}

/// Strategy for task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStrategy {
    /// All agents work on the same task in parallel
    Parallel,
    
    /// Agents work sequentially, passing results
    Sequential,
    
    /// Agents vote on best approach/result
    Consensus { min_agreement: f64 },
    
    /// Task is broken into subtasks distributed across agents
    Distributed { max_subtasks: usize },
    
    /// Agents compete, best result wins
    Competitive,
    
    /// Adapt strategy based on task progress
    Adaptive,
}

/// Result of task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub status: TaskStatus,
    pub output: Option<TaskOutput>,
    pub metadata: TaskMetadata,
    pub agent_contributions: HashMap<Uuid, AgentContribution>,
}

/// Task execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running { progress: f64 },
    Completed,
    Failed { error: String },
    Cancelled,
    Timeout,
}

/// Task output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskOutput {
    Text(String),
    Code { language: String, content: String },
    Analysis(serde_json::Value),
    Multiple(Vec<TaskOutput>),
}

/// Metadata about task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetadata {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_ms: Option<u64>,
    pub iterations: usize,
    pub neural_activity: NeuralActivityMetrics,
}

/// Neural activity during task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralActivityMetrics {
    pub pathways_activated: usize,
    pub avg_pathway_strength: f64,
    pub hormonal_bursts: usize,
    pub memory_consolidations: usize,
}

impl Default for NeuralActivityMetrics {
    fn default() -> Self {
        Self {
            pathways_activated: 0,
            avg_pathway_strength: 0.0,
            hormonal_bursts: 0,
            memory_consolidations: 0,
        }
    }
}

/// Individual agent's contribution to a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContribution {
    pub agent_id: Uuid,
    pub agent_type: String,
    pub work_items: Vec<WorkItem>,
    pub confidence: f64,
    pub neural_impact: f64,
}

/// A unit of work performed by an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItem {
    pub description: String,
    pub result: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Task queue for managing multiple tasks
pub struct TaskQueue {
    pending: Vec<Task>,
    running: HashMap<Uuid, (Task, Instant)>,
    completed: Vec<TaskResult>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            running: HashMap::new(),
            completed: Vec::new(),
        }
    }
    
    pub fn enqueue(&mut self, task: Task) {
        // Insert based on priority
        let pos = self.pending
            .iter()
            .position(|t| t.priority < task.priority)
            .unwrap_or(self.pending.len());
        
        self.pending.insert(pos, task);
    }
    
    pub fn dequeue(&mut self) -> Option<Task> {
        self.pending.pop()
    }
    
    pub fn start_task(&mut self, task: Task) {
        self.running.insert(task.id, (task, Instant::now()));
    }
    
    pub fn complete_task(&mut self, result: TaskResult) {
        self.running.remove(&result.task_id);
        self.completed.push(result);
    }
    
    pub fn active_count(&self) -> usize {
        self.running.len()
    }
    
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_creation() {
        let task = Task::new(
            "Test task".to_string(),
            TaskInput::Text("Process this text".to_string()),
        );
        
        assert_eq!(task.description, "Test task");
        assert_eq!(task.priority, TaskPriority::Medium);
    }
    
    #[test]
    fn test_task_queue_priority() {
        let mut queue = TaskQueue::new();
        
        let low_task = Task::new("Low".to_string(), TaskInput::Text("".to_string()))
            .with_priority(TaskPriority::Low);
        
        let high_task = Task::new("High".to_string(), TaskInput::Text("".to_string()))
            .with_priority(TaskPriority::High);
        
        let med_task = Task::new("Medium".to_string(), TaskInput::Text("".to_string()))
            .with_priority(TaskPriority::Medium);
        
        queue.enqueue(low_task);
        queue.enqueue(high_task);
        queue.enqueue(med_task);
        
        // Should dequeue in priority order
        let first = queue.dequeue().unwrap();
        assert_eq!(first.description, "High");
        
        let second = queue.dequeue().unwrap();
        assert_eq!(second.description, "Medium");
        
        let third = queue.dequeue().unwrap();
        assert_eq!(third.description, "Low");
    }
}