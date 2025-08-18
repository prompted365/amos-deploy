use crate::mcp_protocol::*;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use std::collections::HashMap;
use tracing::warn;

/// MCP Client for connecting to MCP servers
pub struct McpClient {
    client_info: ClientInfo,
    pending_requests: Arc<RwLock<HashMap<String, mpsc::Sender<McpResponse>>>>,
    request_tx: mpsc::Sender<McpRequest>,
    response_rx: Arc<RwLock<mpsc::Receiver<McpResponse>>>,
}

impl McpClient {
    pub fn new(name: String, version: String) -> (Self, mpsc::Receiver<McpRequest>) {
        let (request_tx, request_rx) = mpsc::channel(100);
        let (_response_tx, response_rx) = mpsc::channel(100);
        
        let client = Self {
            client_info: ClientInfo { name, version },
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            request_tx,
            response_rx: Arc::new(RwLock::new(response_rx)),
        };
        
        // Start response handler
        let pending_requests = client.pending_requests.clone();
        let response_rx = client.response_rx.clone();
        
        tokio::spawn(async move {
            let mut rx = response_rx.write().await;
            while let Some(response) = rx.recv().await {
                let mut pending = pending_requests.write().await;
                if let Some(tx) = pending.remove(&response.id) {
                    if let Err(e) = tx.send(response).await {
                        warn!("Failed to send response: {}", e);
                    }
                }
            }
        });
        
        (client, request_rx)
    }
    
    /// Initialize connection with server
    pub async fn initialize(&self) -> Result<InitializeResult> {
        let params = json!({
            "protocol_version": MCP_VERSION,
            "client_info": self.client_info,
        });
        
        let response = self.request("initialize", Some(params)).await?;
        
        if let Some(error) = response.error {
            return Err(anyhow!("Initialize failed: {}", error.message));
        }
        
        let result = response.result
            .ok_or_else(|| anyhow!("No result in initialize response"))?;
        
        Ok(serde_json::from_value(result)?)
    }
    
    /// List available tools
    pub async fn list_tools(&self) -> Result<Vec<Tool>> {
        let response = self.request("tools/list", None).await?;
        
        if let Some(error) = response.error {
            return Err(anyhow!("List tools failed: {}", error.message));
        }
        
        let result = response.result
            .ok_or_else(|| anyhow!("No result in tools/list response"))?;
        
        let tools = result.get("tools")
            .ok_or_else(|| anyhow!("No tools in response"))?;
        
        Ok(serde_json::from_value(tools.clone())?)
    }
    
    /// Call a tool
    pub async fn call_tool(&self, name: String, arguments: Value) -> Result<ToolCallResult> {
        let params = json!({
            "name": name,
            "arguments": arguments,
        });
        
        let response = self.request("tools/call", Some(params)).await?;
        
        if let Some(error) = response.error {
            return Err(anyhow!("Tool call failed: {}", error.message));
        }
        
        let result = response.result
            .ok_or_else(|| anyhow!("No result in tool call response"))?;
        
        Ok(serde_json::from_value(result)?)
    }
    
    /// List available contexts
    pub async fn list_contexts(&self) -> Result<Vec<ContextItem>> {
        let response = self.request("context/list", None).await?;
        
        if let Some(error) = response.error {
            return Err(anyhow!("List contexts failed: {}", error.message));
        }
        
        let result = response.result
            .ok_or_else(|| anyhow!("No result in context/list response"))?;
        
        let contexts = result.get("contexts")
            .ok_or_else(|| anyhow!("No contexts in response"))?;
        
        Ok(serde_json::from_value(contexts.clone())?)
    }
    
    /// Get a specific context
    pub async fn get_context(&self, context_id: String) -> Result<Value> {
        let params = json!({
            "id": context_id,
        });
        
        let response = self.request("context/get", Some(params)).await?;
        
        if let Some(error) = response.error {
            return Err(anyhow!("Get context failed: {}", error.message));
        }
        
        let result = response.result
            .ok_or_else(|| anyhow!("No result in context/get response"))?;
        
        result.get("content")
            .cloned()
            .ok_or_else(|| anyhow!("No content in context response"))
    }
    
    /// Query AMOS agent status
    pub async fn query_agent_status(&self, agent_id: Option<String>) -> Result<Value> {
        let params = if let Some(id) = agent_id {
            json!({ "agent_id": id })
        } else {
            json!({})
        };
        
        let response = self.request("amos/agent/status", Some(params)).await?;
        
        if let Some(error) = response.error {
            return Err(anyhow!("Agent status query failed: {}", error.message));
        }
        
        response.result
            .ok_or_else(|| anyhow!("No result in agent status response"))
    }
    
    /// Send a raw request
    async fn request(&self, method: &str, params: Option<Value>) -> Result<McpResponse> {
        let request = McpRequest::new(method.to_string(), params);
        let request_id = request.id.clone();
        
        // Create response channel
        let (tx, mut rx) = mpsc::channel(1);
        
        // Register pending request
        {
            let mut pending = self.pending_requests.write().await;
            pending.insert(request_id.clone(), tx);
        }
        
        // Send request
        self.request_tx.send(request).await
            .map_err(|e| anyhow!("Failed to send request: {}", e))?;
        
        // Wait for response
        tokio::time::timeout(tokio::time::Duration::from_secs(30), rx.recv())
            .await
            .map_err(|_| anyhow!("Request timeout"))?
            .ok_or_else(|| anyhow!("Response channel closed"))
    }
    
    /// Handle incoming response (called by transport layer)
    pub async fn handle_response(&self, response: McpResponse) -> Result<()> {
        let mut pending = self.pending_requests.write().await;
        
        if let Some(tx) = pending.remove(&response.id) {
            tx.send(response).await
                .map_err(|e| anyhow!("Failed to send response to waiting request: {}", e))?;
        } else {
            warn!("Received response for unknown request id: {}", response.id);
        }
        
        Ok(())
    }
}

/// Result from initialize call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub server_info: Option<ServerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub vendor: Option<String>,
}

/// Builder for creating MCP client instances
pub struct McpClientBuilder {
    name: String,
    version: String,
}

impl McpClientBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "1.0.0".to_string(),
        }
    }
    
    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }
    
    pub fn build(self) -> (McpClient, mpsc::Receiver<McpRequest>) {
        McpClient::new(self.name, self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_client_builder() {
        let (client, _rx) = McpClientBuilder::new("test_client".to_string())
            .with_version("2.0.0".to_string())
            .build();
        
        assert_eq!(client.client_info.name, "test_client");
        assert_eq!(client.client_info.version, "2.0.0");
    }
    
    #[tokio::test]
    async fn test_request_creation() {
        let (client, mut rx) = McpClient::new("test".to_string(), "1.0".to_string());
        
        // Send a request in the background
        tokio::spawn(async move {
            let _ = client.list_tools().await;
        });
        
        // Receive the request
        let request = rx.recv().await.unwrap();
        assert_eq!(request.method, "tools/list");
        assert_eq!(request.jsonrpc, "2.0");
    }
}