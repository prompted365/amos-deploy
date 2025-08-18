# AMOS CLI

## Purpose
Command-line interface for interacting with the AMOS biological mesh. Provides comprehensive tools for system management, monitoring, and development.

## Commands

### System Management
```bash
# Initialize new AMOS instance
amos init --topology mesh --agents 8

# Start AMOS system
amos start --config ./amos.toml

# Check system status
amos status --verbose

# Stop AMOS system
amos stop --graceful
```

### Agent Management
```bash
# List all agents
amos agent list

# Spawn new agent
amos agent spawn TrafficSeer --name "Traffic Monitor"

# Get agent details
amos agent info <agent-id>

# Transform to shadow mode
amos agent shadow <agent-id>

# Remove agent
amos agent remove <agent-id>
```

### Neural Network Operations
```bash
# View neural pathways
amos neural pathways --filter "strength>0.5"

# Strengthen pathway
amos neural strengthen <source-id> <target-id> --delta 0.1

# Trigger learning cycle
amos neural learn --algorithm hebbian

# Export neural state
amos neural export --output ./neural-state.json
```

### Biological Systems
```bash
# Trigger hormonal burst
amos hormonal burst --type dopamine --intensity 0.8

# Run immune scan
amos immune scan --deep

# Consolidate memory
amos memory consolidate --force

# View memory patterns
amos memory patterns --type episodic
```

### Monitoring & Analysis
```bash
# Real-time monitoring
amos monitor --metrics all

# Performance analysis
amos analyze performance --duration 1h

# Generate report
amos report generate --format html --output ./report.html

# Benchmark operations
amos benchmark --suite full
```

### Development Tools
```bash
# Interactive REPL
amos repl

# Run diagnostics
amos diagnose --comprehensive

# Validate configuration
amos config validate ./amos.toml

# Generate completion scripts
amos completions bash > /etc/bash_completion.d/amos
```

## CLI Architecture

### Command Structure
```rust
#[derive(Parser)]
#[command(name = "amos")]
#[command(about = "AMOS Biological Mesh CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
    
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    Init(InitArgs),
    Start(StartArgs),
    Agent(AgentCommands),
    Neural(NeuralCommands),
    // ...
}
```

### Interactive REPL
```rust
// REPL commands
> agent.spawn(TrafficSeer)
Agent spawned: 7f3a2b1c-...

> neural.pathways().filter(p => p.strength > 0.7)
[Pathway { id: ..., strength: 0.85 }, ...]

> mesh.hormonal_burst(Dopamine, 0.5)
Hormonal burst triggered

> monitor.start()
Monitoring started... (Ctrl+C to stop)
```

## Configuration

### Config File (amos.toml)
```toml
[system]
name = "production-amos"
topology = "mesh"
max_agents = 100

[neural]
learning_rate = 0.1
pruning_threshold = 0.3
max_pathways = 100000

[api]
host = "0.0.0.0"
port = 8080

[monitoring]
enable_metrics = true
metrics_port = 9090
```

### Environment Variables
```bash
AMOS_CONFIG_PATH=/etc/amos/config.toml
AMOS_LOG_LEVEL=info
AMOS_API_TOKEN=secret-token
AMOS_NEURAL_CACHE_SIZE=1000000
```

## Output Formats

### JSON Output
```bash
amos agent list --output json
```

### Table Output (Default)
```bash
amos agent list
┌─────────────┬──────────────┬────────┬─────────┐
│ ID          │ Type         │ Status │ Shadow  │
├─────────────┼──────────────┼────────┼─────────┤
│ 7f3a2b1c... │ TrafficSeer  │ Active │ No      │
│ 8e4b3c2d... │ MemoryWeaver │ Active │ Yes     │
└─────────────┴──────────────┴────────┴─────────┘
```

### CSV Output
```bash
amos neural pathways --output csv > pathways.csv
```

## Dependencies
- `amos-core`: Core functionality
- `amos-api`: API client
- `clap`: Command parsing
- `tokio`: Async runtime
- `ratatui`: Terminal UI
- `indicatif`: Progress bars

## Connections
- **Depends on**: All AMOS crates
- **Used by**: Developers, operators
- **Integrates with**: Shell environments

## Advanced Features

### Scripting Support
```bash
#!/usr/bin/env amos-script

# Automated maintenance script
agents = agent.list()
for a in agents:
    if a.idle_time > 3600:
        agent.shadow(a.id)
        
neuralneural.prune(threshold=0.2)
memory.consolidate()
```

### Plugin System
```rust
// Custom command plugin
#[amos_plugin]
fn custom_analysis(mesh: &Mesh) -> Result<Report> {
    // Custom analysis logic
}
```

### Batch Operations
```bash
# Batch file operations
amos batch < commands.txt

# Where commands.txt contains:
agent spawn TrafficSeer
agent spawn MemoryWeaver
neural learn --iterations 100
```

## Error Handling

### User-Friendly Errors
```
Error: Failed to spawn agent
  Caused by: Maximum agent limit (100) reached
  
Suggestion: Remove inactive agents or increase limit in config
```

### Debug Mode
```bash
AMOS_LOG_LEVEL=debug amos agent spawn TrafficSeer
[DEBUG] Connecting to mesh at localhost:8080
[DEBUG] Sending spawn request: AgentType::TrafficSeer
[DEBUG] Response received in 23ms
```

## Development Guidelines
1. Keep commands intuitive and consistent
2. Provide helpful error messages
3. Support multiple output formats
4. Include progress indicators for long operations
5. Test with various terminal environments