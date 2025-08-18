# AMOS MCP

## Purpose
Model Context Protocol (MCP) server and client integration for AMOS. This crate enables external control and integration of the biological mesh through standardized MCP tools.

## Components

### MCP Server
- `AMOSMCPServer`: Main MCP server implementation
- `ToolRegistry`: Available MCP tools registration
- `RequestHandler`: Process incoming MCP requests
- `ResponseFormatter`: Format responses to MCP spec

### MCP Tools

#### Agent Management
- `amos_spawn_agent`: Create new cognitive agents
- `amos_transform_shadow`: Convert agent to shadow mode
- `amos_query_agent`: Get agent status and metrics
- `amos_coordinate_agents`: Multi-agent task coordination

#### Neural Network Control
- `amos_strengthen_pathway`: Manually adjust pathway strength
- `amos_create_pathway`: Establish new neural connections
- `amos_prune_pathways`: Remove weak connections
- `amos_query_mesh`: Natural language mesh queries

#### Biological Systems
- `amos_hormonal_burst`: Trigger system-wide hormonal changes
- `amos_immune_scan`: Run immune system diagnostics
- `amos_memory_consolidate`: Force memory consolidation
- `amos_evolve_topology`: Reshape neural topology

#### System Management
- `amos_health_check`: Comprehensive system health
- `amos_performance_metrics`: Real-time performance data
- `amos_export_state`: Export mesh state for analysis
- `amos_import_state`: Load previous mesh state

### MCP Client Integration
- `MCPClient`: Connect to other MCP servers
- `ToolDiscovery`: Discover available external tools
- `ToolInvocation`: Execute external MCP tools
- `ResultProcessing`: Handle external tool results

## MCP Tool Definitions

```rust
pub struct MCPTool {
    name: String,
    description: String,
    parameters: Vec<Parameter>,
    returns: ReturnType,
}

// Example tool definition
MCPTool {
    name: "amos_spawn_agent".to_string(),
    description: "Spawn a new cognitive agent in the mesh".to_string(),
    parameters: vec![
        Parameter {
            name: "agent_type".to_string(),
            type_: ParameterType::Enum(vec![
                "TrafficSeer", "PathwaySculptor", "MemoryWeaver",
                "CognitionAlchemist", "LearningOracle", "MeshHarmonizer",
                "ConsciousnessEmergent", "PerformanceGuardian"
            ]),
            required: true,
            description: "Type of cognitive agent to spawn".to_string(),
        },
        Parameter {
            name: "config".to_string(),
            type_: ParameterType::Object,
            required: false,
            description: "Optional agent configuration".to_string(),
        }
    ],
    returns: ReturnType::Object(vec![
        ("agent_id", "UUID of spawned agent"),
        ("status", "Spawn status"),
        ("connections", "Initial neural connections")
    ]),
}
```

## Integration Patterns

### Claude Code Integration
```bash
# Add AMOS MCP to Claude Code
claude mcp add amos ./crates/amos-mcp/target/release/amos-mcp

# Use in Claude Code
claude "Use AMOS MCP to spawn a TrafficSeer agent and monitor neural activity"
```

### External MCP Server Integration
```rust
// Connect to external MCP servers
let github_mcp = MCPClient::connect("github-mcp").await?;
let tools = github_mcp.discover_tools().await?;

// Use external tools within AMOS
let result = github_mcp.invoke("github_create_issue", params).await?;
```

## Dependencies
- `amos-core`: Core AMOS types
- `amos-agents`: Agent definitions
- `amos-swarm`: Swarm coordination
- `mcp-rs`: MCP protocol implementation
- `tower`: Service middleware

## Connections
- **Depends on**: All AMOS crates
- **Used by**: External tools and Claude Code
- **Integrates with**: Any MCP-compatible system

## Security Considerations

### Authentication
- Token-based authentication
- Rate limiting per client
- IP allowlist support

### Authorization
- Role-based access control
- Tool-level permissions
- Audit logging

### Sandboxing
- Isolated execution contexts
- Resource limits
- Timeout enforcement

## Performance Optimization
- Connection pooling for external MCPs
- Response caching for read operations
- Batch operation support
- Async/streaming responses

## MCP Server Configuration

```toml
[mcp]
port = 3000
protocol = "stdio"  # or "http"
max_connections = 100
timeout_ms = 30000

[mcp.auth]
enabled = true
token_env = "AMOS_MCP_TOKEN"

[mcp.limits]
max_agents = 1000
max_pathways = 100000
max_memory_mb = 4096
```

## Development Guidelines
1. Follow MCP specification strictly
2. Validate all input parameters
3. Provide helpful error messages
4. Include examples in tool descriptions
5. Test with multiple MCP clients