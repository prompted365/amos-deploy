#!/bin/bash

# Build script for AMOS WASM module
set -e

echo "Building AMOS WASM module..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Clean previous build
rm -rf pkg/

# Build for web target with optimizations
echo "Building with wasm-pack..."
wasm-pack build --target web --release

# Create TypeScript definitions
echo "Generating TypeScript definitions..."
cat > pkg/amos.d.ts << 'EOF'
/* tslint:disable */
/* eslint-disable */
/**
* AMOS WASM Client - Biological mesh network for web browsers
*/

export enum AgentType {
  TrafficSeer = 'TrafficSeer',
  PathwaySculptor = 'PathwaySculptor',
  MemoryWeaver = 'MemoryWeaver',
  Architect = 'Architect',
  Builder = 'Builder',
  Critic = 'Critic',
  Guardian = 'Guardian',
  Tester = 'Tester',
  Optimizer = 'Optimizer',
  Explorer = 'Explorer',
  Coordinator = 'Coordinator',
}

export enum HormoneType {
  Cortisol = 'Cortisol',
  Dopamine = 'Dopamine',
  Serotonin = 'Serotonin',
  Oxytocin = 'Oxytocin',
  Adrenaline = 'Adrenaline',
}

export interface AgentInfo {
  id: string;
  name: string;
  agent_type: AgentType;
  state: string;
  neural_network_id: string;
}

export interface ProcessResult {
  output: string;
  pathways_activated: number;
  agents_involved: string[];
  processing_time_ms: number;
}

export interface MeshStatus {
  active_agents: number;
  total_pathways: number;
  total_nodes: number;
  memory_usage_bytes: number;
  uptime_seconds: number;
}

export class AMOSClient {
  constructor();
  
  /**
  * Spawn a new agent
  */
  spawnAgent(agent_type: AgentType): Promise<string>;
  
  /**
  * Get all agents
  */
  getAgents(): Promise<AgentInfo[]>;
  
  /**
  * Get a specific agent
  */
  getAgent(agent_id: string): Promise<AgentInfo>;
  
  /**
  * Process user input
  */
  processUserInput(input: string): Promise<ProcessResult>;
  
  /**
  * Strengthen a pathway between nodes
  */
  strengthenPathway(source: string, target: string, delta: number): Promise<void>;
  
  /**
  * Trigger a hormonal burst
  */
  triggerHormonalBurst(hormone: HormoneType, intensity: number): Promise<void>;
  
  /**
  * Get current mesh status
  */
  getMeshStatus(): Promise<MeshStatus>;
  
  /**
  * Create local neural network
  */
  createLocalNetwork(): Promise<string>;
  
  /**
  * Add a node to the network
  */
  addNode(node_type: string, data?: any): Promise<string>;
  
  /**
  * Connect two nodes
  */
  connectNodes(source_id: string, target_id: string, strength: number): Promise<void>;
  
  /**
  * Get hormone levels
  */
  getHormoneLevels(): Promise<Record<string, number>>;
  
  /**
  * Debug: dump current state
  */
  debugDumpState(): Promise<any>;
}

/**
* Get AMOS version
*/
export function version(): string;

/**
* Initialize the AMOS module
*/
export function init(): void;

EOF

# Create JavaScript wrapper for easier usage
echo "Creating JavaScript wrapper..."
cat > pkg/amos.js << 'EOF'
import init, * as wasm from './amos_wasm.js';

// Re-export enums
export const AgentType = {
  TrafficSeer: 'TrafficSeer',
  PathwaySculptor: 'PathwaySculptor',
  MemoryWeaver: 'MemoryWeaver',
  Architect: 'Architect',
  Builder: 'Builder',
  Critic: 'Critic',
  Guardian: 'Guardian',
  Tester: 'Tester',
  Optimizer: 'Optimizer',
  Explorer: 'Explorer',
  Coordinator: 'Coordinator',
};

export const HormoneType = {
  Cortisol: 'Cortisol',
  Dopamine: 'Dopamine',
  Serotonin: 'Serotonin',
  Oxytocin: 'Oxytocin',
  Adrenaline: 'Adrenaline',
};

// Re-export the main class and functions
export { AMOSClient, version } from './amos_wasm.js';

// Initialize function
export async function initialize(wasmPath) {
  await init(wasmPath);
  return wasm;
}

// Default export
export default { initialize, AgentType, HormoneType };

EOF

# Create example HTML file
echo "Creating example HTML..."
cat > pkg/example.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>AMOS WASM Example</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }
        button {
            margin: 5px;
            padding: 10px;
            cursor: pointer;
        }
        #output {
            border: 1px solid #ccc;
            padding: 10px;
            margin-top: 20px;
            height: 300px;
            overflow-y: auto;
            background: #f5f5f5;
        }
        .log-entry {
            margin: 5px 0;
            padding: 5px;
            background: white;
            border-radius: 3px;
        }
    </style>
</head>
<body>
    <h1>AMOS WASM Example</h1>
    
    <div>
        <h2>Agent Controls</h2>
        <button onclick="spawnAgent('TrafficSeer')">Spawn Traffic Seer</button>
        <button onclick="spawnAgent('MemoryWeaver')">Spawn Memory Weaver</button>
        <button onclick="spawnAgent('PathwaySculptor')">Spawn Pathway Sculptor</button>
        <button onclick="getAgents()">List Agents</button>
    </div>
    
    <div>
        <h2>Neural Processing</h2>
        <input type="text" id="userInput" placeholder="Enter text to process..." style="width: 300px;">
        <button onclick="processInput()">Process</button>
    </div>
    
    <div>
        <h2>Hormone Controls</h2>
        <button onclick="triggerHormone('Dopamine', 0.3)">Dopamine Burst</button>
        <button onclick="triggerHormone('Cortisol', 0.2)">Cortisol Spike</button>
        <button onclick="triggerHormone('Serotonin', 0.4)">Serotonin Boost</button>
    </div>
    
    <div>
        <h2>System</h2>
        <button onclick="getMeshStatus()">Mesh Status</button>
        <button onclick="getHormoneLevels()">Hormone Levels</button>
        <button onclick="debugState()">Debug State</button>
    </div>
    
    <div id="output"></div>
    
    <script type="module">
        import init, { AMOSClient } from './amos_wasm.js';
        
        let amos;
        
        async function initialize() {
            await init();
            amos = new AMOSClient();
            log('AMOS initialized');
        }
        
        function log(message, data = null) {
            const output = document.getElementById('output');
            const entry = document.createElement('div');
            entry.className = 'log-entry';
            entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
            if (data) {
                entry.textContent += '\n' + JSON.stringify(data, null, 2);
            }
            output.appendChild(entry);
            output.scrollTop = output.scrollHeight;
        }
        
        window.spawnAgent = async (agentType) => {
            try {
                const agentId = await amos.spawnAgent(agentType);
                log(`Spawned ${agentType}: ${agentId}`);
            } catch (e) {
                log('Error spawning agent: ' + e.message);
            }
        };
        
        window.getAgents = async () => {
            try {
                const agents = await amos.getAgents();
                log('Current agents:', agents);
            } catch (e) {
                log('Error getting agents: ' + e.message);
            }
        };
        
        window.processInput = async () => {
            const input = document.getElementById('userInput').value;
            if (!input) return;
            
            try {
                const result = await amos.processUserInput(input);
                log('Process result:', result);
            } catch (e) {
                log('Error processing input: ' + e.message);
            }
        };
        
        window.triggerHormone = async (hormone, intensity) => {
            try {
                await amos.triggerHormonalBurst(hormone, intensity);
                log(`Triggered ${hormone} burst with intensity ${intensity}`);
            } catch (e) {
                log('Error triggering hormone: ' + e.message);
            }
        };
        
        window.getMeshStatus = async () => {
            try {
                const status = await amos.getMeshStatus();
                log('Mesh status:', status);
            } catch (e) {
                log('Error getting mesh status: ' + e.message);
            }
        };
        
        window.getHormoneLevels = async () => {
            try {
                const levels = await amos.getHormoneLevels();
                log('Hormone levels:', levels);
            } catch (e) {
                log('Error getting hormone levels: ' + e.message);
            }
        };
        
        window.debugState = async () => {
            try {
                const state = await amos.debugDumpState();
                log('Debug state:', state);
            } catch (e) {
                log('Error getting debug state: ' + e.message);
            }
        };
        
        // Initialize on load
        initialize().catch(e => log('Initialization error: ' + e.message));
    </script>
</body>
</html>
EOF

# Create package.json for npm publishing
echo "Creating package.json..."
cat > pkg/package.json << 'EOF'
{
  "name": "@amos/wasm",
  "version": "0.1.0",
  "description": "AMOS biological mesh network for web browsers",
  "main": "amos.js",
  "types": "amos.d.ts",
  "files": [
    "amos_wasm_bg.wasm",
    "amos_wasm.js",
    "amos_wasm.d.ts",
    "amos.js",
    "amos.d.ts",
    "README.md"
  ],
  "keywords": [
    "amos",
    "wasm",
    "webassembly",
    "neural",
    "mesh",
    "biological"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/yourusername/amos"
  },
  "license": "MIT",
  "publishConfig": {
    "access": "public"
  }
}
EOF

# Create README for the package
echo "Creating package README..."
cat > pkg/README.md << 'EOF'
# AMOS WASM

WebAssembly library for running AMOS biological mesh in web browsers.

## Installation

```bash
npm install @amos/wasm
```

## Usage

```javascript
import init, { AMOSClient } from '@amos/wasm';

async function main() {
    // Initialize the WASM module
    await init();
    
    // Create client
    const amos = new AMOSClient();
    
    // Spawn agents
    const agentId = await amos.spawnAgent('TrafficSeer');
    
    // Process input
    const result = await amos.processUserInput('analyze neural patterns');
    console.log(result);
}

main();
```

## Features

- Lightweight (<1MB bundle size)
- TypeScript support
- React hooks available
- Web Worker compatible
- IndexedDB persistence

See the [documentation](https://github.com/yourusername/amos) for more details.
EOF

# Check final size
echo ""
echo "Build complete! Checking bundle size..."
ls -lh pkg/*.wasm | awk '{print "WASM size: " $5}'

echo ""
echo "Build artifacts in ./pkg/"
echo "- amos_wasm.js: Main JavaScript bindings"
echo "- amos_wasm_bg.wasm: WASM binary"
echo "- amos.js: Convenient wrapper"
echo "- amos.d.ts: TypeScript definitions"
echo "- example.html: Browser example"

echo ""
echo "To test, run:"
echo "  cd pkg && python3 -m http.server 8080"
echo "  Then open http://localhost:8080/example.html"