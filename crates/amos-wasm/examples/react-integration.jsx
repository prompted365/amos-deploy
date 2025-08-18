// React Integration Example for AMOS WASM
import React, { useState, useEffect, useRef } from 'react';
import init, { AMOSClient, AgentType, HormoneType } from '@amos/wasm';

// Custom hook for AMOS
function useAMOS() {
  const [isInitialized, setIsInitialized] = useState(false);
  const [agents, setAgents] = useState([]);
  const [meshStatus, setMeshStatus] = useState(null);
  const clientRef = useRef(null);

  useEffect(() => {
    async function initialize() {
      await init();
      clientRef.current = new AMOSClient();
      setIsInitialized(true);
    }
    initialize();
  }, []);

  const spawnAgent = async (agentType) => {
    if (!clientRef.current) return;
    
    const agentId = await clientRef.current.spawnAgent(agentType);
    const updatedAgents = await clientRef.current.getAgents();
    setAgents(updatedAgents);
    return agentId;
  };

  const processInput = async (input) => {
    if (!clientRef.current) return;
    return await clientRef.current.processUserInput(input);
  };

  const updateMeshStatus = async () => {
    if (!clientRef.current) return;
    const status = await clientRef.current.getMeshStatus();
    setMeshStatus(status);
  };

  const triggerHormone = async (hormone, intensity) => {
    if (!clientRef.current) return;
    await clientRef.current.triggerHormonalBurst(hormone, intensity);
  };

  return {
    isInitialized,
    agents,
    meshStatus,
    spawnAgent,
    processInput,
    updateMeshStatus,
    triggerHormone,
    client: clientRef.current
  };
}

// Agent Card Component
function AgentCard({ agent }) {
  return (
    <div className="agent-card">
      <h3>{agent.name}</h3>
      <p>Type: {agent.agent_type}</p>
      <p>State: {agent.state}</p>
      <p>ID: {agent.id.substring(0, 8)}...</p>
    </div>
  );
}

// Main AMOS Demo Component
export default function AMOSDemo() {
  const {
    isInitialized,
    agents,
    meshStatus,
    spawnAgent,
    processInput,
    updateMeshStatus,
    triggerHormone
  } = useAMOS();

  const [userInput, setUserInput] = useState('');
  const [processResult, setProcessResult] = useState(null);
  const [isProcessing, setIsProcessing] = useState(false);

  useEffect(() => {
    if (isInitialized) {
      // Update mesh status every 2 seconds
      const interval = setInterval(updateMeshStatus, 2000);
      return () => clearInterval(interval);
    }
  }, [isInitialized]);

  const handleSpawnAgent = async (type) => {
    try {
      await spawnAgent(type);
    } catch (error) {
      console.error('Error spawning agent:', error);
    }
  };

  const handleProcessInput = async () => {
    if (!userInput.trim()) return;
    
    setIsProcessing(true);
    try {
      const result = await processInput(userInput);
      setProcessResult(result);
    } catch (error) {
      console.error('Error processing input:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleHormoneBurst = async (hormone, intensity) => {
    try {
      await triggerHormone(hormone, intensity);
    } catch (error) {
      console.error('Error triggering hormone:', error);
    }
  };

  if (!isInitialized) {
    return <div>Loading AMOS...</div>;
  }

  return (
    <div className="amos-demo">
      <h1>AMOS Biological Mesh Network</h1>
      
      {/* Mesh Status */}
      {meshStatus && (
        <div className="mesh-status">
          <h2>Mesh Status</h2>
          <div className="status-grid">
            <div>Active Agents: {meshStatus.active_agents}</div>
            <div>Total Pathways: {meshStatus.total_pathways}</div>
            <div>Total Nodes: {meshStatus.total_nodes}</div>
            <div>Memory: {(meshStatus.memory_usage_bytes / 1024).toFixed(2)} KB</div>
            <div>Uptime: {meshStatus.uptime_seconds}s</div>
          </div>
        </div>
      )}

      {/* Agent Spawning */}
      <div className="agent-controls">
        <h2>Spawn Agents</h2>
        <div className="button-group">
          {Object.keys(AgentType).map(type => (
            <button
              key={type}
              onClick={() => handleSpawnAgent(AgentType[type])}
            >
              Spawn {type}
            </button>
          ))}
        </div>
      </div>

      {/* Active Agents */}
      <div className="agents-section">
        <h2>Active Agents ({agents.length})</h2>
        <div className="agents-grid">
          {agents.map(agent => (
            <AgentCard key={agent.id} agent={agent} />
          ))}
        </div>
      </div>

      {/* Neural Processing */}
      <div className="processing-section">
        <h2>Neural Processing</h2>
        <div className="input-group">
          <input
            type="text"
            value={userInput}
            onChange={(e) => setUserInput(e.target.value)}
            placeholder="Enter text to process..."
            onKeyPress={(e) => e.key === 'Enter' && handleProcessInput()}
          />
          <button 
            onClick={handleProcessInput}
            disabled={isProcessing}
          >
            {isProcessing ? 'Processing...' : 'Process'}
          </button>
        </div>
        
        {processResult && (
          <div className="process-result">
            <h3>Result:</h3>
            <p>{processResult.output}</p>
            <div className="result-stats">
              <span>Pathways activated: {processResult.pathways_activated}</span>
              <span>Agents involved: {processResult.agents_involved.length}</span>
              <span>Processing time: {processResult.processing_time_ms}ms</span>
            </div>
          </div>
        )}
      </div>

      {/* Hormone Controls */}
      <div className="hormone-controls">
        <h2>Hormonal System</h2>
        <div className="hormone-buttons">
          <button onClick={() => handleHormoneBurst(HormoneType.Dopamine, 0.3)}>
            🎯 Dopamine Burst
          </button>
          <button onClick={() => handleHormoneBurst(HormoneType.Serotonin, 0.4)}>
            😊 Serotonin Boost
          </button>
          <button onClick={() => handleHormoneBurst(HormoneType.Cortisol, 0.2)}>
            😰 Cortisol Spike
          </button>
          <button onClick={() => handleHormoneBurst(HormoneType.Oxytocin, 0.5)}>
            💝 Oxytocin Release
          </button>
          <button onClick={() => handleHormoneBurst(HormoneType.Adrenaline, 0.3)}>
            ⚡ Adrenaline Rush
          </button>
        </div>
      </div>
    </div>
  );
}

// CSS Styles (can be moved to separate file)
const styles = `
.amos-demo {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.mesh-status {
  background: #f0f4f8;
  padding: 20px;
  border-radius: 8px;
  margin-bottom: 20px;
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 10px;
  margin-top: 10px;
}

.status-grid > div {
  background: white;
  padding: 10px;
  border-radius: 4px;
  text-align: center;
}

.button-group {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 10px;
}

button {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  background: #0066cc;
  color: white;
  cursor: pointer;
  transition: background 0.2s;
}

button:hover {
  background: #0052a3;
}

button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.agents-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 15px;
  margin-top: 15px;
}

.agent-card {
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 15px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.agent-card h3 {
  margin: 0 0 10px 0;
  color: #333;
}

.agent-card p {
  margin: 5px 0;
  color: #666;
  font-size: 14px;
}

.input-group {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.input-group input {
  flex: 1;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 16px;
}

.process-result {
  background: #f9f9f9;
  padding: 15px;
  border-radius: 4px;
  margin-top: 15px;
}

.result-stats {
  display: flex;
  gap: 20px;
  margin-top: 10px;
  font-size: 14px;
  color: #666;
}

.hormone-buttons {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 10px;
  margin-top: 15px;
}

.hormone-buttons button {
  background: #6b46c1;
  padding: 15px;
  font-size: 16px;
}

.hormone-buttons button:hover {
  background: #553c9a;
}
`;