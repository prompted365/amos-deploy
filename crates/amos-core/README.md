# AMOS Core

## Purpose
Core biological mesh abstractions and foundational types for the AMOS system. This crate provides the fundamental building blocks that all other crates depend on.

## Components

### Neural Network Foundation
- `ForgeNeuralNetwork`: The central nervous system implementation
- `NeuralPathway`: Connection strength and learning mechanisms
- `CognitiveNode`: Base node types (Memory, Thinking, Agent, MCP, Gateway, Shadow)
- `HormonalState`: System-wide state modulation

### Biological Primitives
- Hebbian learning algorithms ("fire together, wire together")
- Synaptic pruning for unused connections
- Pathway strengthening and weakening
- Neural event bus for system-wide communication

### Immune System
- `ForgeImmuneSystem`: Pattern recognition and threat detection
- `ThreatDetector`: Anomaly detection interfaces
- `ResponseMechanism`: Self-healing capabilities
- Pattern memory for adaptive immunity

## Dependencies
- `tokio`: Async runtime for concurrent processing
- `serde`: Serialization for state persistence
- `dashmap`: Concurrent data structures
- `crossbeam`: Lock-free primitives
- `chrono`: Temporal tracking for pathways

## Connections
- **Used by**: All other AMOS crates
- **Integrates with**: amos-neural (extends neural capabilities)
- **Provides to**: amos-agents (cognitive infrastructure)

## Key Interfaces

```rust
pub trait BiologicalComponent: Send + Sync {
    async fn process(&self, input: SystemInput) -> SystemOutput;
    async fn adapt(&self, feedback: Feedback);
    async fn health_check(&self) -> HealthStatus;
}
```

## Performance Targets
- Neural pathway updates: < 1ms
- Concurrent pathway operations: 10,000+
- Memory footprint: < 50MB base
- Zero-copy pathway strengthening

## Development Guidelines
1. All biological components must be `Send + Sync`
2. Use `Arc<RwLock<T>>` for shared mutable state
3. Emit events for all significant state changes
4. Maintain compatibility with JavaScript AMOS behavior
5. Prioritize lock-free algorithms where possible