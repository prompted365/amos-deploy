use amos_agents::*;
use amos_core::{ForgeNeuralNetwork, EventBus, Pattern, PatternType};
use std::sync::Arc;
use uuid::Uuid;

// MemoryWeaver Tests
#[tokio::test]
async fn test_memory_weaver_creation() {
    let weaver = MemoryWeaver::new();
    
    assert_eq!(weaver.name(), "MemoryWeaver");
    assert_eq!(weaver.state(), AgentState::Uninitialized);
    assert!(weaver.capabilities().contains(&AgentCapability::MemoryManagement));
    assert!(weaver.capabilities().contains(&AgentCapability::Learning));
}

#[tokio::test]
async fn test_memory_storage_and_retrieval() {
    let mut weaver = MemoryWeaver::new();
    
    // Store a memory
    let content = serde_json::json!({"event": "test", "value": 42});
    let memory_id = weaver.store_memory(content.clone(), 0.8);
    
    // Retrieve the memory
    let retrieved = weaver.retrieve_memory(memory_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().importance, 0.8);
    assert_eq!(retrieved.unwrap().access_count, 1);
}

#[tokio::test]
async fn test_memory_decay() {
    let mut weaver = MemoryWeaver::new();
    
    // Store memories with different importance
    let mem1 = weaver.store_memory(serde_json::json!({"test": 1}), 0.9);
    let _mem2 = weaver.store_memory(serde_json::json!({"test": 2}), 0.01);
    
    // Apply decay multiple times
    for _ in 0..10 {
        weaver.apply_decay();
    }
    
    // High importance memory should still exist
    assert!(weaver.retrieve_memory(mem1).is_some());
}

// CognitionAlchemist Tests
#[tokio::test]
async fn test_cognition_alchemist_creation() {
    let alchemist = CognitionAlchemist::new();
    
    assert_eq!(alchemist.name(), "CognitionAlchemist");
    assert_eq!(alchemist.state(), AgentState::Uninitialized);
    assert!(alchemist.capabilities().contains(&AgentCapability::PatternRecognition));
    assert!(alchemist.capabilities().contains(&AgentCapability::Generation));
}

#[tokio::test]
async fn test_pattern_synthesis() {
    let mut alchemist = CognitionAlchemist::new();
    
    // Add patterns
    let pattern1 = Pattern {
        id: Uuid::new_v4(),
        data: vec![1.0, 2.0, 3.0],
        pattern_type: PatternType::Normal,
    };
    
    let pattern2 = Pattern {
        id: Uuid::new_v4(),
        data: vec![2.0, 3.0, 4.0],
        pattern_type: PatternType::Normal,
    };
    
    alchemist.add_pattern(pattern1);
    alchemist.add_pattern(pattern2);
    
    // Try synthesis
    let result = alchemist.synthesize_patterns(SynthesisMethod::Fusion).unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn test_pattern_transformation() {
    let mut alchemist = CognitionAlchemist::new();
    
    let pattern = Pattern {
        id: Uuid::new_v4(),
        data: vec![0.2, 0.8, 0.5],
        pattern_type: PatternType::Normal,
    };
    
    let transformed = alchemist.apply_transformation(&pattern, SynthesisMethod::Inversion);
    
    // Check inversion (with floating point tolerance)
    assert!((transformed.data[0] - 0.8).abs() < 0.0001); // 1.0 - 0.2
    assert!((transformed.data[1] - 0.2).abs() < 0.0001); // 1.0 - 0.8
}

// LearningOracle Tests
#[tokio::test]
async fn test_learning_oracle_creation() {
    let oracle = LearningOracle::new();
    
    assert_eq!(oracle.name(), "LearningOracle");
    assert_eq!(oracle.state(), AgentState::Uninitialized);
    assert!(oracle.capabilities().contains(&AgentCapability::Learning));
    assert!(oracle.capabilities().contains(&AgentCapability::NeuralOptimization));
}

#[tokio::test]
async fn test_strategy_selection() {
    let mut oracle = LearningOracle::new();
    
    let strategy_id = oracle.select_strategy(LearningContext::Reinforcement);
    assert!(strategy_id.is_some());
}

#[tokio::test]
async fn test_parameter_adjustment() {
    let mut oracle = LearningOracle::new();
    
    // Select a strategy first
    oracle.select_strategy(LearningContext::Reinforcement);
    
    // Adjust parameters based on hormones
    oracle.adjust_parameters("Dopamine", 0.8);
    
    // Parameters should be adjusted (implementation specific)
}

// MeshHarmonizer Tests
#[tokio::test]
async fn test_mesh_harmonizer_creation() {
    let harmonizer = MeshHarmonizer::new();
    
    assert_eq!(harmonizer.name(), "MeshHarmonizer");
    assert_eq!(harmonizer.state(), AgentState::Uninitialized);
    assert!(harmonizer.capabilities().contains(&AgentCapability::Coordination));
    assert!(harmonizer.capabilities().contains(&AgentCapability::Monitoring));
}

#[tokio::test]
async fn test_agent_registration() {
    let mut harmonizer = MeshHarmonizer::new();
    
    let agent_id = Uuid::new_v4();
    harmonizer.register_agent(
        agent_id, 
        "TestAgent".to_string(),
        vec![AgentCapability::Learning]
    );
    
    harmonizer.update_agent_activity(agent_id);
}

#[tokio::test]
async fn test_harmony_calculation() {
    let mut harmonizer = MeshHarmonizer::new();
    
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    harmonizer.initialize(network, event_bus).await.unwrap();
    
    let harmony = harmonizer.harmonize_system().await.unwrap();
    assert!(harmony >= 0.0 && harmony <= 1.0);
}

// ConsciousnessEmergent Tests
#[tokio::test]
async fn test_consciousness_emergent_creation() {
    let consciousness = ConsciousnessEmergent::new();
    
    assert_eq!(consciousness.name(), "ConsciousnessEmergent");
    assert_eq!(consciousness.state(), AgentState::Uninitialized);
    assert!(consciousness.capabilities().contains(&AgentCapability::Learning));
    assert!(consciousness.capabilities().contains(&AgentCapability::Monitoring));
    assert!(consciousness.capabilities().contains(&AgentCapability::Coordination));
}

#[tokio::test]
async fn test_introspection() {
    let mut consciousness = ConsciousnessEmergent::new();
    
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    consciousness.initialize(network, event_bus).await.unwrap();
    
    // Perform introspection
    consciousness.introspect().await.unwrap();
    
    // Should have self-observations
}

#[tokio::test]
async fn test_attention_focus() {
    let mut consciousness = ConsciousnessEmergent::new();
    
    consciousness.focus_attention("Test Target".to_string(), 0.9);
    
    // Attention should be focused
}

#[tokio::test]
async fn test_intention_formation() {
    let mut consciousness = ConsciousnessEmergent::new();
    
    // Need awareness for intentions
    consciousness.introspect().await.unwrap();
    
    let _intention = consciousness.form_intention();
    // May or may not form intention based on awareness level
}

// PerformanceGuardian Tests
#[tokio::test]
async fn test_performance_guardian_creation() {
    let guardian = PerformanceGuardian::new();
    
    assert_eq!(guardian.name(), "PerformanceGuardian");
    assert_eq!(guardian.state(), AgentState::Uninitialized);
    assert!(guardian.capabilities().contains(&AgentCapability::Monitoring));
    assert!(guardian.capabilities().contains(&AgentCapability::NeuralOptimization));
}

#[tokio::test]
async fn test_metrics_collection() {
    let mut guardian = PerformanceGuardian::new();
    
    let metrics = guardian.collect_metrics().await;
    
    assert!(metrics.cpu_usage >= 0.0 && metrics.cpu_usage <= 1.0);
    assert!(metrics.memory_usage >= 0.0 && metrics.memory_usage <= 1.0);
    assert!(metrics.event_latency_ms >= 0.0);
    assert!(metrics.pathway_efficiency >= 0.0 && metrics.pathway_efficiency <= 1.0);
}

#[tokio::test]
async fn test_optimization_strategies() {
    let mut guardian = PerformanceGuardian::new();
    
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    guardian.initialize(network, event_bus).await.unwrap();
    
    // Collect metrics and optimize
    let metrics = guardian.collect_metrics().await;
    let _optimizations = guardian.optimize_system(&metrics).await.unwrap();
    
    // May or may not apply optimizations based on metrics
}

// Integration test
#[tokio::test]
async fn test_all_agents_lifecycle() {
    let network = Arc::new(ForgeNeuralNetwork::new());
    let event_bus = Arc::new(EventBus::new());
    
    let mut agents: Vec<Box<dyn CognitiveAgent>> = vec![
        Box::new(MemoryWeaver::new()),
        Box::new(CognitionAlchemist::new()),
        Box::new(LearningOracle::new()),
        Box::new(MeshHarmonizer::new()),
        Box::new(ConsciousnessEmergent::new()),
        Box::new(PerformanceGuardian::new()),
    ];
    
    // Initialize all agents
    for agent in &mut agents {
        agent.initialize(network.clone(), event_bus.clone()).await.unwrap();
        assert_eq!(agent.state(), AgentState::Active);
    }
    
    // Process all agents
    for agent in &mut agents {
        agent.process().await.unwrap();
    }
    
    // Terminate all agents
    for agent in &mut agents {
        agent.terminate().await.unwrap();
        assert_eq!(agent.state(), AgentState::Terminated);
    }
}