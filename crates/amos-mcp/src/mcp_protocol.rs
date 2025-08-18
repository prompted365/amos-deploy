use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

/// MCP Protocol version
pub const MCP_VERSION: &str = "1.0.0";

/// MCP Request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: String,
}

impl McpRequest {
    pub fn new(method: String, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method,
            params,
            id: Uuid::new_v4().to_string(),
        }
    }
}

/// MCP Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<McpError>,
    pub id: String,
}

impl McpResponse {
    pub fn success(id: String, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }
    
    pub fn error(id: String, error: McpError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id,
        }
    }
}

/// MCP Error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

/// MCP Method types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum McpMethod {
    // Tool methods
    #[serde(rename = "tools/list")]
    ToolsList,
    #[serde(rename = "tools/call")]
    ToolsCall,
    
    // Context methods
    #[serde(rename = "context/list")]
    ContextList,
    #[serde(rename = "context/get")]
    ContextGet,
    
    // Resource methods
    #[serde(rename = "resources/list")]
    ResourcesList,
    #[serde(rename = "resources/get")]
    ResourcesGet,
    
    // Prompt methods
    #[serde(rename = "prompts/list")]
    PromptsList,
    #[serde(rename = "prompts/get")]
    PromptsGet,
    
    // System methods
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "ping")]
    Ping,
    
    // Custom AMOS methods
    #[serde(rename = "amos/agent/status")]
    AmosAgentStatus,
    #[serde(rename = "amos/agent/command")]
    AmosAgentCommand,
    #[serde(rename = "amos/neural/query")]
    AmosNeuralQuery,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// Context item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub content_type: String,
}

/// Resource definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
}

/// Prompt template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub arguments: Vec<PromptArgument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
}

/// Initialize parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub client_info: ClientInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: bool,
    pub context: bool,
    pub resources: bool,
    pub prompts: bool,
    pub amos_extensions: bool,
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self {
            tools: true,
            context: true,
            resources: true,
            prompts: true,
            amos_extensions: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

/// Tool call parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    pub arguments: Value,
}

/// Tool call result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub content: Vec<ToolContent>,
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    pub data: Option<Value>,
}

impl ToolContent {
    pub fn text(content: String) -> Self {
        Self {
            content_type: "text".to_string(),
            text: Some(content),
            data: None,
        }
    }
    
    pub fn json(data: Value) -> Self {
        Self {
            content_type: "json".to_string(),
            text: None,
            data: Some(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mcp_request_creation() {
        let request = McpRequest::new(
            "tools/list".to_string(),
            None
        );
        
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "tools/list");
        assert!(request.params.is_none());
        assert!(!request.id.is_empty());
    }
    
    #[test]
    fn test_mcp_response_success() {
        let response = McpResponse::success(
            "test-id".to_string(),
            serde_json::json!({"status": "ok"})
        );
        
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());
        assert!(response.error.is_none());
        assert_eq!(response.id, "test-id");
    }
    
    #[test]
    fn test_tool_content() {
        let text_content = ToolContent::text("Hello".to_string());
        assert_eq!(text_content.content_type, "text");
        assert_eq!(text_content.text, Some("Hello".to_string()));
        
        let json_content = ToolContent::json(serde_json::json!({"key": "value"}));
        assert_eq!(json_content.content_type, "json");
        assert!(json_content.data.is_some());
    }
}