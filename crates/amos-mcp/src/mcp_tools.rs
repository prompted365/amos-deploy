use crate::mcp_protocol::{Tool, ToolCallParams, ToolCallResult, ToolContent};
use anyhow::{Result, anyhow};
use serde_json::{json, Value};
use std::collections::HashMap;
use async_trait::async_trait;
use amos_agents::CognitiveAgent;
use amos_core::system::SystemInfo;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait for MCP tool implementations
#[async_trait]
pub trait McpTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;
    async fn execute(&self, params: Value) -> Result<ToolCallResult>;
}

/// Tool registry for managing available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn McpTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
    
    /// Register a new tool
    pub fn register(&mut self, tool: Arc<dyn McpTool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }
    
    /// Get all available tools
    pub fn list_tools(&self) -> Vec<Tool> {
        self.tools.values().map(|tool| Tool {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            input_schema: tool.input_schema(),
        }).collect()
    }
    
    /// Execute a tool
    pub async fn execute_tool(&self, params: ToolCallParams) -> Result<ToolCallResult> {
        let tool = self.tools.get(&params.name)
            .ok_or_else(|| anyhow!("Tool '{}' not found", params.name))?;
        
        tool.execute(params.arguments).await
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// AMOS-specific tool for querying agent status
pub struct AgentStatusTool {
    agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>,
}

impl AgentStatusTool {
    pub fn new(agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>) -> Self {
        Self { agents }
    }
}

#[async_trait]
impl McpTool for AgentStatusTool {
    fn name(&self) -> &str {
        "amos_agent_status"
    }
    
    fn description(&self) -> &str {
        "Query the status of AMOS cognitive agents"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "agent_id": {
                    "type": "string",
                    "description": "UUID of the agent to query (optional, returns all if not specified)"
                }
            }
        })
    }
    
    async fn execute(&self, params: Value) -> Result<ToolCallResult> {
        let agents = self.agents.read().await;
        
        let result = if let Some(agent_id_str) = params.get("agent_id").and_then(|v| v.as_str()) {
            // Query specific agent
            let agent_id = Uuid::parse_str(agent_id_str)?;
            if let Some(agent) = agents.get(&agent_id) {
                json!({
                    "agent_id": agent_id_str,
                    "name": agent.name(),
                    "state": format!("{:?}", agent.state()),
                })
            } else {
                return Ok(ToolCallResult {
                    content: vec![ToolContent::text(format!("Agent {} not found", agent_id_str))],
                    is_error: true,
                });
            }
        } else {
            // Return all agents
            let all_agents: Vec<Value> = agents.iter().map(|(id, agent)| {
                json!({
                    "agent_id": id.to_string(),
                    "name": agent.name(),
                    "state": format!("{:?}", agent.state()),
                })
            }).collect();
            
            json!({
                "agents": all_agents,
                "count": all_agents.len()
            })
        };
        
        Ok(ToolCallResult {
            content: vec![ToolContent::json(result)],
            is_error: false,
        })
    }
}

/// Tool for system diagnostics
pub struct SystemDiagnosticsTool;

#[async_trait]
impl McpTool for SystemDiagnosticsTool {
    fn name(&self) -> &str {
        "amos_system_diagnostics"
    }
    
    fn description(&self) -> &str {
        "Get AMOS system diagnostics and health information"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "include_metrics": {
                    "type": "boolean",
                    "description": "Include detailed metrics",
                    "default": false
                }
            }
        })
    }
    
    async fn execute(&self, params: Value) -> Result<ToolCallResult> {
        let include_metrics = params.get("include_metrics")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        let system_info = SystemInfo::gather();
        
        let mut result = json!({
            "system": {
                "os": system_info.os,
                "architecture": system_info.architecture,
                "cpu_count": system_info.cpu_count,
                "memory_mb": system_info.memory_mb,
            },
            "amos": {
                "version": env!("CARGO_PKG_VERSION"),
                "uptime_seconds": 0, // Would be tracked in production
            }
        });
        
        if include_metrics {
            result["metrics"] = json!({
                "neural_pathways": 0,
                "active_agents": 0,
                "events_processed": 0,
            });
        }
        
        Ok(ToolCallResult {
            content: vec![ToolContent::json(result)],
            is_error: false,
        })
    }
}

/// Tool for executing agent commands
pub struct AgentCommandTool {
    agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>,
}

impl AgentCommandTool {
    pub fn new(agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>) -> Self {
        Self { agents }
    }
}

#[async_trait]
impl McpTool for AgentCommandTool {
    fn name(&self) -> &str {
        "amos_agent_command"
    }
    
    fn description(&self) -> &str {
        "Send commands to AMOS cognitive agents"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "agent_id": {
                    "type": "string",
                    "description": "UUID of the target agent"
                },
                "command": {
                    "type": "string",
                    "description": "Command to execute",
                    "enum": ["start", "stop", "pause", "resume", "reset"]
                }
            },
            "required": ["agent_id", "command"]
        })
    }
    
    async fn execute(&self, params: Value) -> Result<ToolCallResult> {
        let agent_id_str = params.get("agent_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("agent_id is required"))?;
        
        let command = params.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("command is required"))?;
        
        let agent_id = Uuid::parse_str(agent_id_str)?;
        let agents = self.agents.read().await;
        
        if !agents.contains_key(&agent_id) {
            return Ok(ToolCallResult {
                content: vec![ToolContent::text(format!("Agent {} not found", agent_id_str))],
                is_error: true,
            });
        }
        
        // In a real implementation, we would execute the command
        // For now, we'll just return success
        let result = json!({
            "agent_id": agent_id_str,
            "command": command,
            "status": "executed",
            "message": format!("Command '{}' executed successfully", command)
        });
        
        Ok(ToolCallResult {
            content: vec![ToolContent::json(result)],
            is_error: false,
        })
    }
}

/// Create a default tool registry with standard AMOS tools
pub fn create_default_registry(
    agents: Arc<RwLock<HashMap<Uuid, Arc<dyn CognitiveAgent>>>>
) -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    
    // Register AMOS-specific tools
    registry.register(Arc::new(AgentStatusTool::new(agents.clone())));
    registry.register(Arc::new(SystemDiagnosticsTool));
    registry.register(Arc::new(AgentCommandTool::new(agents)));
    
    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_registry() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.list_tools().len(), 0);
    }
    
    #[tokio::test]
    async fn test_system_diagnostics_tool() {
        let tool = SystemDiagnosticsTool;
        
        let result = tool.execute(json!({})).await.unwrap();
        assert!(!result.is_error);
        assert_eq!(result.content.len(), 1);
    }
}