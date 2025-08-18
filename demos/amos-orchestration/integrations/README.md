# AMOS + ruv-swarm Integration Demonstrations

Hybrid orchestration combining AMOS biological intelligence with ruv-swarm's coordination capabilities.

## Hybrid Demos

### 1. Unified Orchestration (`unified-orchestration.js`)
- AMOS agents managed by ruv-swarm
- Cross-system task delegation
- Shared memory and state
- Performance comparison

### 2. Neural Enhancement (`neural-enhancement.rs`)
- ruv-swarm cognitive patterns
- AMOS neural mesh integration
- Enhanced decision making
- Hybrid learning algorithms

### 3. Full Stack Demo (`fullstack-demo/`)
- React frontend with AMOS-WASM
- ruv-swarm MCP coordination
- Real-time WebSocket updates
- Production-ready architecture

## Integration Patterns

### Pattern 1: AMOS as Intelligence Layer
```javascript
// ruv-swarm orchestrates, AMOS thinks
const swarm = await ruvSwarm.init('hierarchical');
const amosClient = new AMOSClient();

swarm.on('task', async (task) => {
  const analysis = await amosClient.processUserInput(task.description);
  return swarm.executeWithInsights(task, analysis);
});
```

### Pattern 2: Dual Swarm Coordination
```rust
// Both systems work in parallel
let amos_swarm = AmosSwarm::new();
let ruv_swarm = RuvSwarm::new();

let (amos_result, ruv_result) = tokio::join!(
    amos_swarm.orchestrate(&task),
    ruv_swarm.orchestrate(&task)
);
```

### Pattern 3: Biological MCP Extension
```javascript
// AMOS extends ruv-swarm's MCP tools
mcp.registerTool('amos_neural_process', async (input) => {
  return amosClient.processWithHormones(input, {
    dopamine: 0.8,  // High motivation
    cortisol: 0.2   // Low stress
  });
});
```

## Running Integrations

```bash
# Start both systems
./start-hybrid-swarm.sh

# Run unified demo
node unified-orchestration.js

# Full stack demo
cd fullstack-demo && npm start
```

## Benefits of Integration

1. **Best of Both Worlds**: Structure + Emergence
2. **Enhanced Performance**: 15x improvements possible
3. **Flexible Architecture**: Use what works best
4. **Future-Proof**: Evolving AI systems