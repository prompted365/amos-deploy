## Rust version based off working javascript one

```rs
// crates/amos-core/src/neural.rs
use std::collections::{HashMap, BTreeMap, HashSet};
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathwayConnection {
    pub strength: f64,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub use_count: u64,
}

impl PathwayConnection {
    pub fn new(initial_strength: f64) -> Self {
        Self {
            strength: initial_strength,
            last_used: chrono::Utc::now(),
            use_count: 0,
        }
    }
    
    pub fn strengthen(&mut self, delta: f64) {
        self.strength = (self.strength + delta).min(1.0);
        self.last_used = chrono::Utc::now();
        self.use_count += 1;
    }
    
    pub fn decay(&mut self, rate: f64) {
        self.strength *= 1.0 - rate;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub component: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub hits: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub count: u64,
    pub total_time: f64,
    pub avg_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub id: Uuid,
    pub interaction_type: InteractionType,
    pub target: Option<String>,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Chat,
    Document,
    Agent,
    Memory,
    Thinking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NeuralEvent {
    PathwayCreated {
        source: String,
        targets: Vec<String>,
    },
    PathwayStrengthened {
        source: String,
        target: String,
        strength: f64,
    },
    PathwayPruned {
        source: String,
        target: String,
    },
    NodeProcessed {
        node: String,
        duration_ms: f64,
    },
}

pub struct ForgeNeuralNetwork {
    /// Neural pathways with strength-based routing
    pathways: Arc<RwLock<HashMap<String, HashMap<String, PathwayConnection>>>>,
    
    /// Fast cache for endpoint mappings
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    
    /// Active connections
    active_connections: Arc<RwLock<HashSet<String>>>,
    
    /// Pathway usage tracking
    usage_stats: Arc<RwLock<HashMap<String, UsageStats>>>,
    
    /// Event broadcaster for mesh coordination
    event_sender: broadcast::Sender<NeuralEvent>,
    
    /// Neural event receiver
    _event_receiver: broadcast::Receiver<NeuralEvent>,
}

impl ForgeNeuralNetwork {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = broadcast::channel(1000);
        
        let network = Self {
            pathways: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            active_connections: Arc::new(RwLock::new(HashSet::new())),
            usage_stats: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            _event_receiver: event_receiver,
        };
        
        // Initialize in a blocking context or spawn a task
        tokio::spawn({
            let network = network.clone();
            async move {
                network.initialize_core_pathways().await;
            }
        });
        
        network
    }
    
    /// Initialize core neural pathways
    async fn initialize_core_pathways(&self) {
        let core_pathways = vec![
            ("ChatNode", "AgentNode", 0.8),
            ("ChatNode", "MemoryNode", 0.6),
            ("ChatNode", "ThinkingNode", 0.9),
            ("AgentNode", "MCPNode", 0.7),
            ("MemoryNode", "DocumentNode", 0.5),
            ("DocumentNode", "VectorNode", 0.8),
            ("VectorNode", "SearchNode", 0.9),
        ];
        
        for (source, target, strength) in core_pathways {
            self.create_pathway(source, &[target], strength).await;
        }
        
        info!("Initialized {} core neural pathways", core_pathways.len());
    }
    
    /// Process interaction through neural pathways
    pub async fn process(&self, mut interaction: Interaction) -> Result<serde_json::Value> {
        let start_time = std::time::Instant::now();
        
        let start_node = self.identify_start_node(&interaction);
        let target_node = interaction.target.clone()
            .unwrap_or_else(|| "AgentNode".to_string());
        
        debug!("Processing interaction {} from {} to {}", 
               interaction.id, start_node, target_node);
        
        // Find optimal path
        let active_path = self.find_optimal_path(&start_node, &target_node).await?;
        
        // Strengthen used pathways (Hebbian learning)
        self.strengthen_path(&active_path).await;
        
        // Process through each node in the path
        let mut result = interaction.data.clone();
        for node in &active_path {
            result = self.process_node(node, result).await?;
            
            // Emit processing event
            let _ = self.event_sender.send(NeuralEvent::NodeProcessed {
                node: node.clone(),
                duration_ms: start_time.elapsed().as_millis() as f64,
            });
        }
        
        // Update usage statistics
        self.update_usage_stats(&active_path, start_time.elapsed().as_millis() as f64).await;
        
        debug!("Processed interaction {} in {:?}", interaction.id, start_time.elapsed());
        Ok(result)
    }
    
    /// Create new neural pathway
    pub async fn create_pathway(&self, source: &str, targets: &[&str], initial_strength: f64) {
        let mut pathways = self.pathways.write().await;
        
        // Create source node if it doesn't exist
        if !pathways.contains_key(source) {
            pathways.insert(source.to_string(), HashMap::new());
        }
        
        let source_pathways = pathways.get_mut(source).unwrap();
        
        for &target in targets {
            // Create forward connection
            source_pathways.insert(
                target.to_string(),
                PathwayConnection::new(initial_strength)
            );
            
            // Create bidirectional connection with lower strength
            if !pathways.contains_key(target) {
                pathways.insert(target.to_string(), HashMap::new());
            }
            
            pathways.get_mut(target).unwrap().insert(
                source.to_string(),
                PathwayConnection::new(initial_strength * 0.5)
            );
        }
        
        // Emit pathway creation event
        let _ = self.event_sender.send(NeuralEvent::PathwayCreated {
            source: source.to_string(),
            targets: targets.iter().map(|s| s.to_string()).collect(),
        });
        
        debug!("Created pathway from {} to {:?} with strength {}", 
               source, targets, initial_strength);
    }
    
    /// Strengthen a pathway through use (Hebbian learning)
    async fn strengthen_path(&self, path: &[String]) {
        let mut pathways = self.pathways.write().await;
        
        for i in 0..path.len().saturating_sub(1) {
            let source = &path[i];
            let target = &path[i + 1];
            
            if let Some(source_connections) = pathways.get_mut(source) {
                if let Some(connection) = source_connections.get_mut(target) {
                    connection.strengthen(0.1);
                    
                    // Emit strengthening event
                    let _ = self.event_sender.send(NeuralEvent::PathwayStrengthened {
                        source: source.clone(),
                        target: target.clone(),
                        strength: connection.strength,
                    });
                    
                    debug!("Strengthened pathway {} → {} to {:.2}", 
                           source, target, connection.strength);
                }
            }
        }
        
        // Trigger synaptic pruning
        self.decay_unused_pathways(&mut pathways).await;
    }
    
    /// Decay pathways that haven't been used recently (synaptic pruning)
    async fn decay_unused_pathways(&self, pathways: &mut HashMap<String, HashMap<String, PathwayConnection>>) {
        const DECAY_RATE: f64 = 0.01;
        const HOUR_MS: i64 = 3600000;
        let now = chrono::Utc::now();
        
        let mut to_prune = Vec::new();
        
        for (source, targets) in pathways.iter_mut() {
            for (target, connection) in targets.iter_mut() {
                let time_since_use = now.signed_duration_since(connection.last_used)
                    .num_milliseconds();
                
                if time_since_use > HOUR_MS {
                    connection.decay(DECAY_RATE);
                    
                    // Mark very weak connections for pruning
                    if connection.strength < 0.1 {
                        to_prune.push((source.clone(), target.clone()));
                    }
                }
            }
        }
        
        // Prune weak connections
        for (source, target) in to_prune {
            if let Some(source_connections) = pathways.get_mut(&source) {
                source_connections.remove(&target);
                
                // Emit pruning event
                let _ = self.event_sender.send(NeuralEvent::PathwayPruned {
                    source: source.clone(),
                    target: target.clone(),
                });
                
                debug!("Pruned weak pathway {} → {}", source, target);
            }
        }
    }
    
    /// Find optimal path between nodes using A* with strength heuristic
    async fn find_optimal_path(&self, start: &str, end: &str) -> Result<Vec<String>> {
        let pathways = self.pathways.read().await;
        
        if start == end {
            return Ok(vec![start.to_string()]);
        }
        
        // A* pathfinding
        let mut open_set = HashSet::new();
        open_set.insert(start.to_string());
        
        let mut came_from: HashMap<String, String> = HashMap::new();
        let mut g_score: HashMap<String, f64> = HashMap::new();
        let mut f_score: HashMap<String, f64> = HashMap::new();
        
        g_score.insert(start.to_string(), 0.0);
        f_score.insert(start.to_string(), self.heuristic(start, end));
        
        while !open_set.is_empty() {
            // Find node with lowest f_score
            let current = open_set.iter()
                .min_by(|a, b| {
                    let a_score = f_score.get(*a).unwrap_or(&f64::INFINITY);
                    let b_score = f_score.get(*b).unwrap_or(&f64::INFINITY);
                    a_score.partial_cmp(b_score).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap()
                .clone();
            
            if current == end {
                return Ok(self.reconstruct_path(&came_from, &current));
            }
            
            open_set.remove(&current);
            
            // Check neighbors
            if let Some(neighbors) = pathways.get(&current) {
                for (neighbor, connection) in neighbors {
                    if connection.strength <= 0.1 {
                        continue; // Skip weak connections
                    }
                    
                    let tentative_g_score = g_score.get(&current).unwrap_or(&f64::INFINITY) 
                        + self.get_cost(&current, neighbor, connection);
                    
                    let neighbor_g_score = *g_score.get(neighbor).unwrap_or(&f64::INFINITY);
                    
                    if tentative_g_score < neighbor_g_score {
                        came_from.insert(neighbor.clone(), current.clone());
                        g_score.insert(neighbor.clone(), tentative_g_score);
                        f_score.insert(neighbor.clone(), 
                            tentative_g_score + self.heuristic(neighbor, end));
                        open_set.insert(neighbor.clone());
                    }
                }
            }
        }
        
        // No path found, create direct pathway
        warn!("No path found from {} to {}, creating new pathway", start, end);
        drop(pathways); // Release read lock
        self.create_pathway(start, &[end], 0.5).await;
        Ok(vec![start.to_string(), end.to_string()])
    }
    
    /// Get cost of moving between nodes (inverse of strength)
    fn get_cost(&self, _source: &str, _target: &str, connection: &PathwayConnection) -> f64 {
        1.0 / connection.strength.max(0.1) // Avoid division by zero
    }
    
    /// Heuristic for A* pathfinding
    fn heuristic(&self, node: &str, goal: &str) -> f64 {
        if node == goal {
            return 0.0;
        }
        
        // Simple heuristic based on node type similarity
        let node_type = node.replace("Node", "");
        let goal_type = goal.replace("Node", "");
        
        if node_type == goal_type {
            1.0
        } else if node.contains(&goal_type) || goal.contains(&node_type) {
            2.0
        } else {
            5.0
        }
    }
    
    /// Reconstruct path from A* came_from map
    fn reconstruct_path(&self, came_from: &HashMap<String, String>, current: &str) -> Vec<String> {
        let mut path = vec![current.to_string()];
        let mut current = current;
        
        while let Some(parent) = came_from.get(current) {
            path.insert(0, parent.clone());
            current = parent;
        }
        
        path
    }
    
    /// Process a single node
    async fn process_node(&self, node: &str, data: serde_json::Value) -> Result<serde_json::Value> {
        match node {
            "ChatNode" => {
                let mut result = data.as_object().unwrap_or(&serde_json::Map::new()).clone();
                result.insert("processed".to_string(), serde_json::Value::Bool(true));
                result.insert("chatEnhanced".to_string(), serde_json::Value::Bool(true));
                Ok(serde_json::Value::Object(result))
            }
            
            "AgentNode" => {
                let mut result = data.as_object().unwrap_or(&serde_json::Map::new()).clone();
                result.insert("agentProcessed".to_string(), serde_json::Value::Bool(true));
                Ok(serde_json::Value::Object(result))
            }
            
            "MemoryNode" => {
                let mut result = data.as_object().unwrap_or(&serde_json::Map::new()).clone();
                let memory_context = self.fetch_memory_context(&data).await?;
                result.insert("memoryContext".to_string(), memory_context);
                Ok(serde_json::Value::Object(result))
            }
            
            "ThinkingNode" => {
                let mut result = data.as_object().unwrap_or(&serde_json::Map::new()).clone();
                let reasoning = self.generate_reasoning(&data).await?;
                result.insert("reasoning".to_string(), reasoning);
                Ok(serde_json::Value::Object(result))
            }
            
            _ => {
                debug!("Unknown node type: {}, passing data through", node);
                Ok(data)
            }
        }
    }
    
    /// Cache endpoint mapping
    pub async fn cache_mapping(&self, endpoint: &str, component: &str) {
        let mut cache = self.cache.write().await;
        
        cache.insert(endpoint.to_string(), CacheEntry {
            component: component.to_string(),
            timestamp: chrono::Utc::now(),
            hits: 0,
        });
        
        // Limit cache size
        if cache.len() > 1000 {
            self.evict_oldest_cache(&mut cache).await;
        }
    }
    
    /// Get cached mapping
    pub async fn get_cached_mapping(&self, endpoint: &str) -> Option<String> {
        let mut cache = self.cache.write().await;
        
        if let Some(entry) = cache.get_mut(endpoint) {
            entry.hits += 1;
            
            // Refresh timestamp on every 10th hit
            if entry.hits % 10 == 0 {
                entry.timestamp = chrono::Utc::now();
            }
            
            Some(entry.component.clone())
        } else {
            None
        }
    }
    
    /// Get system health (ratio of active to total pathways)
    pub async fn get_health(&self) -> f64 {
        let pathways = self.pathways.read().await;
        
        let total_pathways: usize = pathways.values()
            .map(|targets| targets.len())
            .sum();
        
        let active_pathways: usize = pathways.values()
            .map(|targets| {
                targets.values()
                    .filter(|conn| conn.strength > 0.5)
                    .count()
            })
            .sum();
        
        if total_pathways == 0 {
            1.0
        } else {
            active_pathways as f64 / total_pathways as f64
        }
    }
    
    /// Subscribe to neural events
    pub fn subscribe_events(&self) -> broadcast::Receiver<NeuralEvent> {
        self.event_sender.subscribe()
    }
    
    /// Update usage statistics
    async fn update_usage_stats(&self, path: &[String], duration_ms: f64) {
        let path_key = path.join(" → ");
        let mut stats = self.usage_stats.write().await;
        
        let entry = stats.entry(path_key).or_insert(UsageStats {
            count: 0,
            total_time: 0.0,
            avg_time: 0.0,
        });
        
        entry.count += 1;
        entry.total_time += duration_ms;
        entry.avg_time = entry.total_time / entry.count as f64;
    }
    
    /// Helper methods
    fn identify_start_node(&self, interaction: &Interaction) -> String {
        match interaction.interaction_type {
            InteractionType::Chat => "ChatNode".to_string(),
            InteractionType::Document => "DocumentNode".to_string(),
            InteractionType::Agent => "AgentNode".to_string(),
            InteractionType::Memory => "MemoryNode".to_string(),
            InteractionType::Thinking => "ThinkingNode".to_string(),
        }
    }
    
    async fn evict_oldest_cache(&self, cache: &mut HashMap<String, CacheEntry>) {
        if let Some((oldest_key, _)) = cache.iter()
            .min_by(|a, b| a.1.timestamp.cmp(&b.1.timestamp))
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            cache.remove(&oldest_key);
            debug!("Evicted oldest cache entry: {}", oldest_key);
        }
    }
    
    async fn fetch_memory_context(&self, _data: &serde_json::Value) -> Result<serde_json::Value> {
        // Placeholder for memory fetching
        Ok(serde_json::json!({
            "relevant": [],
            "recent": []
        }))
    }
    
    async fn generate_reasoning(&self, _data: &serde_json::Value) -> Result<serde_json::Value> {
        // Placeholder for reasoning generation
        Ok(serde_json::json!({
            "steps": [],
            "conclusion": ""
        }))
    }
}

impl Clone for ForgeNeuralNetwork {
    fn clone(&self) -> Self {
        let (event_sender, event_receiver) = broadcast::channel(1000);
        
        Self {
            pathways: Arc::clone(&self.pathways),
            cache: Arc::clone(&self.cache),
            active_connections: Arc::clone(&self.active_connections),
            usage_stats: Arc::clone(&self.usage_stats),
            event_sender,
            _event_receiver: event_receiver,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[tokio::test]
    async fn test_pathway_creation() {
        let network = ForgeNeuralNetwork::new();
        
        network.create_pathway("TestSource", &["TestTarget"], 0.7).await;
        
        let pathways = network.pathways.read().await;
        assert!(pathways.contains_key("TestSource"));
        assert!(pathways.contains_key("TestTarget"));
        
        let connection = pathways.get("TestSource")
            .unwrap()
            .get("TestTarget")
            .unwrap();
        assert_eq!(connection.strength, 0.7);
    }
    
    #[tokio::test]
    async fn test_hebbian_learning() {
        let network = ForgeNeuralNetwork::new();
        
        network.create_pathway("NodeA", &["NodeB"], 0.5).await;
        
        // Simulate repeated use
        let path = vec!["NodeA".to_string(), "NodeB".to_string()];
        for _ in 0..5 {
            network.strengthen_path(&path).await;
        }
        
        let pathways = network.pathways.read().await;
        let connection = pathways.get("NodeA")
            .unwrap()
            .get("NodeB")
            .unwrap();
        
        assert!(connection.strength > 0.5, "Pathway should strengthen through use");
        assert!(connection.use_count == 5, "Use count should be tracked");
    }
    
    #[tokio::test]
    async fn test_pathway_processing() {
        let network = ForgeNeuralNetwork::new();
        
        let interaction = Interaction {
            id: Uuid::new_v4(),
            interaction_type: InteractionType::Chat,
            target: Some("AgentNode".to_string()),
            data: serde_json::json!({"message": "test"}),
            timestamp: chrono::Utc::now(),
        };
        
        let result = network.process(interaction).await.unwrap();
        
        assert!(result.get("processed").is_some());
        assert!(result.get("chatEnhanced").is_some());
    }
}
```

## There ya have it. 
