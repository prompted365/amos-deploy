// Web Worker Example for AMOS WASM
// This shows how to run AMOS in a background worker thread

// worker.js - The worker script
const workerCode = `
importScripts('./amos_wasm.js');

let amos = null;

// Message handler
self.onmessage = async function(e) {
    const { type, id, ...params } = e.data;
    
    try {
        let result;
        
        switch (type) {
            case 'init':
                // Initialize AMOS in the worker
                await wasm_bindgen('./amos_wasm_bg.wasm');
                amos = new wasm_bindgen.AMOSClient();
                result = { success: true };
                break;
                
            case 'spawn_agent':
                result = await amos.spawnAgent(params.agentType);
                break;
                
            case 'process_input':
                result = await amos.processUserInput(params.input);
                break;
                
            case 'get_agents':
                result = await amos.getAgents();
                break;
                
            case 'get_mesh_status':
                result = await amos.getMeshStatus();
                break;
                
            case 'trigger_hormone':
                await amos.triggerHormonalBurst(params.hormone, params.intensity);
                result = { success: true };
                break;
                
            case 'strengthen_pathway':
                await amos.strengthenPathway(params.source, params.target, params.delta);
                result = { success: true };
                break;
                
            case 'batch_process':
                // Process multiple inputs in parallel
                const results = await Promise.all(
                    params.inputs.map(input => amos.processUserInput(input))
                );
                result = results;
                break;
                
            default:
                throw new Error(\`Unknown command: \${type}\`);
        }
        
        // Send result back
        self.postMessage({ id, type: 'result', result });
        
    } catch (error) {
        // Send error back
        self.postMessage({ 
            id, 
            type: 'error', 
            error: error.message 
        });
    }
};

// Periodic tasks
let intervalId = null;

self.onmessage = async function(e) {
    if (e.data.type === 'start_monitoring') {
        intervalId = setInterval(async () => {
            if (amos) {
                const status = await amos.getMeshStatus();
                self.postMessage({ 
                    type: 'status_update', 
                    status 
                });
            }
        }, e.data.interval || 1000);
    } else if (e.data.type === 'stop_monitoring') {
        if (intervalId) {
            clearInterval(intervalId);
            intervalId = null;
        }
    }
};
`;

// Main thread code - AMOSWorkerClient class
class AMOSWorkerClient {
    constructor() {
        this.worker = null;
        this.messageId = 0;
        this.pendingMessages = new Map();
        this.statusCallback = null;
    }
    
    async initialize() {
        // Create worker from blob
        const blob = new Blob([workerCode], { type: 'application/javascript' });
        const workerUrl = URL.createObjectURL(blob);
        this.worker = new Worker(workerUrl);
        
        // Set up message handler
        this.worker.onmessage = (e) => {
            const { id, type, result, error, status } = e.data;
            
            if (type === 'result' || type === 'error') {
                const pending = this.pendingMessages.get(id);
                if (pending) {
                    if (type === 'error') {
                        pending.reject(new Error(error));
                    } else {
                        pending.resolve(result);
                    }
                    this.pendingMessages.delete(id);
                }
            } else if (type === 'status_update' && this.statusCallback) {
                this.statusCallback(status);
            }
        };
        
        // Initialize AMOS in worker
        await this.sendMessage('init');
    }
    
    sendMessage(type, params = {}) {
        return new Promise((resolve, reject) => {
            const id = this.messageId++;
            this.pendingMessages.set(id, { resolve, reject });
            this.worker.postMessage({ id, type, ...params });
        });
    }
    
    // AMOS API methods
    async spawnAgent(agentType) {
        return this.sendMessage('spawn_agent', { agentType });
    }
    
    async processUserInput(input) {
        return this.sendMessage('process_input', { input });
    }
    
    async getAgents() {
        return this.sendMessage('get_agents');
    }
    
    async getMeshStatus() {
        return this.sendMessage('get_mesh_status');
    }
    
    async triggerHormonalBurst(hormone, intensity) {
        return this.sendMessage('trigger_hormone', { hormone, intensity });
    }
    
    async strengthenPathway(source, target, delta) {
        return this.sendMessage('strengthen_pathway', { source, target, delta });
    }
    
    async batchProcess(inputs) {
        return this.sendMessage('batch_process', { inputs });
    }
    
    // Monitoring
    startMonitoring(callback, interval = 1000) {
        this.statusCallback = callback;
        this.worker.postMessage({ type: 'start_monitoring', interval });
    }
    
    stopMonitoring() {
        this.worker.postMessage({ type: 'stop_monitoring' });
        this.statusCallback = null;
    }
    
    // Cleanup
    terminate() {
        this.stopMonitoring();
        this.worker.terminate();
    }
}

// Example usage HTML
const exampleHTML = `
<!DOCTYPE html>
<html>
<head>
    <title>AMOS Web Worker Example</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 1000px;
            margin: 0 auto;
            padding: 20px;
        }
        .status-box {
            background: #f0f0f0;
            padding: 15px;
            border-radius: 5px;
            margin-bottom: 20px;
        }
        .controls {
            margin-bottom: 20px;
        }
        button {
            margin: 5px;
            padding: 10px 15px;
        }
        #log {
            background: #f9f9f9;
            border: 1px solid #ddd;
            padding: 10px;
            height: 300px;
            overflow-y: auto;
            font-family: monospace;
            font-size: 12px;
        }
        .batch-section {
            background: #e8f4f8;
            padding: 15px;
            border-radius: 5px;
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <h1>AMOS Web Worker Example</h1>
    
    <div class="status-box">
        <h3>Real-time Status</h3>
        <div id="status">Initializing...</div>
    </div>
    
    <div class="controls">
        <h3>Agent Controls</h3>
        <button onclick="spawnRandomAgent()">Spawn Random Agent</button>
        <button onclick="listAgents()">List All Agents</button>
    </div>
    
    <div class="controls">
        <h3>Neural Processing</h3>
        <input type="text" id="input" placeholder="Enter text to process" style="width: 300px;">
        <button onclick="processInput()">Process</button>
    </div>
    
    <div class="batch-section">
        <h3>Batch Processing Demo</h3>
        <p>Process multiple inputs in parallel using the worker thread:</p>
        <button onclick="runBatchDemo()">Run Batch Processing</button>
        <div id="batchResults"></div>
    </div>
    
    <h3>Activity Log</h3>
    <div id="log"></div>
    
    <script type="module">
        // Include the AMOSWorkerClient code here
        ${workerCode}
        
        // Initialize
        let client;
        
        async function init() {
            log('Initializing AMOS in Web Worker...');
            client = new AMOSWorkerClient();
            await client.initialize();
            log('AMOS Worker initialized successfully');
            
            // Start real-time monitoring
            client.startMonitoring((status) => {
                document.getElementById('status').innerHTML = \`
                    Agents: \${status.active_agents} | 
                    Pathways: \${status.total_pathways} | 
                    Nodes: \${status.total_nodes} | 
                    Memory: \${(status.memory_usage_bytes / 1024).toFixed(2)} KB | 
                    Uptime: \${status.uptime_seconds}s
                \`;
            });
            
            // Spawn initial agents
            await spawnInitialAgents();
        }
        
        async function spawnInitialAgents() {
            const types = ['TrafficSeer', 'MemoryWeaver', 'PathwaySculptor'];
            for (const type of types) {
                const id = await client.spawnAgent(type);
                log(\`Spawned \${type}: \${id}\`);
            }
        }
        
        async function spawnRandomAgent() {
            const types = ['TrafficSeer', 'MemoryWeaver', 'PathwaySculptor', 
                          'Architect', 'Builder', 'Critic', 'Guardian'];
            const type = types[Math.floor(Math.random() * types.length)];
            
            const id = await client.spawnAgent(type);
            log(\`Spawned \${type}: \${id}\`);
        }
        
        async function listAgents() {
            const agents = await client.getAgents();
            log(\`Active agents (\${agents.length}):\`);
            agents.forEach(agent => {
                log(\`  - \${agent.name} (\${agent.agent_type}): \${agent.state}\`);
            });
        }
        
        async function processInput() {
            const input = document.getElementById('input').value;
            if (!input) return;
            
            log(\`Processing: "\${input}"\`);
            const result = await client.processUserInput(input);
            log(\`Result: \${result.output}\`);
            log(\`  Pathways: \${result.pathways_activated}, Time: \${result.processing_time_ms}ms\`);
            
            // Trigger dopamine if good result
            if (result.pathways_activated > 5) {
                await client.triggerHormonalBurst('Dopamine', 0.2);
                log('  -> Dopamine reward triggered!');
            }
        }
        
        async function runBatchDemo() {
            const inputs = [
                'analyze neural network performance',
                'optimize memory pathways',
                'strengthen agent connections',
                'process visual information',
                'coordinate team activities',
                'explore new solutions',
                'test system resilience',
                'build new neural structures'
            ];
            
            log('Starting batch processing of ' + inputs.length + ' inputs...');
            const startTime = Date.now();
            
            const results = await client.batchProcess(inputs);
            const endTime = Date.now();
            
            const totalTime = endTime - startTime;
            const avgTime = totalTime / inputs.length;
            
            let html = '<h4>Batch Results:</h4><ul>';
            results.forEach((result, i) => {
                html += \`<li>"\${inputs[i]}" - \${result.pathways_activated} pathways, \${result.processing_time_ms}ms</li>\`;
            });
            html += '</ul>';
            html += \`<p>Total time: \${totalTime}ms, Average: \${avgTime.toFixed(2)}ms per input</p>\`;
            
            document.getElementById('batchResults').innerHTML = html;
            log(\`Batch processing complete in \${totalTime}ms\`);
        }
        
        function log(message) {
            const logDiv = document.getElementById('log');
            const entry = document.createElement('div');
            entry.textContent = \`[\${new Date().toLocaleTimeString()}] \${message}\`;
            logDiv.appendChild(entry);
            logDiv.scrollTop = logDiv.scrollHeight;
        }
        
        // Start everything
        init().catch(error => {
            log('Error: ' + error.message);
            console.error(error);
        });
        
        // Cleanup on page unload
        window.addEventListener('beforeunload', () => {
            if (client) {
                client.terminate();
            }
        });
    </script>
</body>
</html>
`;

// Export for use in other modules
export { AMOSWorkerClient, exampleHTML };