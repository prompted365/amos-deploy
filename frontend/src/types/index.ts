export interface Agent {
  id: string;
  name: string;
  type: 'researcher' | 'coder' | 'analyst' | 'optimizer' | 'coordinator';
  status: 'idle' | 'active' | 'busy' | 'error';
  capabilities: string[];
  metrics: {
    tasksCompleted: number;
    successRate: number;
    avgResponseTime: number;
    cpuUsage: number;
    memoryUsage: number;
  };
  currentTask?: string;
  lastActivity?: string;
}

export interface NeuralNode {
  id: string;
  type: 'agent' | 'task' | 'memory' | 'neural';
  data: {
    label: string;
    status?: string;
    metrics?: any;
    agent?: Agent;
  };
  position: { x: number; y: number };
}

export interface NeuralEdge {
  id: string;
  source: string;
  target: string;
  type?: string;
  animated?: boolean;
  data?: {
    strength?: number;
    label?: string;
  };
}

export interface SwarmStatus {
  topology: 'mesh' | 'hierarchical' | 'ring' | 'star';
  agentCount: number;
  activeAgents: number;
  totalTasks: number;
  completedTasks: number;
  memoryUsage: number;
  performance: {
    avgResponseTime: number;
    throughput: number;
    errorRate: number;
  };
}

export interface Event {
  id: string;
  timestamp: string;
  type: 'system' | 'agent' | 'task' | 'error' | 'warning';
  level: 'info' | 'warning' | 'error' | 'critical';
  source: string;
  message: string;
  data?: any;
}

export interface Task {
  id: string;
  name: string;
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  priority: 'low' | 'medium' | 'high' | 'critical';
  assignedAgent?: string;
  progress: number;
  startTime?: string;
  endTime?: string;
  error?: string;
}

export interface CognitivePattern {
  type: 'convergent' | 'divergent' | 'lateral' | 'systems' | 'critical' | 'abstract';
  strength: number;
  usage: number;
  effectiveness: number;
}