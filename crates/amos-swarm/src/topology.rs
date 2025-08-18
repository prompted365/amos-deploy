use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

/// Swarm topology defines how agents are connected and communicate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SwarmTopology {
    /// Mesh topology - all agents can communicate with all others
    Mesh {
        max_connections: usize,
    },
    
    /// Hierarchical topology - tree-like structure with levels
    Hierarchical {
        levels: usize,
        agents_per_level: usize,
    },
    
    /// Ring topology - agents connected in a circular pattern
    Ring,
    
    /// Star topology - central hub with satellites
    Star {
        max_satellites: usize,
    },
}

impl SwarmTopology {
    /// Calculate the optimal placement for a new agent
    pub fn calculate_placement(
        &self,
        existing_agents: &HashMap<Uuid, AgentPlacement>,
    ) -> AgentPlacement {
        match self {
            SwarmTopology::Mesh { .. } => {
                // In mesh, all agents are equal
                AgentPlacement::Mesh {
                    connections: existing_agents.keys().copied().collect(),
                }
            }
            
            SwarmTopology::Hierarchical { levels, agents_per_level } => {
                // Find the level with fewest agents
                let mut level_counts = vec![0; *levels];
                
                for placement in existing_agents.values() {
                    if let AgentPlacement::Hierarchical { level, .. } = placement {
                        if *level < *levels {
                            level_counts[*level] += 1;
                        }
                    }
                }
                
                // Find level with room
                let target_level = level_counts
                    .iter()
                    .enumerate()
                    .find(|(_, &count)| count < *agents_per_level)
                    .map(|(level, _)| level)
                    .unwrap_or(0);
                
                // Find parent in previous level
                let parent = if target_level > 0 {
                    existing_agents
                        .iter()
                        .find_map(|(id, placement)| {
                            if let AgentPlacement::Hierarchical { level, .. } = placement {
                                if *level == target_level - 1 {
                                    Some(*id)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                } else {
                    None
                };
                
                AgentPlacement::Hierarchical {
                    level: target_level,
                    parent,
                    children: HashSet::new(),
                }
            }
            
            SwarmTopology::Ring => {
                // Find two neighbors to insert between
                let agents: Vec<Uuid> = existing_agents.keys().copied().collect();
                
                let (prev, next) = if agents.len() < 2 {
                    (agents.first().copied(), None)
                } else {
                    // Insert between first two agents
                    (Some(agents[0]), Some(agents[1]))
                };
                
                AgentPlacement::Ring { prev, next }
            }
            
            SwarmTopology::Star { .. } => {
                // Check if we need a hub or satellite
                let has_hub = existing_agents
                    .values()
                    .any(|p| matches!(p, AgentPlacement::Star { is_hub: true, .. }));
                
                if has_hub {
                    // Find the hub
                    let hub_id = existing_agents
                        .iter()
                        .find_map(|(id, p)| {
                            if matches!(p, AgentPlacement::Star { is_hub: true, .. }) {
                                Some(*id)
                            } else {
                                None
                            }
                        });
                    
                    AgentPlacement::Star {
                        is_hub: false,
                        connections: hub_id.into_iter().collect(),
                    }
                } else {
                    // This agent becomes the hub
                    AgentPlacement::Star {
                        is_hub: true,
                        connections: HashSet::new(),
                    }
                }
            }
        }
    }
    
    /// Check if adding an agent would exceed topology limits
    pub fn can_add_agent(&self, current_count: usize) -> bool {
        match self {
            SwarmTopology::Mesh { max_connections } => {
                // Rough estimate: each agent can have max_connections
                current_count < max_connections * max_connections
            }
            SwarmTopology::Hierarchical { levels, agents_per_level } => {
                current_count < levels * agents_per_level
            }
            SwarmTopology::Ring => {
                current_count < 1000 // Practical limit
            }
            SwarmTopology::Star { max_satellites } => {
                current_count <= *max_satellites
            }
        }
    }
}

/// Agent placement within the swarm topology
#[derive(Debug, Clone)]
pub enum AgentPlacement {
    Mesh {
        connections: HashSet<Uuid>,
    },
    Hierarchical {
        level: usize,
        parent: Option<Uuid>,
        children: HashSet<Uuid>,
    },
    Ring {
        prev: Option<Uuid>,
        next: Option<Uuid>,
    },
    Star {
        is_hub: bool,
        connections: HashSet<Uuid>,
    },
}

impl AgentPlacement {
    /// Get all connected agents
    pub fn connections(&self) -> Vec<Uuid> {
        match self {
            AgentPlacement::Mesh { connections } => connections.iter().copied().collect(),
            AgentPlacement::Hierarchical { parent, children, .. } => {
                let mut conns = Vec::new();
                if let Some(p) = parent {
                    conns.push(*p);
                }
                conns.extend(children.iter());
                conns
            }
            AgentPlacement::Ring { prev, next } => {
                let mut conns = Vec::new();
                if let Some(p) = prev {
                    conns.push(*p);
                }
                if let Some(n) = next {
                    conns.push(*n);
                }
                conns
            }
            AgentPlacement::Star { connections, .. } => connections.iter().copied().collect(),
        }
    }
    
    /// Update connections when an agent joins
    pub fn on_agent_joined(&mut self, new_agent: Uuid, new_placement: &AgentPlacement) {
        match (self, new_placement) {
            (AgentPlacement::Mesh { connections }, AgentPlacement::Mesh { .. }) => {
                connections.insert(new_agent);
            }
            (
                AgentPlacement::Hierarchical { children, level, .. },
                AgentPlacement::Hierarchical { parent: Some(p), .. },
            ) if p == &Uuid::nil() => {
                // Fix: use a proper parent ID check
                children.insert(new_agent);
            }
            (AgentPlacement::Ring { next, .. }, AgentPlacement::Ring { prev: Some(p), .. }) => {
                // Update ring connections
                if next.is_none() {
                    *next = Some(new_agent);
                }
            }
            (AgentPlacement::Star { connections, is_hub, .. }, AgentPlacement::Star { .. }) => {
                if *is_hub {
                    connections.insert(new_agent);
                }
            }
            _ => {}
        }
    }
    
    /// Update connections when an agent leaves
    pub fn on_agent_left(&mut self, agent_id: Uuid) {
        match self {
            AgentPlacement::Mesh { connections } => {
                connections.remove(&agent_id);
            }
            AgentPlacement::Hierarchical { parent, children, .. } => {
                if parent == &Some(agent_id) {
                    *parent = None;
                }
                children.remove(&agent_id);
            }
            AgentPlacement::Ring { prev, next } => {
                if prev == &Some(agent_id) {
                    *prev = None;
                }
                if next == &Some(agent_id) {
                    *next = None;
                }
            }
            AgentPlacement::Star { connections, .. } => {
                connections.remove(&agent_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mesh_topology() {
        let topology = SwarmTopology::Mesh { max_connections: 6 };
        let agents = HashMap::new();
        
        let placement = topology.calculate_placement(&agents);
        match placement {
            AgentPlacement::Mesh { connections } => {
                assert!(connections.is_empty());
            }
            _ => panic!("Expected mesh placement"),
        }
    }
    
    #[test]
    fn test_hierarchical_topology() {
        let topology = SwarmTopology::Hierarchical {
            levels: 3,
            agents_per_level: 4,
        };
        
        let mut agents = HashMap::new();
        let root_id = Uuid::new_v4();
        agents.insert(
            root_id,
            AgentPlacement::Hierarchical {
                level: 0,
                parent: None,
                children: HashSet::new(),
            },
        );
        
        let placement = topology.calculate_placement(&agents);
        match placement {
            AgentPlacement::Hierarchical { level, parent, .. } => {
                assert_eq!(level, 1);
                assert_eq!(parent, Some(root_id));
            }
            _ => panic!("Expected hierarchical placement"),
        }
    }
}