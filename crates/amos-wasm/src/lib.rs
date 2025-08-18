use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use uuid::Uuid;
use std::collections::HashMap;
use web_sys::console;

// Macro for logging to browser console
macro_rules! log {
    ($($t:tt)*) => {
        console::log_1(&format!($($t)*).into());
    };
}

// JavaScript-friendly agent types
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AgentType {
    TrafficSeer = "TrafficSeer",
    PathwaySculptor = "PathwaySculptor",
    MemoryWeaver = "MemoryWeaver",
    Architect = "Architect",
    Builder = "Builder",
    Critic = "Critic",
    Guardian = "Guardian",
    Tester = "Tester",
    Optimizer = "Optimizer",
    Explorer = "Explorer",
    Coordinator = "Coordinator",
}

// JavaScript-friendly hormone types
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HormoneType {
    Cortisol = "Cortisol",
    Dopamine = "Dopamine",
    Serotonin = "Serotonin",
    Oxytocin = "Oxytocin",
    Adrenaline = "Adrenaline",
}

// Simplified neural pathway for WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralPathway {
    pub id: String,
    pub source_node: String,
    pub target_node: String,
    pub strength: f64,
    pub usage_count: u32,
}

// Simplified cognitive node for WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveNode {
    pub id: String,
    pub node_type: String,
    pub connections: Vec<String>,
}

// Agent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub agent_type: AgentType,
    pub state: String,
    pub neural_network_id: String,
}

// Process result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub output: String,
    pub pathways_activated: u32,
    pub agents_involved: Vec<String>,
    pub processing_time_ms: u32,
}

// Mesh status structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStatus {
    pub active_agents: u32,
    pub total_pathways: u32,
    pub total_nodes: u32,
    pub memory_usage_bytes: u32,
    pub uptime_seconds: u32,
}

// Main AMOS client for WASM
#[wasm_bindgen]
pub struct AMOSClient {
    agents: HashMap<String, AgentInfo>,
    pathways: HashMap<String, NeuralPathway>,
    nodes: HashMap<String, CognitiveNode>,
    hormone_levels: HashMap<String, f64>,
    start_time: f64,
}

#[wasm_bindgen]
impl AMOSClient {
    // Constructor
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<AMOSClient, JsError> {
        // Set panic hook for better error messages
        console_error_panic_hook::set_once();
        
        log!("Initializing AMOS WASM Client");
        
        let mut hormone_levels = HashMap::new();
        hormone_levels.insert("Cortisol".to_string(), 0.5);
        hormone_levels.insert("Dopamine".to_string(), 0.5);
        hormone_levels.insert("Serotonin".to_string(), 0.5);
        hormone_levels.insert("Oxytocin".to_string(), 0.5);
        hormone_levels.insert("Adrenaline".to_string(), 0.5);
        
        Ok(AMOSClient {
            agents: HashMap::new(),
            pathways: HashMap::new(),
            nodes: HashMap::new(),
            hormone_levels,
            start_time: js_sys::Date::now(),
        })
    }
    
    // Spawn a new agent
    #[wasm_bindgen(js_name = spawnAgent)]
    pub fn spawn_agent(&mut self, agent_type: AgentType) -> Result<String, JsError> {
        let agent_id = Uuid::new_v4().to_string();
        let neural_network_id = Uuid::new_v4().to_string();
        
        let agent_name = match agent_type {
            AgentType::TrafficSeer => "Traffic Seer",
            AgentType::PathwaySculptor => "Pathway Sculptor",
            AgentType::MemoryWeaver => "Memory Weaver",
            AgentType::Architect => "System Architect",
            AgentType::Builder => "Code Builder",
            AgentType::Critic => "Quality Critic",
            AgentType::Guardian => "Security Guardian",
            AgentType::Tester => "Test Engineer",
            AgentType::Optimizer => "Performance Optimizer",
            AgentType::Explorer => "Solution Explorer",
            AgentType::Coordinator => "Team Coordinator",
            AgentType::__Invalid => "Invalid Agent",
        };
        
        let agent = AgentInfo {
            id: agent_id.clone(),
            name: agent_name.to_string(),
            agent_type,
            state: "active".to_string(),
            neural_network_id,
        };
        
        self.agents.insert(agent_id.clone(), agent);
        
        log!("Spawned agent: {} ({})", agent_name, agent_id);
        
        // Create initial neural nodes for the agent
        self.create_agent_nodes(&agent_id)?;
        
        Ok(agent_id)
    }
    
    // Get all agents
    #[wasm_bindgen(js_name = getAgents)]
    pub fn get_agents(&self) -> Result<JsValue, JsError> {
        let agents: Vec<&AgentInfo> = self.agents.values().collect();
        to_value(&agents).map_err(|e| JsError::new(&e.to_string()))
    }
    
    // Get a specific agent
    #[wasm_bindgen(js_name = getAgent)]
    pub fn get_agent(&self, agent_id: &str) -> Result<JsValue, JsError> {
        match self.agents.get(agent_id) {
            Some(agent) => to_value(agent).map_err(|e| JsError::new(&e.to_string())),
            None => Err(JsError::new(&format!("Agent {} not found", agent_id))),
        }
    }
    
    // Process user input
    #[wasm_bindgen(js_name = processUserInput)]
    pub async fn process_user_input(&mut self, input: &str) -> Result<JsValue, JsError> {
        let start_time = js_sys::Date::now();
        
        log!("Processing input: {}", input);
        
        // Simulate neural processing
        let mut activated_pathways = 0;
        let mut involved_agents = Vec::new();
        
        // Activate relevant agents based on input
        let agents_to_activate: Vec<String> = self.agents.iter()
            .filter(|(_, agent)| self.should_activate_agent(&agent.agent_type, input))
            .map(|(id, _)| id.clone())
            .collect();
        
        for agent_id in &agents_to_activate {
            involved_agents.push(agent_id.clone());
            activated_pathways += self.activate_agent_pathways(agent_id)?;
        }
        
        // Generate response
        let output = self.generate_response(input, &involved_agents);
        
        let processing_time = (js_sys::Date::now() - start_time) as u32;
        
        let result = ProcessResult {
            output,
            pathways_activated: activated_pathways,
            agents_involved: involved_agents,
            processing_time_ms: processing_time,
        };
        
        to_value(&result).map_err(|e| JsError::new(&e.to_string()))
    }
    
    // Strengthen a pathway between nodes
    #[wasm_bindgen(js_name = strengthenPathway)]
    pub fn strengthen_pathway(&mut self, source: &str, target: &str, delta: f64) -> Result<(), JsError> {
        let pathway_key = format!("{}->{}", source, target);
        
        if let Some(pathway) = self.pathways.get_mut(&pathway_key) {
            pathway.strength = (pathway.strength + delta).min(1.0);
            pathway.usage_count += 1;
            log!("Strengthened pathway {} to {:.2}", pathway_key, pathway.strength);
        } else {
            // Create new pathway if it doesn't exist
            let pathway = NeuralPathway {
                id: Uuid::new_v4().to_string(),
                source_node: source.to_string(),
                target_node: target.to_string(),
                strength: delta.min(1.0),
                usage_count: 1,
            };
            
            self.pathways.insert(pathway_key.clone(), pathway);
            log!("Created new pathway {}", pathway_key);
        }
        
        Ok(())
    }
    
    // Trigger a hormonal burst
    #[wasm_bindgen(js_name = triggerHormonalBurst)]
    pub fn trigger_hormonal_burst(&mut self, hormone: HormoneType, intensity: f64) -> Result<(), JsError> {
        let hormone_name = match hormone {
            HormoneType::Cortisol => "Cortisol",
            HormoneType::Dopamine => "Dopamine",
            HormoneType::Serotonin => "Serotonin",
            HormoneType::Oxytocin => "Oxytocin",
            HormoneType::Adrenaline => "Adrenaline",
            HormoneType::__Invalid => return Err(JsError::new("Invalid hormone type")),
        };
        
        if let Some(level) = self.hormone_levels.get_mut(hormone_name) {
            *level = (*level + intensity).min(1.0);
            log!("Triggered {} burst with intensity {:.2}, new level: {:.2}", 
                hormone_name, intensity, *level);
            
            // Apply hormone effects to pathways
            self.apply_hormone_effects(hormone_name, intensity)?;
        }
        
        Ok(())
    }
    
    // Get current mesh status
    #[wasm_bindgen(js_name = getMeshStatus)]
    pub fn get_mesh_status(&self) -> Result<JsValue, JsError> {
        let uptime = ((js_sys::Date::now() - self.start_time) / 1000.0) as u32;
        
        let status = MeshStatus {
            active_agents: self.agents.len() as u32,
            total_pathways: self.pathways.len() as u32,
            total_nodes: self.nodes.len() as u32,
            memory_usage_bytes: self.estimate_memory_usage(),
            uptime_seconds: uptime,
        };
        
        to_value(&status).map_err(|e| JsError::new(&e.to_string()))
    }
    
    // Create local neural network
    #[wasm_bindgen(js_name = createLocalNetwork)]
    pub fn create_local_network(&mut self) -> Result<String, JsError> {
        let network_id = Uuid::new_v4().to_string();
        log!("Created local neural network: {}", network_id);
        Ok(network_id)
    }
    
    // Add a node to the network
    #[wasm_bindgen(js_name = addNode)]
    pub fn add_node(&mut self, node_type: &str, _data: JsValue) -> Result<String, JsError> {
        let node_id = Uuid::new_v4().to_string();
        
        let node = CognitiveNode {
            id: node_id.clone(),
            node_type: node_type.to_string(),
            connections: Vec::new(),
        };
        
        self.nodes.insert(node_id.clone(), node);
        log!("Added {} node: {}", node_type, node_id);
        
        Ok(node_id)
    }
    
    // Connect two nodes
    #[wasm_bindgen(js_name = connectNodes)]
    pub fn connect_nodes(&mut self, source_id: &str, target_id: &str, strength: f64) -> Result<(), JsError> {
        // Update source node connections
        if let Some(source_node) = self.nodes.get_mut(source_id) {
            source_node.connections.push(target_id.to_string());
        } else {
            return Err(JsError::new(&format!("Source node {} not found", source_id)));
        }
        
        // Create pathway
        self.strengthen_pathway(source_id, target_id, strength)?;
        
        Ok(())
    }
    
    // Get hormone levels
    #[wasm_bindgen(js_name = getHormoneLevels)]
    pub fn get_hormone_levels(&self) -> Result<JsValue, JsError> {
        to_value(&self.hormone_levels).map_err(|e| JsError::new(&e.to_string()))
    }
    
    // Debug: dump current state
    #[wasm_bindgen(js_name = debugDumpState)]
    pub fn debug_dump_state(&self) -> Result<JsValue, JsError> {
        let state = serde_json::json!({
            "agents": self.agents.len(),
            "pathways": self.pathways.len(),
            "nodes": self.nodes.len(),
            "hormone_levels": self.hormone_levels,
            "uptime_ms": js_sys::Date::now() - self.start_time,
        });
        
        to_value(&state).map_err(|e| JsError::new(&e.to_string()))
    }
}

// Private implementation methods
impl AMOSClient {
    fn create_agent_nodes(&mut self, _agent_id: &str) -> Result<(), JsError> {
        // Create base nodes for the agent
        let memory_node = self.add_node("memory", JsValue::NULL)?;
        let thinking_node = self.add_node("thinking", JsValue::NULL)?;
        let agent_node = self.add_node("agent", JsValue::NULL)?;
        
        // Connect nodes
        self.connect_nodes(&memory_node, &thinking_node, 0.5)?;
        self.connect_nodes(&thinking_node, &agent_node, 0.5)?;
        
        Ok(())
    }
    
    fn should_activate_agent(&self, agent_type: &AgentType, input: &str) -> bool {
        let input_lower = input.to_lowercase();
        
        match agent_type {
            AgentType::TrafficSeer => input_lower.contains("traffic") || input_lower.contains("flow"),
            AgentType::PathwaySculptor => input_lower.contains("pathway") || input_lower.contains("connection"),
            AgentType::MemoryWeaver => input_lower.contains("memory") || input_lower.contains("remember"),
            AgentType::Architect => input_lower.contains("design") || input_lower.contains("architecture"),
            AgentType::Builder => input_lower.contains("build") || input_lower.contains("create"),
            AgentType::Critic => input_lower.contains("review") || input_lower.contains("quality"),
            AgentType::Guardian => input_lower.contains("security") || input_lower.contains("protect"),
            AgentType::Tester => input_lower.contains("test") || input_lower.contains("verify"),
            AgentType::Optimizer => input_lower.contains("optimize") || input_lower.contains("performance"),
            AgentType::Explorer => input_lower.contains("explore") || input_lower.contains("discover"),
            AgentType::Coordinator => input_lower.contains("coordinate") || input_lower.contains("manage"),
            AgentType::__Invalid => false,
        }
    }
    
    fn activate_agent_pathways(&mut self, _agent_id: &str) -> Result<u32, JsError> {
        let mut activated = 0;
        
        // Simulate pathway activation
        for (_, pathway) in self.pathways.iter_mut() {
            if pathway.strength > 0.3 {
                pathway.usage_count += 1;
                activated += 1;
            }
        }
        
        Ok(activated)
    }
    
    fn generate_response(&self, input: &str, involved_agents: &[String]) -> String {
        if involved_agents.is_empty() {
            return format!("Processed: '{}' (no specific agents activated)", input);
        }
        
        let agent_names: Vec<String> = involved_agents.iter()
            .filter_map(|id| self.agents.get(id))
            .map(|agent| agent.name.clone())
            .collect();
        
        format!(
            "Processed: '{}' with assistance from: {}",
            input,
            agent_names.join(", ")
        )
    }
    
    fn apply_hormone_effects(&mut self, hormone: &str, intensity: f64) -> Result<(), JsError> {
        // Apply hormone-specific effects to pathways
        match hormone {
            "Dopamine" => {
                // Strengthen recently used pathways
                for pathway in self.pathways.values_mut() {
                    if pathway.usage_count > 0 {
                        pathway.strength = (pathway.strength + intensity * 0.1).min(1.0);
                    }
                }
            },
            "Cortisol" => {
                // Weaken weak pathways (pruning under stress)
                for pathway in self.pathways.values_mut() {
                    if pathway.strength < 0.3 {
                        pathway.strength = (pathway.strength - intensity * 0.05).max(0.0);
                    }
                }
            },
            _ => {}
        }
        
        Ok(())
    }
    
    fn estimate_memory_usage(&self) -> u32 {
        // Rough estimation of memory usage
        let agent_size = 200; // bytes per agent
        let pathway_size = 100; // bytes per pathway
        let node_size = 150; // bytes per node
        
        (self.agents.len() * agent_size + 
         self.pathways.len() * pathway_size + 
         self.nodes.len() * node_size) as u32
    }
}

// Module initialization
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    log!("AMOS WASM module initialized");
}

// Version information
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_client_creation() {
        let client = AMOSClient::new().unwrap();
        assert_eq!(client.agents.len(), 0);
        assert_eq!(client.pathways.len(), 0);
        assert_eq!(client.nodes.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_spawn_agent() {
        let mut client = AMOSClient::new().unwrap();
        let agent_id = client.spawn_agent(AgentType::TrafficSeer).unwrap();
        assert!(!agent_id.is_empty());
        assert_eq!(client.agents.len(), 1);
    }

    #[wasm_bindgen_test]
    fn test_hormone_burst() {
        let mut client = AMOSClient::new().unwrap();
        client.trigger_hormonal_burst(HormoneType::Dopamine, 0.3).unwrap();
        assert_eq!(*client.hormone_levels.get("Dopamine").unwrap(), 0.8);
    }
}