import axios from 'axios';
import { io, Socket } from 'socket.io-client';

// Create axios instance
export const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

// WebSocket connection
let socket: Socket | null = null;

export const getSocket = (): Socket => {
  if (!socket) {
    socket = io('/', {
      path: '/ws',
      transports: ['websocket'],
    });
  }
  return socket;
};

// API endpoints
export const swarmAPI = {
  // Swarm management
  initSwarm: (topology: string, maxAgents?: number) =>
    api.post('/swarm/init', { topology, maxAgents }),
  
  getStatus: () => api.get('/swarm/status'),
  
  // Agent management
  spawnAgent: (type: string, capabilities?: string[]) =>
    api.post('/agents/spawn', { type, capabilities }),
  
  listAgents: (filter?: string) =>
    api.get('/agents/list', { params: { filter } }),
  
  getAgentMetrics: (agentId?: string) =>
    api.get('/agents/metrics', { params: { agentId } }),
  
  // Task management
  orchestrateTask: (task: string, priority?: string, strategy?: string) =>
    api.post('/task/orchestrate', { task, priority, strategy }),
  
  getTaskStatus: (taskId?: string) =>
    api.get('/task/status', { params: { taskId } }),
  
  getTaskResults: (taskId: string) =>
    api.get(`/task/results/${taskId}`),
  
  // Neural features
  getNeuralStatus: () => api.get('/neural/status'),
  
  trainNeural: (iterations?: number) =>
    api.post('/neural/train', { iterations }),
  
  getCognitivePatterns: () => api.get('/neural/patterns'),
  
  // Memory
  getMemoryUsage: () => api.get('/memory/usage'),
  
  // Performance
  runBenchmark: (type?: string) =>
    api.post('/benchmark/run', { type }),
  
  getFeatures: () => api.get('/features/detect'),
};

// WebSocket event types
export type WSEventHandler = (data: any) => void;

export const wsEvents = {
  onAgentUpdate: (handler: WSEventHandler) => {
    getSocket().on('agent:update', handler);
  },
  
  onTaskUpdate: (handler: WSEventHandler) => {
    getSocket().on('task:update', handler);
  },
  
  onSystemEvent: (handler: WSEventHandler) => {
    getSocket().on('system:event', handler);
  },
  
  onNeuralUpdate: (handler: WSEventHandler) => {
    getSocket().on('neural:update', handler);
  },
  
  onPerformanceMetrics: (handler: WSEventHandler) => {
    getSocket().on('performance:metrics', handler);
  },
  
  cleanup: () => {
    if (socket) {
      socket.disconnect();
      socket = null;
    }
  },
};