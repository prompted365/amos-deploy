use amos_core::neural::*;
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_neural_pathway_creation() {
    let source = Uuid::new_v4();
    let target = Uuid::new_v4();
    
    let pathway = NeuralPathway::new(source, target);
    
    assert_eq!(pathway.source_node, source);
    assert_eq!(pathway.target_node, target);
    assert_eq!(pathway.strength, 0.1); // Default starting strength
    assert_eq!(pathway.usage_count, 0);
    assert!(pathway.last_used <= Utc::now());
}

#[test]
fn test_neural_pathway_strengthening() {
    let mut pathway = NeuralPathway::new(Uuid::new_v4(), Uuid::new_v4());
    let initial_strength = pathway.strength;
    
    pathway.strengthen(0.1);
    
    assert_eq!(pathway.strength, initial_strength + 0.1);
    assert_eq!(pathway.usage_count, 1);
    assert!(pathway.strength <= 1.0); // Should not exceed 1.0
}

#[test]
fn test_neural_pathway_weakening() {
    let mut pathway = NeuralPathway::new(Uuid::new_v4(), Uuid::new_v4());
    pathway.strength = 0.5;
    
    pathway.weaken(0.2);
    
    assert_eq!(pathway.strength, 0.3);
    assert!(pathway.strength >= 0.0); // Should not go below 0.0
}

#[test]
fn test_cognitive_node_creation() {
    let node = CognitiveNode::new(NodeType::Memory);
    
    assert_eq!(node.node_type, NodeType::Memory);
    assert_eq!(node.connections.len(), 0);
    assert!(node.state.is_object());
}

#[test]
fn test_cognitive_node_add_connection() {
    let mut node = CognitiveNode::new(NodeType::Thinking);
    let connection_id = Uuid::new_v4();
    
    node.add_connection(connection_id);
    
    assert_eq!(node.connections.len(), 1);
    assert!(node.connections.contains(&connection_id));
}

#[test]
fn test_node_type_equality() {
    assert_eq!(NodeType::Memory, NodeType::Memory);
    assert_ne!(NodeType::Memory, NodeType::Thinking);
    assert_ne!(NodeType::Agent, NodeType::MCP);
}

#[test]
fn test_forge_neural_network_creation() {
    let network = ForgeNeuralNetwork::new();
    
    assert_eq!(network.node_count_sync(), 0);
    assert_eq!(network.pathway_count_sync(), 0);
}

#[test]
fn test_forge_neural_network_add_node() {
    let network = ForgeNeuralNetwork::new();
    
    let node_id = network.add_node_sync(NodeType::Memory);
    
    assert_eq!(network.node_count_sync(), 1);
    assert!(network.get_node_sync(node_id).is_some());
}

#[test]
fn test_forge_neural_network_create_pathway() {
    let network = ForgeNeuralNetwork::new();
    
    let node1 = network.add_node_sync(NodeType::Memory);
    let node2 = network.add_node_sync(NodeType::Thinking);
    
    let pathway_id = network.create_pathway_sync(node1, node2, 0.5);
    
    assert_eq!(network.pathway_count_sync(), 1);
    assert!(network.get_pathway_sync(pathway_id).is_some());
}

#[test]
fn test_hebbian_learning() {
    let network = ForgeNeuralNetwork::new();
    
    let node1 = network.add_node_sync(NodeType::Memory);
    let node2 = network.add_node_sync(NodeType::Thinking);
    
    // Fire together
    network.fire_node_sync(node1);
    network.fire_node_sync(node2);
    
    // Should strengthen or create pathway
    network.hebbian_learning_sync(node1, node2);
    
    let pathways = network.find_pathways_between_sync(node1, node2);
    assert!(!pathways.is_empty());
    
    let pathway = network.get_pathway_sync(pathways[0]).unwrap();
    assert!(pathway.strength > 0.0);
}

#[test]
fn test_synaptic_pruning() {
    let network = ForgeNeuralNetwork::new();
    
    let node1 = network.add_node_sync(NodeType::Memory);
    let node2 = network.add_node_sync(NodeType::Thinking);
    
    // Create weak pathway
    let pathway_id = network.create_pathway_sync(node1, node2, 0.1);
    
    // Simulate time passing without use
    network.run_synaptic_pruning_sync(0.2); // Prune if below 0.2
    
    // Weak pathway should be removed
    assert!(network.get_pathway_sync(pathway_id).is_none());
}

#[test]
fn test_neural_event_emission() {
    let network = ForgeNeuralNetwork::new();
    let mut event_rx = network.subscribe_to_events();
    
    let node1 = network.add_node_sync(NodeType::Memory);
    let node2 = network.add_node_sync(NodeType::Thinking);
    
    network.create_pathway_sync(node1, node2, 0.5);
    
    // Should have received pathway created event
    match event_rx.try_recv() {
        Ok(NeuralEvent::PathwayCreated { source, target, strength, .. }) => {
            assert_eq!(source, node1);
            assert_eq!(target, node2);
            assert_eq!(strength, 0.5);
        }
        _ => panic!("Expected PathwayCreated event"),
    }
}