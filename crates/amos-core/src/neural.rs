use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct NeuralPathway {
    pub id: Uuid,
    pub strength: f64,
    pub last_used: DateTime<Utc>,
    pub usage_count: u64,
    pub source_node: Uuid,
    pub target_node: Uuid,
}

impl NeuralPathway {
    pub fn new(source: Uuid, target: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            strength: 0.1,
            last_used: Utc::now(),
            usage_count: 0,
            source_node: source,
            target_node: target,
        }
    }

    pub fn strengthen(&mut self, delta: f64) {
        self.strength = (self.strength + delta).min(1.0);
        self.usage_count += 1;
        self.last_used = Utc::now();
    }

    pub fn weaken(&mut self, delta: f64) {
        self.strength = (self.strength - delta).max(0.0);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Memory,
    Thinking,
    Agent,
    MCP,
    Gateway,
    Shadow,
}

#[derive(Debug, Clone)]
pub struct CognitiveNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub state: serde_json::Value,
    pub connections: Vec<Uuid>,
    pub processing_fn: String,
}

impl CognitiveNode {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            id: Uuid::new_v4(),
            node_type,
            state: serde_json::json!({}),
            connections: Vec::new(),
            processing_fn: String::new(),
        }
    }

    pub fn add_connection(&mut self, connection_id: Uuid) {
        self.connections.push(connection_id);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NeuralEvent {
    PathwayCreated {
        pathway_id: Uuid,
        source: Uuid,
        target: Uuid,
        strength: f64,
    },
    PathwayStrengthened {
        pathway_id: Uuid,
        new_strength: f64,
    },
    PathwayWeakened {
        pathway_id: Uuid,
        new_strength: f64,
    },
    PathwayRemoved {
        pathway_id: Uuid,
    },
    NodeFired {
        node_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Clone)]
pub struct ForgeNeuralNetwork {
    nodes: Arc<RwLock<HashMap<Uuid, CognitiveNode>>>,
    pathways: Arc<RwLock<HashMap<Uuid, NeuralPathway>>>,
    event_bus: broadcast::Sender<NeuralEvent>,
    fired_nodes: Arc<RwLock<HashMap<Uuid, DateTime<Utc>>>>,
}

impl ForgeNeuralNetwork {
    pub fn new() -> Self {
        let (event_bus, _) = broadcast::channel(1000);
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            pathways: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
            fired_nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn node_count(&self) -> usize {
        self.nodes.read().await.len()
    }

    pub async fn pathway_count(&self) -> usize {
        self.pathways.read().await.len()
    }

    pub async fn add_node(&self, node_type: NodeType) -> Uuid {
        let node = CognitiveNode::new(node_type);
        let node_id = node.id;
        self.nodes.write().await.insert(node_id, node);
        node_id
    }

    pub async fn get_node(&self, node_id: Uuid) -> Option<CognitiveNode> {
        self.nodes.read().await.get(&node_id).cloned()
    }

    pub async fn create_pathway(&self, source: Uuid, target: Uuid, strength: f64) -> Uuid {
        let mut pathway = NeuralPathway::new(source, target);
        pathway.strength = strength;
        let pathway_id = pathway.id;
        
        self.pathways.write().await.insert(pathway_id, pathway);
        
        let _ = self.event_bus.send(NeuralEvent::PathwayCreated {
            pathway_id,
            source,
            target,
            strength,
        });
        
        pathway_id
    }

    pub async fn get_pathway(&self, pathway_id: Uuid) -> Option<NeuralPathway> {
        self.pathways.read().await.get(&pathway_id).cloned()
    }

    pub async fn strengthen_pathway(&self, pathway_id: Uuid, delta: f64) {
        let mut pathways = self.pathways.write().await;
        if let Some(pathway) = pathways.get_mut(&pathway_id) {
            pathway.strengthen(delta);
            let new_strength = pathway.strength;
            
            let _ = self.event_bus.send(NeuralEvent::PathwayStrengthened {
                pathway_id,
                new_strength,
            });
        }
    }

    pub async fn fire_node(&self, node_id: Uuid) {
        self.fired_nodes.write().await.insert(node_id, Utc::now());
        
        let _ = self.event_bus.send(NeuralEvent::NodeFired {
            node_id,
            timestamp: Utc::now(),
        });
    }

    pub async fn hebbian_learning(&self, source: Uuid, target: Uuid) {
        let fired_nodes = self.fired_nodes.read().await;
        
        // Check if both nodes fired recently (within 100ms)
        if let (Some(source_time), Some(target_time)) = 
            (fired_nodes.get(&source), fired_nodes.get(&target)) {
            
            let time_diff = (*target_time - *source_time).num_milliseconds().abs();
            if time_diff < 100 {
                // Fire together, wire together
                if let Some(pathway_id) = self.find_pathway(source, target).await {
                    self.strengthen_pathway(pathway_id, 0.1).await;
                } else {
                    self.create_pathway(source, target, 0.1).await;
                }
            }
        }
    }

    pub async fn find_pathway(&self, source: Uuid, target: Uuid) -> Option<Uuid> {
        let pathways = self.pathways.read().await;
        pathways.iter()
            .find(|(_, p)| p.source_node == source && p.target_node == target)
            .map(|(id, _)| *id)
    }

    pub async fn find_pathways_between(&self, source: Uuid, target: Uuid) -> Vec<Uuid> {
        let pathways = self.pathways.read().await;
        pathways.iter()
            .filter(|(_, p)| p.source_node == source && p.target_node == target)
            .map(|(id, _)| *id)
            .collect()
    }

    pub async fn run_synaptic_pruning(&self, threshold: f64) {
        let mut pathways = self.pathways.write().await;
        let to_remove: Vec<Uuid> = pathways.iter()
            .filter(|(_, p)| p.strength < threshold)
            .map(|(id, _)| *id)
            .collect();
        
        for pathway_id in to_remove {
            pathways.remove(&pathway_id);
            let _ = self.event_bus.send(NeuralEvent::PathwayRemoved { pathway_id });
        }
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<NeuralEvent> {
        self.event_bus.subscribe()
    }
}

// For sync tests
impl ForgeNeuralNetwork {
    pub fn node_count_sync(&self) -> usize {
        tokio::runtime::Runtime::new().unwrap().block_on(self.node_count())
    }

    pub fn pathway_count_sync(&self) -> usize {
        tokio::runtime::Runtime::new().unwrap().block_on(self.pathway_count())
    }

    pub fn add_node_sync(&self, node_type: NodeType) -> Uuid {
        tokio::runtime::Runtime::new().unwrap().block_on(self.add_node(node_type))
    }

    pub fn get_node_sync(&self, node_id: Uuid) -> Option<CognitiveNode> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_node(node_id))
    }

    pub fn create_pathway_sync(&self, source: Uuid, target: Uuid, strength: f64) -> Uuid {
        tokio::runtime::Runtime::new().unwrap().block_on(self.create_pathway(source, target, strength))
    }

    pub fn get_pathway_sync(&self, pathway_id: Uuid) -> Option<NeuralPathway> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_pathway(pathway_id))
    }

    pub fn fire_node_sync(&self, node_id: Uuid) {
        tokio::runtime::Runtime::new().unwrap().block_on(self.fire_node(node_id))
    }

    pub fn hebbian_learning_sync(&self, source: Uuid, target: Uuid) {
        tokio::runtime::Runtime::new().unwrap().block_on(self.hebbian_learning(source, target))
    }

    pub fn find_pathways_between_sync(&self, source: Uuid, target: Uuid) -> Vec<Uuid> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.find_pathways_between(source, target))
    }

    pub fn run_synaptic_pruning_sync(&self, threshold: f64) {
        tokio::runtime::Runtime::new().unwrap().block_on(self.run_synaptic_pruning(threshold))
    }
}

