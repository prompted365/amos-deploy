# Basic AMOS Demonstrations

Simple examples to get started with AMOS biological swarm orchestration.

## Available Demos

### 1. Hello Swarm (`hello-swarm.rs`)
- Spawn your first AMOS agents
- Basic neural mesh creation
- Simple task execution
- Console output visualization

### 2. Agent Coordination (`agent-coordination.rs`)
- Multiple agent types working together
- Pathway strengthening demonstration
- Basic hormonal responses
- Task result aggregation

### 3. Neural Visualization (`neural-viz.html`)
- Interactive web-based neural mesh viewer
- Real-time pathway updates
- Agent activity monitoring
- Uses AMOS WASM client

## Running the Demos

### Rust Demos
```bash
cargo run --example hello-swarm
cargo run --example agent-coordination
```

### Web Visualization
```bash
# Build WASM first
cd ../../../crates/amos-wasm && ./build.sh

# Serve the demo
python3 -m http.server 8080
# Open http://localhost:8080/neural-viz.html
```

## Key Learning Points

1. **Agent Creation**: How to spawn different agent types
2. **Task Assignment**: Basic task orchestration patterns
3. **Neural Activity**: Understanding pathway activation
4. **System State**: Monitoring mesh health and performance

## Next Steps

Once comfortable with these basics, explore the `../advanced/` directory for more complex scenarios.