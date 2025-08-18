# AMOS Demonstrations

This directory contains comprehensive demonstrations of the AMOS biological swarm orchestration system.

## Directory Structure

```
demos/
├── amos-orchestration/    # Core AMOS swarm demonstrations
│   ├── basic/            # Simple examples for getting started
│   ├── advanced/         # Complex multi-agent scenarios
│   └── integrations/     # AMOS + ruv-swarm hybrid demos
└── README.md             # This file
```

## Quick Start

1. **Basic Demo**: Start with `amos-orchestration/basic/` for simple agent spawning
2. **Advanced Scenarios**: Explore `amos-orchestration/advanced/` for complex workflows
3. **Hybrid Power**: See `amos-orchestration/integrations/` for AMOS + ruv-swarm

## Key Features Demonstrated

- 🧠 Neural mesh network visualization
- 🤖 Multi-agent coordination patterns
- 💊 Hormonal system effects on behavior
- 🔄 Real-time adaptation and learning
- 📊 Performance metrics and monitoring

## Running Demos

All demos can be executed using the build scripts:
```bash
./demos/run-demos.sh [demo-name]
```

Or using ruv-swarm orchestration:
```bash
npx ruv-swarm orchestrate "Run AMOS demonstration: [demo-name]"
```