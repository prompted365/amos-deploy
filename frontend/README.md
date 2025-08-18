# AMOS Frontend

Modern React dashboard for the Adaptive Multi-agent Orchestration System.

## Features

- **Neural Network Visualization**: Real-time visualization of agent topology using react-flow
- **Agent Management**: Spawn, monitor, and manage intelligent agents
- **Live Event Stream**: WebSocket-based real-time event monitoring
- **Performance Dashboard**: Comprehensive metrics and system health monitoring
- **Dark Theme UI**: Modern, responsive design with Tailwind CSS

## Tech Stack

- **React 18** with TypeScript
- **Vite** for fast development and building
- **React Flow** for neural network visualization
- **React Query** for server state management
- **Socket.io** for real-time WebSocket communication
- **Tailwind CSS** for styling
- **Lucide React** for icons

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Environment Variables

Create a `.env` file:

```env
VITE_API_URL=http://localhost:8000
VITE_WS_URL=ws://localhost:8000
```

## Project Structure

```
src/
├── components/       # React components
│   ├── NeuralNetworkView.tsx
│   ├── AgentPanel.tsx
│   ├── EventStream.tsx
│   └── StatusDashboard.tsx
├── hooks/           # Custom React hooks
│   └── useSwarm.ts
├── utils/           # Utilities and API client
│   └── api.ts
├── types/           # TypeScript type definitions
│   └── index.ts
├── App.tsx          # Main application component
├── main.tsx         # Application entry point
└── index.css        # Global styles
```

## API Integration

The frontend connects to the AMOS backend through:
- REST API endpoints (via axios)
- WebSocket for real-time updates

Proxy configuration in `vite.config.ts` handles API routing during development.