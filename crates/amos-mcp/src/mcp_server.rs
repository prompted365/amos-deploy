use crate::{
    mcp_protocol::*,
    mcp_tools::{ToolRegistry, create_default_registry},
    mcp_context::ContextProvider,
};
use anyhow::{Result, anyhow};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use amos_core::neural::ForgeNeuralNetwork;
use amos_agents::CognitiveAgent;
use std::collections::HashMap;
use tracing::{info, error};

/// MCP Server implementation
pub struct McpServer {
    tool_registry: Arc<RwLock<ToolRegistry>>,
    context_provider: Arc<ContextProvider>,
    capabilities: ServerCapabilities,
    server_info: ServerInfo,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub vendor: String,
}

impl McpServer {
    pub fn new(
        neural_network: Arc<ForgeNeuralNetwork>,
        agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>
    ) -> Self {
        let tool_registry = Arc::new(RwLock::new(create_default_registry(agents.clone())));
        let context_provider = Arc::new(ContextProvider::new(neural_network, agents));
        
        Self {
            tool_registry,
            context_provider,
            capabilities: ServerCapabilities::default(),
            server_info: ServerInfo {
                name: "AMOS MCP Server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                vendor: "AMOS Project".to_string(),
            },
        }
    }
    
    /// Handle an incoming MCP request
    pub async fn handle_request(&self, request: McpRequest) -> McpResponse {
        info!("Handling MCP request: {} (id: {})", request.method, request.id);
        
        let result = match self.route_request(&request).await {
            Ok(value) => McpResponse::success(request.id.clone(), value),
            Err(e) => {
                error!("Error handling request: {}", e);
                McpResponse::error(request.id.clone(), McpError {
                    code: -32603,
                    message: e.to_string(),
                    data: None,
                })
            }
        };
        
        result
    }
    
    /// Route request to appropriate handler
    async fn route_request(&self, request: &McpRequest) -> Result<Value> {
        match request.method.as_str() {
            // System methods
            "initialize" => self.handle_initialize(request.params.as_ref()).await,
            "ping" => Ok(json!({"pong": true})),
            
            // Tool methods
            "tools/list" => self.handle_tools_list().await,
            "tools/call" => self.handle_tools_call(request.params.as_ref()).await,
            
            // Context methods
            "context/list" => self.handle_context_list().await,
            "context/get" => self.handle_context_get(request.params.as_ref()).await,
            
            // Resource methods
            "resources/list" => self.handle_resources_list().await,
            "resources/get" => self.handle_resources_get(request.params.as_ref()).await,
            
            // Prompt methods
            "prompts/list" => self.handle_prompts_list().await,
            "prompts/get" => self.handle_prompts_get(request.params.as_ref()).await,
            
            // AMOS-specific methods
            method if method.starts_with("amos/") => {
                self.handle_amos_method(method, request.params.as_ref()).await
            }
            
            _ => Err(anyhow!("Unknown method: {}", request.method)),
        }
    }
    
    /// Handle initialize request
    async fn handle_initialize(&self, params: Option<&Value>) -> Result<Value> {
        let client_info = if let Some(params) = params {
            params.get("client_info")
                .and_then(|ci| serde_json::from_value::<ClientInfo>(ci.clone()).ok())
        } else {
            None
        };
        
        if let Some(info) = &client_info {
            info!("MCP client connected: {} v{}", info.name, info.version);
        }
        
        Ok(json!({
            "protocol_version": MCP_VERSION,
            "capabilities": self.capabilities,
            "server_info": {
                "name": self.server_info.name,
                "version": self.server_info.version,
                "vendor": self.server_info.vendor,
            }
        }))
    }
    
    /// Handle tools/list request
    async fn handle_tools_list(&self) -> Result<Value> {
        let registry = self.tool_registry.read().await;
        let tools = registry.list_tools();
        
        Ok(json!({
            "tools": tools
        }))
    }
    
    /// Handle tools/call request
    async fn handle_tools_call(&self, params: Option<&Value>) -> Result<Value> {
        let params = params.ok_or_else(|| anyhow!("Missing parameters"))?;
        let tool_params: ToolCallParams = serde_json::from_value(params.clone())?;
        
        let registry = self.tool_registry.read().await;
        let result = registry.execute_tool(tool_params).await?;
        
        Ok(serde_json::to_value(result)?)
    }
    
    /// Handle context/list request
    async fn handle_context_list(&self) -> Result<Value> {
        let contexts = self.context_provider.list_contexts().await;
        
        Ok(json!({
            "contexts": contexts
        }))
    }
    
    /// Handle context/get request
    async fn handle_context_get(&self, params: Option<&Value>) -> Result<Value> {
        let params = params.ok_or_else(|| anyhow!("Missing parameters"))?;
        let context_id = params.get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing context id"))?;
        
        let content = self.context_provider.get_context(context_id).await?;
        
        Ok(json!({
            "content": content
        }))
    }
    
    /// Handle resources/list request
    async fn handle_resources_list(&self) -> Result<Value> {
        // AMOS doesn't use traditional resources, but we can expose some
        let resources = vec![
            Resource {
                uri: "amos://neural/network".to_string(),
                name: "Neural Network".to_string(),
                description: "AMOS neural network structure and state".to_string(),
                mime_type: "application/json".to_string(),
            },
            Resource {
                uri: "amos://agents/swarm".to_string(),
                name: "Agent Swarm".to_string(),
                description: "Cognitive agent swarm configuration".to_string(),
                mime_type: "application/json".to_string(),
            },
        ];
        
        Ok(json!({
            "resources": resources
        }))
    }
    
    /// Handle resources/get request
    async fn handle_resources_get(&self, params: Option<&Value>) -> Result<Value> {
        let params = params.ok_or_else(|| anyhow!("Missing parameters"))?;
        let uri = params.get("uri")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing resource URI"))?;
        
        // Map URIs to context IDs
        let context_id = match uri {
            "amos://neural/network" => "neural_network",
            "amos://agents/swarm" => "agent_swarm",
            _ => return Err(anyhow!("Unknown resource URI: {}", uri)),
        };
        
        let content = self.context_provider.get_context(context_id).await?;
        
        Ok(json!({
            "contents": [{
                "uri": uri,
                "mime_type": "application/json",
                "data": content
            }]
        }))
    }
    
    /// Handle prompts/list request
    async fn handle_prompts_list(&self) -> Result<Value> {
        let prompts = vec![
            PromptTemplate {
                id: "analyze_neural_state".to_string(),
                name: "Analyze Neural State".to_string(),
                description: "Analyze the current state of the neural network".to_string(),
                arguments: vec![
                    PromptArgument {
                        name: "focus_area".to_string(),
                        description: "Specific area to focus analysis on".to_string(),
                        required: false,
                    }
                ],
            },
            PromptTemplate {
                id: "optimize_swarm".to_string(),
                name: "Optimize Agent Swarm".to_string(),
                description: "Suggest optimizations for the agent swarm".to_string(),
                arguments: vec![
                    PromptArgument {
                        name: "metric".to_string(),
                        description: "Metric to optimize for".to_string(),
                        required: true,
                    }
                ],
            },
        ];
        
        Ok(json!({
            "prompts": prompts
        }))
    }
    
    /// Handle prompts/get request
    async fn handle_prompts_get(&self, params: Option<&Value>) -> Result<Value> {
        let params = params.ok_or_else(|| anyhow!("Missing parameters"))?;
        let prompt_id = params.get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing prompt id"))?;
        
        let prompt_text = match prompt_id {
            "analyze_neural_state" => {
                "Analyze the current state of the AMOS neural network. \
                Focus on: {{focus_area|overall health and performance}}. \
                Consider pathway strength distribution, pruning activity, \
                and overall network connectivity."
            },
            "optimize_swarm" => {
                "Suggest optimizations for the AMOS agent swarm to improve {{metric}}. \
                Consider current agent states, resource allocation, \
                and potential bottlenecks in the system."
            },
            _ => return Err(anyhow!("Unknown prompt id: {}", prompt_id)),
        };
        
        Ok(json!({
            "prompt": prompt_text
        }))
    }
    
    /// Handle AMOS-specific methods
    async fn handle_amos_method(&self, method: &str, params: Option<&Value>) -> Result<Value> {
        match method {
            "amos/agent/status" => {
                // Delegate to the agent status tool
                let registry = self.tool_registry.read().await;
                let tool_params = ToolCallParams {
                    name: "amos_agent_status".to_string(),
                    arguments: params.cloned().unwrap_or(json!({})),
                };
                let result = registry.execute_tool(tool_params).await?;
                Ok(serde_json::to_value(result)?)
            },
            "amos/neural/query" => {
                // Direct neural network query
                self.context_provider.get_context("neural_network").await
            },
            _ => Err(anyhow!("Unknown AMOS method: {}", method)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mcp_server_creation() {
        let neural_network = Arc::new(ForgeNeuralNetwork::new());
        let agents = Arc::new(RwLock::new(HashMap::new()));
        
        let server = McpServer::new(neural_network, agents);
        assert_eq!(server.server_info.name, "AMOS MCP Server");
    }
    
    #[tokio::test]
    async fn test_ping_request() {
        let neural_network = Arc::new(ForgeNeuralNetwork::new());
        let agents = Arc::new(RwLock::new(HashMap::new()));
        let server = McpServer::new(neural_network, agents);
        
        let request = McpRequest::new("ping".to_string(), None);
        let response = server.handle_request(request).await;
        
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }
    
    #[tokio::test]
    async fn test_initialize_request() {
        let neural_network = Arc::new(ForgeNeuralNetwork::new());
        let agents = Arc::new(RwLock::new(HashMap::new()));
        let server = McpServer::new(neural_network, agents);
        
        let params = json!({
            "client_info": {
                "name": "test_client",
                "version": "1.0.0"
            }
        });
        
        let request = McpRequest::new("initialize".to_string(), Some(params));
        let response = server.handle_request(request).await;
        
        assert!(response.result.is_some());
        let result = response.result.unwrap();
        assert_eq!(result["protocol_version"], MCP_VERSION);
    }
}