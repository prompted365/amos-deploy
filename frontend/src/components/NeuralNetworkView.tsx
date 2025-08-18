import React, { useCallback, useEffect, useState } from 'react';
import ReactFlow, {
  Node,
  Edge,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
  BackgroundVariant,
} from 'react-flow-renderer';
import { useAgents } from '../hooks/useSwarm';
import { Agent, NeuralNode, NeuralEdge } from '../types';

const nodeTypes = {
  agent: ({ data }: { data: any }) => (
    <div className="px-4 py-2 shadow-md rounded-md bg-gray-900 border-2 border-gray-700 min-w-[150px]">
      <div className="flex items-center justify-between mb-1">
        <div className={`w-2 h-2 rounded-full ${
          data.agent?.status === 'active' ? 'bg-green-500' :
          data.agent?.status === 'busy' ? 'bg-yellow-500' :
          data.agent?.status === 'error' ? 'bg-red-500' :
          'bg-gray-500'
        }`} />
        <span className="text-xs text-gray-400">{data.agent?.type}</span>
      </div>
      <div className="font-bold text-sm text-gray-100">{data.label}</div>
      {data.agent?.currentTask && (
        <div className="text-xs text-gray-400 mt-1 truncate">
          {data.agent.currentTask}
        </div>
      )}
      {data.agent?.metrics && (
        <div className="flex justify-between mt-2 text-xs">
          <span className="text-gray-500">CPU: {data.agent.metrics.cpuUsage.toFixed(1)}%</span>
          <span className="text-gray-500">Mem: {data.agent.metrics.memoryUsage.toFixed(1)}%</span>
        </div>
      )}
    </div>
  ),
  
  task: ({ data }: { data: any }) => (
    <div className="px-3 py-2 shadow-md rounded-md bg-blue-900 border-2 border-blue-700">
      <div className="font-bold text-sm text-blue-100">{data.label}</div>
      {data.status && (
        <div className="text-xs text-blue-300 mt-1">{data.status}</div>
      )}
    </div>
  ),
  
  neural: ({ data }: { data: any }) => (
    <div className="px-3 py-2 shadow-md rounded-full bg-purple-900 border-2 border-purple-700 text-center">
      <div className="font-bold text-sm text-purple-100">{data.label}</div>
    </div>
  ),
};

export const NeuralNetworkView: React.FC = () => {
  const { data: agents = [] } = useAgents();
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  // Convert agents to nodes
  useEffect(() => {
    const agentNodes: NeuralNode[] = agents.map((agent, index) => ({
      id: agent.id,
      type: 'agent',
      position: {
        x: 150 + (index % 3) * 250,
        y: 100 + Math.floor(index / 3) * 150,
      },
      data: {
        label: agent.name,
        agent,
      },
    }));

    // Add a central coordination node
    const coordinationNode: NeuralNode = {
      id: 'coordination-center',
      type: 'neural',
      position: { x: 400, y: 50 },
      data: { label: 'Coordination' },
    };

    setNodes([coordinationNode, ...agentNodes]);

    // Create edges from coordination to agents
    const newEdges: NeuralEdge[] = agentNodes.map((node) => ({
      id: `e-coord-${node.id}`,
      source: 'coordination-center',
      target: node.id,
      type: 'smoothstep',
      animated: node.data.agent?.status === 'active',
      data: {
        strength: node.data.agent?.metrics?.successRate || 0,
      },
    }));

    // Add inter-agent connections for mesh topology
    if (agents.length > 1) {
      for (let i = 0; i < agents.length - 1; i++) {
        for (let j = i + 1; j < Math.min(i + 3, agents.length); j++) {
          newEdges.push({
            id: `e-${agents[i].id}-${agents[j].id}`,
            source: agents[i].id,
            target: agents[j].id,
            type: 'straight',
            animated: false,
            style: { stroke: '#444', strokeWidth: 1 },
          } as NeuralEdge);
        }
      }
    }

    setEdges(newEdges);
  }, [agents, setNodes, setEdges]);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  return (
    <div className="w-full h-full bg-amos-darker rounded-lg">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        nodeTypes={nodeTypes}
        fitView
        className="bg-amos-darker"
      >
        <Background
          color="#333"
          variant={BackgroundVariant.Dots}
          gap={20}
          size={1}
        />
        <MiniMap
          nodeColor={(node) => {
            if (node.type === 'agent') return '#1f2937';
            if (node.type === 'task') return '#1e3a8a';
            return '#581c87';
          }}
          style={{
            backgroundColor: '#0a0a0a',
            border: '1px solid #333',
          }}
        />
        <Controls />
      </ReactFlow>
    </div>
  );
};