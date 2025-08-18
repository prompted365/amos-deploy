// Node.js Example for AMOS WASM
// Note: Requires Node.js with experimental WASM support

import { readFile } from 'fs/promises';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Load WASM module in Node.js
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function loadWASM() {
    // Read the WASM file
    const wasmPath = join(__dirname, '../pkg/amos_wasm_bg.wasm');
    const wasmBuffer = await readFile(wasmPath);
    
    // Import the JS bindings
    const { default: init, AMOSClient } = await import('../pkg/amos_wasm.js');
    
    // Initialize with the WASM buffer
    await init(wasmBuffer);
    
    return AMOSClient;
}

async function main() {
    console.log('Loading AMOS WASM module...');
    const AMOSClient = await loadWASM();
    
    // Create client
    const amos = new AMOSClient();
    console.log('AMOS client created');
    
    // Spawn some agents
    console.log('\nSpawning agents...');
    const trafficSeerId = await amos.spawnAgent('TrafficSeer');
    console.log(`- Traffic Seer spawned: ${trafficSeerId}`);
    
    const memoryWeaverId = await amos.spawnAgent('MemoryWeaver');
    console.log(`- Memory Weaver spawned: ${memoryWeaverId}`);
    
    const pathwaySculptorId = await amos.spawnAgent('PathwaySculptor');
    console.log(`- Pathway Sculptor spawned: ${pathwaySculptorId}`);
    
    // Get all agents
    const agents = await amos.getAgents();
    console.log(`\nTotal agents: ${agents.length}`);
    
    // Process some input
    console.log('\nProcessing neural input...');
    const inputs = [
        'analyze traffic patterns in the neural network',
        'remember this important information',
        'optimize pathway connections',
        'test system performance'
    ];
    
    for (const input of inputs) {
        console.log(`\nInput: "${input}"`);
        const result = await amos.processUserInput(input);
        console.log(`Output: ${result.output}`);
        console.log(`- Pathways activated: ${result.pathways_activated}`);
        console.log(`- Agents involved: ${result.agents_involved.length}`);
        console.log(`- Processing time: ${result.processing_time_ms}ms`);
    }
    
    // Trigger hormone bursts
    console.log('\nTriggering hormonal responses...');
    await amos.triggerHormonalBurst('Dopamine', 0.3);
    console.log('- Dopamine burst triggered');
    
    await amos.triggerHormonalBurst('Serotonin', 0.4);
    console.log('- Serotonin boost triggered');
    
    // Get hormone levels
    const hormoneLevels = await amos.getHormoneLevels();
    console.log('\nCurrent hormone levels:');
    for (const [hormone, level] of Object.entries(hormoneLevels)) {
        console.log(`- ${hormone}: ${(level * 100).toFixed(1)}%`);
    }
    
    // Strengthen some pathways
    console.log('\nStrengthening neural pathways...');
    const node1 = await amos.addNode('memory', {});
    const node2 = await amos.addNode('thinking', {});
    await amos.connectNodes(node1, node2, 0.5);
    console.log('- Connected memory node to thinking node');
    
    await amos.strengthenPathway(node1, node2, 0.2);
    console.log('- Strengthened pathway by 20%');
    
    // Get mesh status
    const status = await amos.getMeshStatus();
    console.log('\nMesh Status:');
    console.log(`- Active agents: ${status.active_agents}`);
    console.log(`- Total pathways: ${status.total_pathways}`);
    console.log(`- Total nodes: ${status.total_nodes}`);
    console.log(`- Memory usage: ${(status.memory_usage_bytes / 1024).toFixed(2)} KB`);
    console.log(`- Uptime: ${status.uptime_seconds} seconds`);
    
    // Simulate extended processing
    console.log('\nSimulating extended neural processing...');
    for (let i = 0; i < 5; i++) {
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        // Process with learning
        const learningInput = `learning iteration ${i + 1}`;
        const result = await amos.processUserInput(learningInput);
        
        // Reward learning with dopamine
        if (result.pathways_activated > 5) {
            await amos.triggerHormonalBurst('Dopamine', 0.1);
            console.log(`- Iteration ${i + 1}: Good performance, dopamine reward applied`);
        } else {
            console.log(`- Iteration ${i + 1}: Normal performance`);
        }
    }
    
    // Final state
    const finalState = await amos.debugDumpState();
    console.log('\nFinal System State:');
    console.log(`- Total agents: ${finalState.agents}`);
    console.log(`- Total pathways: ${finalState.pathways}`);
    console.log(`- Total nodes: ${finalState.nodes}`);
    console.log(`- Runtime: ${(finalState.uptime_ms / 1000).toFixed(2)} seconds`);
    
    console.log('\nAMOS demonstration complete!');
}

// Run the example
main().catch(console.error);