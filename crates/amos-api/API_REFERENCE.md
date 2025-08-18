# AMOS REST API Reference

The AMOS REST API provides comprehensive control over the neural swarm system.

## Base URL
```
http://localhost:3000/api/v1
```

## Authentication

All endpoints except `/health`, `/auth/*`, and documentation require JWT authentication.

### Login
```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "amos123"
}
```

Response:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 86400,
  "user_id": "admin-user-id",
  "role": "admin"
}
```

Use the token in subsequent requests:
```bash
Authorization: Bearer <token>
```

## Endpoints

### Agent Management

#### List Agents
```bash
GET /api/v1/agents
```

#### Get Agent
```bash
GET /api/v1/agents/{id}
```

#### Create Agent
```bash
POST /api/v1/agents
Content-Type: application/json

{
  "name": "Traffic Monitor",
  "agent_type": "traffic_seer",
  "shadow_mode": false
}
```

Available agent types:
- `traffic_seer` - Pattern recognition and monitoring
- `pathway_sculptor` - Neural pathway optimization
- `memory_weaver` - Memory management
- `cognition_alchemist` - Cognitive transformations
- `learning_oracle` - Learning and adaptation
- `mesh_harmonizer` - Coordination
- `consciousness_emergent` - Emergent behaviors
- `performance_guardian` - Performance monitoring

#### Delete Agent
```bash
DELETE /api/v1/agents/{id}
```

#### Send Command
```bash
POST /api/v1/agents/{id}/command
Content-Type: application/json

{
  "command": "start",
  "parameters": {}
}
```

### Neural Network Control

#### Get Neural State
```bash
GET /api/v1/neural/state
```

#### Update Neural Pathway
```bash
POST /api/v1/neural/pathways
Content-Type: application/json

{
  "from_node": "550e8400-e29b-41d4-a716-446655440000",
  "to_node": "550e8400-e29b-41d4-a716-446655440001",
  "strength_delta": 0.2,
  "reason": "Pattern recognition improvement"
}
```

### Hormonal System

#### Get Hormonal Levels
```bash
GET /api/v1/hormonal/levels
```

#### Update Hormonal Levels
```bash
POST /api/v1/hormonal/update
Content-Type: application/json

{
  "hormone": "dopamine",
  "delta": 0.1,
  "reason": "Reward for successful task completion"
}
```

### Swarm Orchestration

#### List Swarms
```bash
GET /api/v1/swarms
```

#### Create Swarm
```bash
POST /api/v1/swarms
Content-Type: application/json

{
  "name": "Analysis Swarm",
  "agent_ids": ["agent-id-1", "agent-id-2"]
}
```

#### Orchestrate Task
```bash
POST /api/v1/swarms/{id}/orchestrate
Content-Type: application/json

{
  "task": "Analyze system performance",
  "priority": "high",
  "parameters": {
    "duration": 300,
    "metrics": ["cpu", "memory", "latency"]
  }
}
```

### Performance Metrics

#### System Metrics
```bash
GET /api/v1/metrics/system?interval=60
```

#### Agent Metrics
```bash
GET /api/v1/metrics/agents
```

#### Swarm Metrics
```bash
GET /api/v1/metrics/swarms
```

### WebSocket Connection

Connect to real-time neural activity:
```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

// Subscribe to channels
ws.send(JSON.stringify({
  type: 'Subscribe',
  data: { channels: ['neural_activity', 'agent_updates'] }
}));

// Listen for messages
ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

## OpenAPI Documentation

Interactive API documentation is available at:
```
http://localhost:3000/swagger-ui
```

## Running the Server

```bash
# Set environment variables
export JWT_SECRET="your-secret-key"
export PORT=3000

# Run the server
cargo run --bin amos-api-server
```

## Example Client Usage

```python
import requests
import json

# Login
login_response = requests.post('http://localhost:3000/api/v1/auth/login', 
    json={'username': 'admin', 'password': 'amos123'})
token = login_response.json()['token']

# Set headers
headers = {'Authorization': f'Bearer {token}'}

# Create an agent
agent_data = {
    'name': 'Pattern Analyzer',
    'agent_type': 'traffic_seer',
    'shadow_mode': False
}
agent_response = requests.post('http://localhost:3000/api/v1/agents', 
    json=agent_data, headers=headers)
agent_id = agent_response.json()['id']

# Get neural state
neural_state = requests.get('http://localhost:3000/api/v1/neural/state', 
    headers=headers).json()
print(f"Neural network has {neural_state['total_nodes']} nodes")
```