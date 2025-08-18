# AMOS Backend API Server

This is the backend API server for AMOS deployment, providing REST endpoints and WebSocket support for the AMOS neural network system.

## Features

- **Health Check**: Monitor server status
- **Neural Network API**: Create nodes and pathways, monitor network status
- **Agent Management**: Spawn and manage AMOS agents (TrafficSeer, PathwaySculptor, etc.)
- **Swarm Coordination**: Monitor swarm topology and status
- **WebSocket Support**: Real-time event streaming
- **Static File Serving**: Serves the frontend build

## API Endpoints

### Health
- `GET /api/health` - Health check endpoint

### Neural Network
- `GET /api/neural/status` - Get neural network metrics (node/pathway count)
- `POST /api/neural/create-node` - Create a new neural node
- `POST /api/neural/create-pathway` - Create a pathway between nodes

### Agents
- `GET /api/agents/list` - List all agents
- `POST /api/agents/spawn` - Spawn a new agent

### Swarm
- `GET /api/swarm/status` - Get swarm status and topology

### System
- `GET /api/system/info` - Get system information

### WebSocket
- `WS /ws` - WebSocket connection for real-time events

## Running Locally

```bash
cd amos-deploy/backend
cargo run
```

The server will start on port 8080 by default.

## Environment Variables

- `PORT` - Server port (default: 8080)
- `AMOS_STATIC_DIR` - Directory for static files (default: "static")
- `RUST_LOG` - Logging level

## Agent Types

The following agent types can be spawned:
- `traffic_seer` - Traffic Seer
- `pathway_sculptor` - Pathway Sculptor
- `memory_weaver` - Memory Weaver
- `cognition_alchemist` - Cognition Alchemist
- `learning_oracle` - Learning Oracle
- `mesh_harmonizer` - Mesh Harmonizer
- `consciousness_emergent` - Consciousness Emergent
- `performance_guardian` - Performance Guardian

## Example Requests

### Create a Neural Node
```bash
curl -X POST http://localhost:8080/api/neural/create-node \
  -H "Content-Type: application/json" \
  -d '{"node_type": "memory"}'
```

### Spawn an Agent
```bash
curl -X POST http://localhost:8080/api/agents/spawn \
  -H "Content-Type: application/json" \
  -d '{"agent_type": "traffic_seer", "name": "My Traffic Seer"}'
```

### Create a Pathway
```bash
curl -X POST http://localhost:8080/api/neural/create-pathway \
  -H "Content-Type: application/json" \
  -d '{"source": "uuid-here", "target": "uuid-here", "strength": 0.5}'
```

## WebSocket Events

Connect to the WebSocket endpoint to receive real-time events:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', data);
};
```

Events include:
- Node creation
- Pathway creation
- Agent spawning
- System events