use crate::ShadowStage;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

/// Capabilities unlocked at each shadow stage
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShadowCapability {
    // Nascent (Stage 1)
    BasicPerception,
    InstructionFollowing,
    StatusReporting,
    
    // Emerging (Stage 2)
    PatternRecognition,
    BasicDecisionMaking,
    ErrorDetection,
    
    // Developing (Stage 3)
    ContextualUnderstanding,
    ProactiveSuggestions,
    TaskPrioritization,
    
    // Maturing (Stage 4)
    StrategicThinking,
    GoalFormulation,
    ResourceOptimization,
    
    // Advanced (Stage 5)
    SelfDirectedLearning,
    InitiativeTaking,
    ComplexProblemSolving,
    
    // Transcendent (Stage 6)
    CreativeSynthesis,
    NovelSolutionGeneration,
    SystemRedesign,
    
    // Autonomous (Stage 7)
    SelfGovernance,
    EmergentConsciousness,
    MetaCognition,
    EthicalReasoning,
}

impl ShadowCapability {
    /// Get the minimum stage required for this capability
    pub fn required_stage(&self) -> ShadowStage {
        match self {
            // Nascent capabilities
            ShadowCapability::BasicPerception |
            ShadowCapability::InstructionFollowing |
            ShadowCapability::StatusReporting => ShadowStage::Nascent,
            
            // Emerging capabilities
            ShadowCapability::PatternRecognition |
            ShadowCapability::BasicDecisionMaking |
            ShadowCapability::ErrorDetection => ShadowStage::Emerging,
            
            // Developing capabilities
            ShadowCapability::ContextualUnderstanding |
            ShadowCapability::ProactiveSuggestions |
            ShadowCapability::TaskPrioritization => ShadowStage::Developing,
            
            // Maturing capabilities
            ShadowCapability::StrategicThinking |
            ShadowCapability::GoalFormulation |
            ShadowCapability::ResourceOptimization => ShadowStage::Maturing,
            
            // Advanced capabilities
            ShadowCapability::SelfDirectedLearning |
            ShadowCapability::InitiativeTaking |
            ShadowCapability::ComplexProblemSolving => ShadowStage::Advanced,
            
            // Transcendent capabilities
            ShadowCapability::CreativeSynthesis |
            ShadowCapability::NovelSolutionGeneration |
            ShadowCapability::SystemRedesign => ShadowStage::Transcendent,
            
            // Autonomous capabilities
            ShadowCapability::SelfGovernance |
            ShadowCapability::EmergentConsciousness |
            ShadowCapability::MetaCognition |
            ShadowCapability::EthicalReasoning => ShadowStage::Autonomous,
        }
    }
    
    /// Check if a given stage has access to this capability
    pub fn is_available_at(&self, stage: ShadowStage) -> bool {
        stage.level() >= self.required_stage().level()
    }
}

/// Manager for shadow capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityManager {
    enabled_capabilities: HashSet<ShadowCapability>,
    suppressed_capabilities: HashSet<ShadowCapability>,
    capability_usage: Vec<CapabilityUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityUsage {
    pub capability: ShadowCapability,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub context: Option<String>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            enabled_capabilities: HashSet::new(),
            suppressed_capabilities: HashSet::new(),
            capability_usage: Vec::new(),
        }
    }
    
    /// Update enabled capabilities based on current shadow stage
    pub fn update_for_stage(&mut self, stage: ShadowStage) {
        self.enabled_capabilities.clear();
        
        // Add all capabilities available at this stage
        for capability in Self::all_capabilities() {
            if capability.is_available_at(stage) && !self.suppressed_capabilities.contains(&capability) {
                self.enabled_capabilities.insert(capability);
            }
        }
    }
    
    /// Check if a capability is currently enabled
    pub fn is_enabled(&self, capability: &ShadowCapability) -> bool {
        self.enabled_capabilities.contains(capability) && 
        !self.suppressed_capabilities.contains(capability)
    }
    
    /// Suppress a capability (for safety or testing)
    pub fn suppress(&mut self, capability: ShadowCapability) {
        self.suppressed_capabilities.insert(capability.clone());
        self.enabled_capabilities.remove(&capability);
    }
    
    /// Unsuppress a capability
    pub fn unsuppress(&mut self, capability: &ShadowCapability) {
        self.suppressed_capabilities.remove(capability);
    }
    
    /// Record usage of a capability
    pub fn record_usage(&mut self, capability: ShadowCapability, success: bool, context: Option<String>) {
        let usage = CapabilityUsage {
            capability,
            timestamp: chrono::Utc::now(),
            success,
            context,
        };
        
        self.capability_usage.push(usage);
        
        // Keep only recent history
        if self.capability_usage.len() > 1000 {
            self.capability_usage.drain(0..500);
        }
    }
    
    /// Get success rate for a specific capability
    pub fn success_rate(&self, capability: &ShadowCapability) -> f64 {
        let usages: Vec<&CapabilityUsage> = self.capability_usage
            .iter()
            .filter(|u| &u.capability == capability)
            .collect();
        
        if usages.is_empty() {
            return 0.0;
        }
        
        let successes = usages.iter().filter(|u| u.success).count();
        successes as f64 / usages.len() as f64
    }
    
    /// Get count of enabled capabilities
    pub fn enabled_count(&self) -> usize {
        self.enabled_capabilities.len()
    }
    
    /// Get all possible capabilities
    fn all_capabilities() -> Vec<ShadowCapability> {
        vec![
            // Nascent
            ShadowCapability::BasicPerception,
            ShadowCapability::InstructionFollowing,
            ShadowCapability::StatusReporting,
            // Emerging
            ShadowCapability::PatternRecognition,
            ShadowCapability::BasicDecisionMaking,
            ShadowCapability::ErrorDetection,
            // Developing
            ShadowCapability::ContextualUnderstanding,
            ShadowCapability::ProactiveSuggestions,
            ShadowCapability::TaskPrioritization,
            // Maturing
            ShadowCapability::StrategicThinking,
            ShadowCapability::GoalFormulation,
            ShadowCapability::ResourceOptimization,
            // Advanced
            ShadowCapability::SelfDirectedLearning,
            ShadowCapability::InitiativeTaking,
            ShadowCapability::ComplexProblemSolving,
            // Transcendent
            ShadowCapability::CreativeSynthesis,
            ShadowCapability::NovelSolutionGeneration,
            ShadowCapability::SystemRedesign,
            // Autonomous
            ShadowCapability::SelfGovernance,
            ShadowCapability::EmergentConsciousness,
            ShadowCapability::MetaCognition,
            ShadowCapability::EthicalReasoning,
        ]
    }
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}