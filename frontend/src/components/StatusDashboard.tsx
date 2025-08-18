import React from 'react';
import { useSwarmStatus, useMemoryUsage, useNeuralStatus, useCognitivePatterns } from '../hooks/useSwarm';
import {
  Activity,
  Brain,
  Cpu,
  HardDrive,
  Network,
  TrendingUp,
  Zap,
  BarChart3,
} from 'lucide-react';

interface MetricCardProps {
  title: string;
  value: string | number;
  subtitle?: string;
  icon: React.ComponentType<{ className?: string }>;
  trend?: number;
  color?: string;
}

const MetricCard: React.FC<MetricCardProps> = ({ 
  title, 
  value, 
  subtitle, 
  icon: Icon, 
  trend,
  color = 'text-amos-accent' 
}) => (
  <div className="amos-panel p-4">
    <div className="flex items-start justify-between mb-2">
      <div>
        <p className="text-xs text-gray-500 uppercase">{title}</p>
        <p className={`text-2xl font-bold ${color}`}>{value}</p>
        {subtitle && <p className="text-xs text-gray-400 mt-1">{subtitle}</p>}
      </div>
      <div className="p-2 bg-gray-800 rounded-lg">
        <Icon className="w-5 h-5 text-gray-400" />
      </div>
    </div>
    {trend !== undefined && (
      <div className="flex items-center text-xs">
        <TrendingUp className={`w-3 h-3 mr-1 ${trend > 0 ? 'text-green-500' : 'text-red-500'}`} />
        <span className={trend > 0 ? 'text-green-500' : 'text-red-500'}>
          {trend > 0 ? '+' : ''}{trend.toFixed(1)}%
        </span>
      </div>
    )}
  </div>
);

export const StatusDashboard: React.FC = () => {
  const { data: swarmStatus } = useSwarmStatus();
  const { data: memoryUsage } = useMemoryUsage();
  const { data: neuralStatus } = useNeuralStatus();
  const { data: patterns } = useCognitivePatterns();

  if (!swarmStatus) {
    return (
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 animate-pulse">
        {[...Array(8)].map((_, i) => (
          <div key={i} className="amos-panel h-24 bg-gray-900" />
        ))}
      </div>
    );
  }

  const taskCompletionRate = swarmStatus.totalTasks > 0 
    ? (swarmStatus.completedTasks / swarmStatus.totalTasks * 100).toFixed(1)
    : '0';

  const activePercentage = swarmStatus.agentCount > 0
    ? (swarmStatus.activeAgents / swarmStatus.agentCount * 100).toFixed(1)
    : '0';

  return (
    <div className="space-y-4">
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <MetricCard
          title="Active Agents"
          value={`${swarmStatus.activeAgents}/${swarmStatus.agentCount}`}
          subtitle={`${activePercentage}% utilization`}
          icon={Network}
          color="text-amos-agent"
        />
        
        <MetricCard
          title="Task Completion"
          value={`${taskCompletionRate}%`}
          subtitle={`${swarmStatus.completedTasks}/${swarmStatus.totalTasks} tasks`}
          icon={Activity}
          trend={5.2}
        />
        
        <MetricCard
          title="Avg Response Time"
          value={`${swarmStatus.performance.avgResponseTime.toFixed(0)}ms`}
          subtitle="Last 5 minutes"
          icon={Zap}
          color="text-yellow-500"
          trend={-12.3}
        />
        
        <MetricCard
          title="Error Rate"
          value={`${(swarmStatus.performance.errorRate * 100).toFixed(1)}%`}
          subtitle="System health"
          icon={BarChart3}
          color={swarmStatus.performance.errorRate < 0.05 ? 'text-green-500' : 'text-red-500'}
        />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <div className="amos-panel p-4">
          <h3 className="font-semibold mb-3 flex items-center">
            <Brain className="w-4 h-4 mr-2 text-amos-neural" />
            Neural Status
          </h3>
          {neuralStatus ? (
            <div className="space-y-2">
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">Active Models:</span>
                <span className="text-gray-300">{neuralStatus.activeModels || 0}</span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">Training Progress:</span>
                <span className="text-gray-300">{neuralStatus.trainingProgress || 0}%</span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">Accuracy:</span>
                <span className="text-gray-300">{neuralStatus.accuracy || 0}%</span>
              </div>
            </div>
          ) : (
            <p className="text-sm text-gray-500">No neural data available</p>
          )}
        </div>

        <div className="amos-panel p-4">
          <h3 className="font-semibold mb-3 flex items-center">
            <HardDrive className="w-4 h-4 mr-2 text-blue-500" />
            Memory Usage
          </h3>
          {memoryUsage ? (
            <div className="space-y-2">
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">System:</span>
                <span className="text-gray-300">{memoryUsage.system || '0 MB'}</span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">Agents:</span>
                <span className="text-gray-300">{memoryUsage.agents || '0 MB'}</span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">Cache:</span>
                <span className="text-gray-300">{memoryUsage.cache || '0 MB'}</span>
              </div>
              <div className="w-full bg-gray-800 rounded-full h-2 mt-2">
                <div 
                  className="bg-amos-accent h-2 rounded-full transition-all duration-300"
                  style={{ width: `${memoryUsage.percentage || 0}%` }}
                />
              </div>
            </div>
          ) : (
            <p className="text-sm text-gray-500">Loading memory data...</p>
          )}
        </div>

        <div className="amos-panel p-4">
          <h3 className="font-semibold mb-3 flex items-center">
            <Cpu className="w-4 h-4 mr-2 text-purple-500" />
            Cognitive Patterns
          </h3>
          {patterns && patterns.length > 0 ? (
            <div className="space-y-2">
              {patterns.slice(0, 3).map((pattern: any, idx: number) => (
                <div key={idx} className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 capitalize">{pattern.type}:</span>
                  <div className="flex items-center space-x-2">
                    <div className="w-16 bg-gray-800 rounded-full h-1.5">
                      <div 
                        className="bg-purple-500 h-1.5 rounded-full"
                        style={{ width: `${pattern.effectiveness * 100}%` }}
                      />
                    </div>
                    <span className="text-gray-300 text-xs">
                      {(pattern.effectiveness * 100).toFixed(0)}%
                    </span>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <p className="text-sm text-gray-500">No pattern data available</p>
          )}
        </div>
      </div>
    </div>
  );
};