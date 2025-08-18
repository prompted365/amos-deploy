# AMOS Railway Deployment

This directory contains everything needed to deploy AMOS to Railway with a React frontend.

## 🚀 Quick Start

### 1. Test Locally First

```bash
# Check prerequisites
./deploy.sh check

# Build and test locally
./deploy.sh test
```

Visit http://localhost:8080 to see the AMOS dashboard.

### 2. Deploy to Railway

```bash
# Install Railway CLI if needed
npm install -g @railway/cli

# Deploy to Railway
./deploy.sh deploy
```

## 📁 Project Structure

```
amos-deploy/
├── backend/          # Axum API server
│   ├── src/
│   │   └── main.rs   # API endpoints & WebSocket
│   └── Cargo.toml    # Rust dependencies
├── frontend/         # React dashboard
│   ├── src/
│   │   ├── App.tsx   # Main dashboard
│   │   └── components/
│   └── package.json  # Node dependencies
├── docker/           # Docker configuration
├── config/           # Environment configs
├── Dockerfile        # Multi-stage build
├── docker-compose.yml # Local testing
├── deploy.sh         # Deployment script
└── README.md         # This file
```

## 🎯 Features

### Backend (Axum)
- ✅ Health check endpoint
- ✅ Neural network status API
- ✅ Agent management (spawn/list)
- ✅ WebSocket for real-time events
- ✅ Static file serving
- ✅ CORS enabled

### Frontend (React)
- ✅ Neural network visualization
- ✅ Agent management panel
- ✅ Real-time event stream
- ✅ Dark theme UI
- ✅ Responsive design

### Deployment
- ✅ Multi-stage Docker build
- ✅ Railway configuration
- ✅ Environment variables
- ✅ Health checks
- ✅ Auto-scaling ready

## 🔧 Configuration

### Environment Variables

Create a `.env` file based on `.env.example`:

```bash
# Required
PORT=8080
RAILWAY_ENVIRONMENT=production

# Optional
AMOS_LOG_LEVEL=info
AMOS_MAX_AGENTS=50
```

### API Endpoints

- `GET /api/health` - Health check
- `GET /api/neural/status` - Neural network status
- `POST /api/agents/spawn` - Spawn new agent
- `GET /api/agents/list` - List all agents
- `WS /ws` - WebSocket connection

## 🐛 Troubleshooting

### Build Issues
- Ensure Rust 1.83+ is installed
- Check that all AMOS crates are in the correct location
- Verify Docker is running

### Runtime Issues
- Check logs: `docker-compose logs`
- Verify port 8080 is not in use
- Ensure environment variables are set

### Railway Issues
- Verify Railway CLI is logged in: `railway login`
- Check Railway dashboard for deployment logs
- Ensure billing is set up on Railway

## 📊 Monitoring

Once deployed, monitor your app:

```bash
# View logs
railway logs

# Check status
railway status

# Open dashboard
railway open
```

## 🚀 Next Steps

1. **Add Authentication**: Implement JWT-based auth
2. **Add Database**: Connect PostgreSQL via Railway
3. **Implement Zones**: Add zone-specific features
4. **Add Monitoring**: Set up Datadog or similar
5. **Scale**: Configure auto-scaling rules

## 📝 License

Part of the AMOS project.