import React, { useState } from 'react';
import { useAgents, useSpawnAgent } from '../hooks/useSwarm';
import { Agent } from '../types';
import { 
  Brain, 
  Code, 
  BarChart3, 
  Zap, 
  Network,
  Plus,
  Activity,
  AlertCircle,
  CheckCircle,
  Clock
} from 'lucide-react';

const agentTypeIcons = {
  researcher: Brain,
  coder: Code,
  analyst: BarChart3,
  optimizer: Zap,
  coordinator: Network,
};

const statusColors = {
  idle: 'text-gray-500',
  active: 'text-green-500',
  busy: 'text-yellow-500',
  error: 'text-red-500',
};

const statusIcons = {
  idle: Clock,
  active: CheckCircle,
  busy: Activity,
  error: AlertCircle,
};

interface AgentCardProps {
  agent: Agent;
}

const AgentCard: React.FC<AgentCardProps> = ({ agent }) => {
  const Icon = agentTypeIcons[agent.type] || Brain;
  const StatusIcon = statusIcons[agent.status] || Clock;

  return (
    <div className="amos-panel p-4 hover:border-gray-700 transition-colors">
      <div className="flex items-start justify-between mb-3">
        <div className="flex items-center space-x-3">
          <div className="p-2 bg-gray-800 rounded-lg">
            <Icon className="w-5 h-5 text-amos-agent" />
          </div>
          <div>
            <h3 className="font-semibold text-gray-100">{agent.name}</h3>
            <p className="text-xs text-gray-500">{agent.id}</p>
          </div>
        </div>
        <div className={`flex items-center space-x-1 ${statusColors[agent.status]}`}>
          <StatusIcon className="w-4 h-4" />
          <span className="text-xs capitalize">{agent.status}</span>
        </div>
      </div>

      {agent.currentTask && (
        <div className="mb-3 p-2 bg-gray-900 rounded text-xs text-gray-400">
          {agent.currentTask}
        </div>
      )}

      <div className="grid grid-cols-2 gap-2 text-xs">
        <div className="flex justify-between">
          <span className="text-gray-500">Tasks:</span>
          <span className="text-gray-300">{agent.metrics.tasksCompleted}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-500">Success:</span>
          <span className="text-gray-300">{(agent.metrics.successRate * 100).toFixed(0)}%</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-500">CPU:</span>
          <span className="text-gray-300">{agent.metrics.cpuUsage.toFixed(1)}%</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-500">Memory:</span>
          <span className="text-gray-300">{agent.metrics.memoryUsage.toFixed(1)}%</span>
        </div>
      </div>

      {agent.capabilities.length > 0 && (
        <div className="mt-3 flex flex-wrap gap-1">
          {agent.capabilities.map((cap, idx) => (
            <span key={idx} className="px-2 py-1 bg-gray-800 rounded text-xs text-gray-400">
              {cap}
            </span>
          ))}
        </div>
      )}
    </div>
  );
};

export const AgentPanel: React.FC = () => {
  const { data: agents = [] } = useAgents();
  const spawnAgentMutation = useSpawnAgent();
  const [showSpawnForm, setShowSpawnForm] = useState(false);
  const [selectedType, setSelectedType] = useState<Agent['type']>('researcher');

  const handleSpawnAgent = () => {
    spawnAgentMutation.mutate(
      { type: selectedType },
      {
        onSuccess: () => {
          setShowSpawnForm(false);
        },
      }
    );
  };

  const activeCount = agents.filter(a => a.status === 'active' || a.status === 'busy').length;

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h2 className="text-xl font-bold text-gray-100">Agents</h2>
          <p className="text-sm text-gray-500">
            {activeCount} active / {agents.length} total
          </p>
        </div>
        <button
          onClick={() => setShowSpawnForm(!showSpawnForm)}
          className="amos-button flex items-center space-x-2"
        >
          <Plus className="w-4 h-4" />
          <span>Spawn Agent</span>
        </button>
      </div>

      {showSpawnForm && (
        <div className="amos-panel p-4 mb-4">
          <h3 className="font-semibold mb-3">Spawn New Agent</h3>
          <div className="grid grid-cols-3 gap-2 mb-3">
            {Object.entries(agentTypeIcons).map(([type, Icon]) => (
              <button
                key={type}
                onClick={() => setSelectedType(type as Agent['type'])}
                className={`p-3 rounded-lg border-2 transition-colors ${
                  selectedType === type
                    ? 'border-amos-accent bg-gray-800'
                    : 'border-gray-700 hover:border-gray-600'
                }`}
              >
                <Icon className="w-5 h-5 mx-auto mb-1 text-gray-300" />
                <p className="text-xs capitalize">{type}</p>
              </button>
            ))}
          </div>
          <div className="flex space-x-2">
            <button
              onClick={handleSpawnAgent}
              disabled={spawnAgentMutation.isPending}
              className="amos-button flex-1"
            >
              {spawnAgentMutation.isPending ? 'Spawning...' : 'Spawn'}
            </button>
            <button
              onClick={() => setShowSpawnForm(false)}
              className="amos-button-secondary"
            >
              Cancel
            </button>
          </div>
        </div>
      )}

      <div className="flex-1 overflow-y-auto space-y-3">
        {agents.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <Network className="w-12 h-12 mx-auto mb-3 opacity-50" />
            <p>No agents spawned yet</p>
            <p className="text-sm mt-1">Click "Spawn Agent" to get started</p>
          </div>
        ) : (
          agents.map((agent) => (
            <AgentCard key={agent.id} agent={agent} />
          ))
        )}
      </div>
    </div>
  );
};