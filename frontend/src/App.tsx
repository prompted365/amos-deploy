import React, { useState, useEffect } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { NeuralNetworkView } from './components/NeuralNetworkView';
import { AgentPanel } from './components/AgentPanel';
import { EventStream } from './components/EventStream';
import { StatusDashboard } from './components/StatusDashboard';
import { useInitSwarm, useSwarmStatus } from './hooks/useSwarm';
import { wsEvents } from './utils/api';
import {
  Brain,
  Activity,
  Terminal,
  LayoutDashboard,
  Loader2,
  AlertCircle,
} from 'lucide-react';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000,
      refetchInterval: false,
    },
  },
});

function AppContent() {
  const [activeView, setActiveView] = useState<'neural' | 'dashboard'>('neural');
  const { data: swarmStatus, isLoading, error } = useSwarmStatus();
  const initSwarmMutation = useInitSwarm();

  useEffect(() => {
    // Initialize swarm if not already initialized
    if (!isLoading && !swarmStatus && !error) {
      initSwarmMutation.mutate({ topology: 'mesh', maxAgents: 8 });
    }
  }, [isLoading, swarmStatus, error]);

  useEffect(() => {
    // Cleanup WebSocket on unmount
    return () => {
      wsEvents.cleanup();
    };
  }, []);

  if (isLoading) {
    return (
      <div className="min-h-screen bg-amos-darker flex items-center justify-center">
        <div className="text-center">
          <Loader2 className="w-12 h-12 animate-spin text-amos-accent mx-auto mb-4" />
          <p className="text-gray-400">Initializing AMOS...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-amos-darker flex items-center justify-center">
        <div className="text-center">
          <AlertCircle className="w-12 h-12 text-red-500 mx-auto mb-4" />
          <p className="text-gray-400 mb-4">Failed to connect to AMOS backend</p>
          <button
            onClick={() => window.location.reload()}
            className="amos-button"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-amos-darker text-gray-100">
      {/* Header */}
      <header className="bg-amos-dark border-b border-gray-800">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="p-2 bg-gradient-to-br from-amos-accent to-amos-neural rounded-lg">
                <Brain className="w-6 h-6 text-black" />
              </div>
              <div>
                <h1 className="text-2xl font-bold bg-gradient-to-r from-amos-accent to-amos-agent bg-clip-text text-transparent">
                  AMOS
                </h1>
                <p className="text-xs text-gray-500">Adaptive Multi-agent Orchestration System</p>
              </div>
            </div>
            
            <div className="flex items-center space-x-4">
              <div className="flex bg-gray-900 rounded-lg p-1">
                <button
                  onClick={() => setActiveView('neural')}
                  className={`px-3 py-1 rounded flex items-center space-x-2 transition-colors ${
                    activeView === 'neural' 
                      ? 'bg-amos-accent text-black' 
                      : 'text-gray-400 hover:text-gray-200'
                  }`}
                >
                  <Activity className="w-4 h-4" />
                  <span>Neural View</span>
                </button>
                <button
                  onClick={() => setActiveView('dashboard')}
                  className={`px-3 py-1 rounded flex items-center space-x-2 transition-colors ${
                    activeView === 'dashboard' 
                      ? 'bg-amos-accent text-black' 
                      : 'text-gray-400 hover:text-gray-200'
                  }`}
                >
                  <LayoutDashboard className="w-4 h-4" />
                  <span>Dashboard</span>
                </button>
              </div>
              
              {swarmStatus && (
                <div className="flex items-center space-x-2 text-sm">
                  <div className={`w-2 h-2 rounded-full ${
                    swarmStatus.activeAgents > 0 ? 'bg-green-500' : 'bg-gray-500'
                  } animate-pulse`} />
                  <span className="text-gray-400">
                    {swarmStatus.topology} • {swarmStatus.activeAgents} agents
                  </span>
                </div>
              )}
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-6">
        {activeView === 'neural' ? (
          <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 h-[calc(100vh-8rem)]">
            {/* Neural Network Visualization */}
            <div className="lg:col-span-3 h-full">
              <div className="amos-panel h-full p-4">
                <h2 className="text-xl font-bold mb-4 flex items-center">
                  <Activity className="w-5 h-5 mr-2 text-amos-accent" />
                  Neural Network Topology
                </h2>
                <div className="h-[calc(100%-3rem)]">
                  <NeuralNetworkView />
                </div>
              </div>
            </div>

            {/* Agent Panel */}
            <div className="h-full">
              <div className="amos-panel h-full p-4">
                <AgentPanel />
              </div>
            </div>
          </div>
        ) : (
          <div className="space-y-6">
            {/* Status Dashboard */}
            <StatusDashboard />

            {/* Event Stream */}
            <div className="amos-panel p-4 h-96">
              <EventStream />
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AppContent />
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  );
}

export default App;