# AMOS API

## Purpose
HTTP/WebSocket API server for external access to the AMOS biological mesh. Provides RESTful endpoints and real-time WebSocket connections for system interaction.

## Components

### HTTP Endpoints

#### System Management
- `GET /health` - System health check
- `GET /metrics` - Prometheus-compatible metrics
- `GET /status` - Comprehensive system status
- `POST /reset` - Reset mesh to initial state

#### Agent Operations
- `GET /agents` - List all active agents
- `POST /agents` - Spawn new agent
- `GET /agents/:id` - Get agent details
- `DELETE /agents/:id` - Remove agent
- `POST /agents/:id/shadow` - Transform to shadow mode

#### Neural Network
- `GET /neural/pathways` - List neural pathways
- `POST /neural/pathways` - Create new pathway
- `PUT /neural/pathways/:id` - Update pathway strength
- `DELETE /neural/pathways/:id` - Remove pathway
- `POST /neural/learn` - Trigger learning cycle

#### Biological Systems
- `POST /hormonal/burst` - Trigger hormonal burst
- `GET /immune/status` - Immune system status
- `POST /immune/scan` - Run threat scan
- `POST /memory/consolidate` - Consolidate memories

### WebSocket Channels

#### Real-time Monitoring
```typescript
// Connect to monitoring channel
ws.connect('/ws/monitor')

// Receive real-time updates
{
  "type": "pathway_update",
  "pathway_id": "uuid",
  "strength": 0.85,
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### Agent Communication
```typescript
// Direct agent interaction
ws.connect('/ws/agent/:id')

// Send commands
{
  "command": "process",
  "input": { "data": "...", "context": "..." }
}
```

#### Event Stream
```typescript
// Subscribe to system events
ws.connect('/ws/events')

// Event types
- agent_spawned
- pathway_strengthened
- hormonal_burst
- immune_threat_detected
- memory_consolidated
```

## API Architecture

### Request Pipeline
```rust
Request → Authentication → Validation → Handler → Response
         ↓                ↓            ↓         ↓
      Middleware      JSON Schema   Business   JSON/Proto
```

### Middleware Stack
- `AuthMiddleware`: Token validation
- `RateLimitMiddleware`: Request throttling
- `LoggingMiddleware`: Request/response logging
- `MetricsMiddleware`: Performance tracking
- `CorsMiddleware`: Cross-origin support

## OpenAPI Documentation

```yaml
openapi: 3.0.0
info:
  title: AMOS Biological Mesh API
  version: 1.0.0
  description: API for interacting with the AMOS cognitive system

paths:
  /agents:
    post:
      summary: Spawn a new cognitive agent
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                agent_type:
                  type: string
                  enum: [TrafficSeer, PathwaySculptor, ...]
                config:
                  type: object
      responses:
        201:
          description: Agent created successfully
```

## Dependencies
- `amos-core`: Core types and systems
- `amos-agents`: Agent implementations
- `amos-swarm`: Swarm coordination
- `axum`: Web framework
- `tower`: Middleware support
- `tokio-tungstenite`: WebSocket support

## Connections
- **Depends on**: Core AMOS functionality
- **Used by**: External clients, web UIs
- **Integrates with**: Monitoring systems, databases

## Authentication & Security

### API Key Authentication
```rust
#[derive(Debug, Clone)]
struct ApiKey(String);

impl FromRequestParts<AppState> for ApiKey {
    // Extract and validate API key from headers
}
```

### JWT Authentication
```rust
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    roles: Vec<String>,
}
```

### Rate Limiting
```rust
// Per-IP rate limiting
RateLimitLayer::new(100, Duration::from_secs(60))

// Per-API key rate limiting
ApiKeyRateLimit::new(1000, Duration::from_secs(60))
```

## Performance Considerations

### Response Caching
- Cache GET endpoints with ETags
- Invalidate on state changes
- Redis-backed distributed cache

### Connection Pooling
- Database connection pool
- Redis connection pool
- External service pools

### Async Processing
- All handlers are async
- Non-blocking I/O throughout
- Background task processing

## Monitoring Integration

### Prometheus Metrics
```rust
// Custom metrics
register_histogram!("amos_api_request_duration_seconds");
register_counter!("amos_api_requests_total");
register_gauge!("amos_active_connections");
```

### Health Checks
```rust
pub async fn health_check() -> HealthStatus {
    HealthStatus {
        api: "healthy",
        mesh: check_mesh_health().await,
        agents: check_agents_health().await,
        database: check_db_health().await,
    }
}
```

## Development Guidelines
1. Follow REST principles strictly
2. Version APIs appropriately
3. Provide comprehensive OpenAPI docs
4. Include request/response examples
5. Test with various client libraries