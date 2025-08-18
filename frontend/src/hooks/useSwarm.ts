import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { swarmAPI } from '../utils/api';
import { SwarmStatus, Agent, Task } from '../types';

export const useSwarmStatus = () => {
  return useQuery<SwarmStatus>({
    queryKey: ['swarmStatus'],
    queryFn: async () => {
      const response = await swarmAPI.getStatus();
      return response.data;
    },
    refetchInterval: 2000, // Poll every 2 seconds
  });
};

export const useAgents = (filter?: string) => {
  return useQuery<Agent[]>({
    queryKey: ['agents', filter],
    queryFn: async () => {
      const response = await swarmAPI.listAgents(filter);
      return response.data;
    },
    refetchInterval: 3000,
  });
};

export const useInitSwarm = () => {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ topology, maxAgents }: { topology: string; maxAgents?: number }) =>
      swarmAPI.initSwarm(topology, maxAgents),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['swarmStatus'] });
      queryClient.invalidateQueries({ queryKey: ['agents'] });
    },
  });
};

export const useSpawnAgent = () => {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ type, capabilities }: { type: string; capabilities?: string[] }) =>
      swarmAPI.spawnAgent(type, capabilities),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['agents'] });
    },
  });
};

export const useOrchestrateTask = () => {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ task, priority, strategy }: { 
      task: string; 
      priority?: string; 
      strategy?: string 
    }) => swarmAPI.orchestrateTask(task, priority, strategy),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
};

export const useTaskStatus = (taskId?: string) => {
  return useQuery<Task[]>({
    queryKey: ['tasks', taskId],
    queryFn: async () => {
      const response = await swarmAPI.getTaskStatus(taskId);
      return response.data;
    },
    refetchInterval: 1000,
    enabled: true,
  });
};

export const useNeuralStatus = () => {
  return useQuery({
    queryKey: ['neuralStatus'],
    queryFn: async () => {
      const response = await swarmAPI.getNeuralStatus();
      return response.data;
    },
    refetchInterval: 5000,
  });
};

export const useCognitivePatterns = () => {
  return useQuery({
    queryKey: ['cognitivePatterns'],
    queryFn: async () => {
      const response = await swarmAPI.getCognitivePatterns();
      return response.data;
    },
    refetchInterval: 10000,
  });
};

export const useMemoryUsage = () => {
  return useQuery({
    queryKey: ['memoryUsage'],
    queryFn: async () => {
      const response = await swarmAPI.getMemoryUsage();
      return response.data;
    },
    refetchInterval: 5000,
  });
};