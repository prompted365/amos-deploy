import React, { useEffect, useState, useRef } from 'react';
import { format } from 'date-fns';
import { wsEvents } from '../utils/api';
import { Event } from '../types';
import {
  Info,
  AlertTriangle,
  AlertCircle,
  CheckCircle,
  Activity,
  Terminal,
  Filter,
} from 'lucide-react';

const eventIcons = {
  system: Terminal,
  agent: Activity,
  task: CheckCircle,
  error: AlertCircle,
  warning: AlertTriangle,
};

const levelColors = {
  info: 'text-blue-500',
  warning: 'text-yellow-500',
  error: 'text-red-500',
  critical: 'text-red-600',
};

const levelBgColors = {
  info: 'bg-blue-900/20',
  warning: 'bg-yellow-900/20',
  error: 'bg-red-900/20',
  critical: 'bg-red-900/30',
};

export const EventStream: React.FC = () => {
  const [events, setEvents] = useState<Event[]>([]);
  const [filter, setFilter] = useState<'all' | Event['type']>('all');
  const [autoScroll, setAutoScroll] = useState(true);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Subscribe to WebSocket events
    const handleEvent = (data: any) => {
      const event: Event = {
        id: Date.now().toString(),
        timestamp: new Date().toISOString(),
        type: data.type || 'system',
        level: data.level || 'info',
        source: data.source || 'system',
        message: data.message || JSON.stringify(data),
        data: data.data,
      };

      setEvents((prev) => [...prev.slice(-99), event]); // Keep last 100 events
    };

    wsEvents.onSystemEvent(handleEvent);
    wsEvents.onAgentUpdate((data) => 
      handleEvent({ ...data, type: 'agent', source: data.agentId })
    );
    wsEvents.onTaskUpdate((data) => 
      handleEvent({ ...data, type: 'task', source: data.taskId })
    );
    wsEvents.onNeuralUpdate((data) => 
      handleEvent({ ...data, type: 'system', source: 'neural', message: 'Neural network updated' })
    );

    return () => {
      // Cleanup is handled by wsEvents.cleanup() in main App
    };
  }, []);

  useEffect(() => {
    if (autoScroll && containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [events, autoScroll]);

  const filteredEvents = filter === 'all' 
    ? events 
    : events.filter(e => e.type === filter);

  const eventTypes: Array<'all' | Event['type']> = ['all', 'system', 'agent', 'task', 'error', 'warning'];

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-bold text-gray-100">Event Stream</h2>
        <div className="flex items-center space-x-2">
          <button
            onClick={() => setAutoScroll(!autoScroll)}
            className={`px-3 py-1 rounded text-sm ${
              autoScroll 
                ? 'bg-amos-accent text-black' 
                : 'bg-gray-800 text-gray-300'
            }`}
          >
            Auto-scroll
          </button>
          <button
            onClick={() => setEvents([])}
            className="px-3 py-1 bg-gray-800 hover:bg-gray-700 rounded text-sm"
          >
            Clear
          </button>
        </div>
      </div>

      <div className="flex space-x-1 mb-3 overflow-x-auto">
        {eventTypes.map((type) => (
          <button
            key={type}
            onClick={() => setFilter(type)}
            className={`px-3 py-1 rounded-full text-xs whitespace-nowrap transition-colors ${
              filter === type
                ? 'bg-amos-accent text-black'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
            }`}
          >
            {type === 'all' ? 'All' : type.charAt(0).toUpperCase() + type.slice(1)}
            {type !== 'all' && (
              <span className="ml-1 text-xs opacity-70">
                ({events.filter(e => e.type === type).length})
              </span>
            )}
          </button>
        ))}
      </div>

      <div
        ref={containerRef}
        className="flex-1 overflow-y-auto space-y-1 font-mono text-xs"
      >
        {filteredEvents.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <Activity className="w-12 h-12 mx-auto mb-3 opacity-50" />
            <p>No events yet</p>
            <p className="text-sm mt-1">Events will appear here as they occur</p>
          </div>
        ) : (
          filteredEvents.map((event) => {
            const Icon = eventIcons[event.type] || Info;
            return (
              <div
                key={event.id}
                className={`p-2 rounded ${levelBgColors[event.level]} border border-gray-800`}
              >
                <div className="flex items-start space-x-2">
                  <Icon className={`w-4 h-4 mt-0.5 ${levelColors[event.level]}`} />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center space-x-2 mb-1">
                      <span className="text-gray-500">
                        {format(new Date(event.timestamp), 'HH:mm:ss.SSS')}
                      </span>
                      <span className={`px-2 py-0.5 rounded text-xs ${
                        event.type === 'system' ? 'bg-gray-800' :
                        event.type === 'agent' ? 'bg-blue-900' :
                        event.type === 'task' ? 'bg-green-900' :
                        'bg-red-900'
                      }`}>
                        {event.source}
                      </span>
                    </div>
                    <p className="text-gray-300 break-words">{event.message}</p>
                    {event.data && (
                      <pre className="mt-1 p-1 bg-gray-900 rounded text-xs text-gray-500 overflow-x-auto">
                        {JSON.stringify(event.data, null, 2)}
                      </pre>
                    )}
                  </div>
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};