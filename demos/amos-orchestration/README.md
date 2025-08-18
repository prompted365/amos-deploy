# AMOS Orchestration Demos

This directory contains working demonstrations of the AMOS (Adaptive Mesh Operating System) framework, showcasing biological intelligence and swarm orchestration capabilities.

## Structure

```
demos/amos-orchestration/
├── basic/                    # Basic demonstrations
│   ├── hello-swarm.rs       # Simple swarm initialization and agent spawning
│   ├── agent-coordination.rs # Inter-agent communication and coordination
│   └── neural-viz.html      # Interactive neural network visualization
└── advanced/                 # Advanced demonstrations
    ├── emergent-consensus.rs # Multi-agent decision making
    └── stress-response.rs    # System behavior under extreme load
```

## Running the Demos

### Prerequisites

1. Ensure you have Rust installed (1.75 or later)
2. Clone the repository and navigate to the demo directory
3. The demos use the AMOS crates from the workspace

### Basic Demos

#### Hello Swarm
```bash
cd demos/amos-orchestration/basic
cargo run --bin hello-swarm
```
This demonstrates:
- Creating a neural network and event bus
- Initializing a swarm with mesh topology
- Spawning different types of cognitive agents
- Basic task orchestration

#### Agent Coordination
```bash
cargo run --bin agent-coordination
```
This shows:
- Hierarchical swarm structure
- Inter-agent communication via events
- Coordinated task execution
- Handling agent failures gracefully

#### Neural Visualization
Open `neural-viz.html` in a web browser to see:
- Real-time neural network activity
- Agent movements and influences
- Interactive controls for stimulation
- Different topology visualizations

### Advanced Demos

#### Emergent Consensus
```bash
cd demos/amos-orchestration/advanced
cargo run --bin emergent-consensus
```
Demonstrates:
- Multiple agents reaching consensus through voting
- Neural synchronization effects
- Different agent perspectives and biases
- Emergent behavior from simple rules

#### Stress Response
```bash
cargo run --bin stress-response
```
Shows:
- System behavior under increasing load
- Hormonal stress response mechanisms
- Agent failure handling and recovery
- Performance metrics and resilience scoring

## Key Features Demonstrated

### Biological Intelligence
- Neural networks with thousands of neurons
- Hormonal system responses (cortisol, adrenaline, etc.)
- Stress response and recovery mechanisms
- Neural synchronization and coherence

### Swarm Orchestration
- Multiple topology types (mesh, hierarchical, ring, star)
- Dynamic agent spawning and removal
- Task distribution strategies (parallel, sequential, adaptive)
- Inter-agent communication via event bus

### Agent Types
- **Traffic Seer**: Pattern recognition and flow analysis
- **Memory Weaver**: State management and historical context
- **Learning Oracle**: Predictive capabilities and adaptation
- **Pathway Sculptor**: Dynamic routing and optimization
- **Cognition Alchemist**: Creative problem solving
- **Mesh Harmonizer**: Swarm coordination
- **Performance Guardian**: System health monitoring
- **Consciousness Emergent**: Meta-cognitive capabilities

## Architecture

The demos are built on the AMOS framework:
- `amos-core`: Neural networks, event bus, hormonal system
- `amos-agents`: Cognitive agent implementations
- `amos-swarm`: Swarm orchestration and topology management
- `amos-neural`: Advanced neural processing capabilities

## Extending the Demos

To create your own demos:

1. Add a new binary to the appropriate `Cargo.toml`
2. Import the necessary AMOS crates
3. Initialize a neural network and event bus
4. Create a swarm with your chosen topology
5. Spawn agents and orchestrate tasks
6. Monitor system behavior and metrics

## Performance Notes

- The demos are designed to showcase functionality, not maximum performance
- Neural network sizes can be adjusted based on your system
- Agent counts and task loads can be modified
- Consider using release mode for better performance: `cargo run --release`

## Troubleshooting

If you encounter issues:
1. Ensure all AMOS crates are built: `cargo build --workspace`
2. Check that you're in the correct directory
3. Verify Rust version compatibility
4. For the HTML visualization, use a modern web browser

## Next Steps

After running these demos, explore:
- Creating custom agent types
- Implementing new swarm topologies
- Building real-world applications with AMOS
- Integrating with external systems via MCP

For more information, see the main AMOS documentation in the repository root.