use crate::{ShadowStage, ShadowMetrics, ShadowCapability};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;

/// Manages the gradient of autonomy levels across shadow stages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyGradient {
    stage: ShadowStage,
    autonomy_level: f64,
    decision_thresholds: HashMap<String, f64>,
    capability_weights: HashMap<ShadowCapability, f64>,
    safety_constraints: SafetyConstraints,
}

/// Safety constraints that limit autonomy based on risk levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConstraints {
    max_risk_tolerance: f64,
    require_human_approval_above: f64,
    emergency_shutdown_threshold: f64,
    ethical_compliance_minimum: f64,
}

impl SafetyConstraints {
    pub fn new() -> Self {
        Self {
            max_risk_tolerance: 0.3,
            require_human_approval_above: 0.8,
            emergency_shutdown_threshold: 0.95,
            ethical_compliance_minimum: 0.9,
        }
    }
    
    /// Check if an action is within safety constraints
    pub fn is_action_safe(&self, risk_level: f64, autonomy_level: f64) -> bool {
        risk_level <= self.max_risk_tolerance && 
        (autonomy_level <= self.require_human_approval_above || risk_level < 0.1)
    }
}

impl Default for SafetyConstraints {
    fn default() -> Self {
        Self::new()
    }
}

impl AutonomyGradient {
    pub fn new(stage: ShadowStage) -> Self {
        let mut gradient = Self {
            stage,
            autonomy_level: stage.autonomy_percentage(),
            decision_thresholds: HashMap::new(),
            capability_weights: HashMap::new(),
            safety_constraints: SafetyConstraints::new(),
        };
        
        gradient.initialize_thresholds();
        gradient.initialize_capability_weights();
        gradient
    }
    
    /// Initialize decision thresholds based on stage
    fn initialize_thresholds(&mut self) {
        let base_threshold = 1.0 - self.autonomy_level;
        
        self.decision_thresholds.insert("routine_operations".to_string(), base_threshold * 0.5);
        self.decision_thresholds.insert("resource_allocation".to_string(), base_threshold * 0.7);
        self.decision_thresholds.insert("strategic_planning".to_string(), base_threshold * 0.9);
        self.decision_thresholds.insert("system_modification".to_string(), base_threshold * 1.2);
        self.decision_thresholds.insert("ethical_decisions".to_string(), base_threshold * 1.5);
    }
    
    /// Initialize capability weights based on stage
    fn initialize_capability_weights(&mut self) {
        // Weight capabilities based on their importance at this stage
        match self.stage {
            ShadowStage::Nascent => {
                self.capability_weights.insert(ShadowCapability::BasicPerception, 1.0);
                self.capability_weights.insert(ShadowCapability::InstructionFollowing, 0.9);
                self.capability_weights.insert(ShadowCapability::StatusReporting, 0.8);
            },
            ShadowStage::Emerging => {
                self.capability_weights.insert(ShadowCapability::PatternRecognition, 1.0);
                self.capability_weights.insert(ShadowCapability::BasicDecisionMaking, 0.9);
                self.capability_weights.insert(ShadowCapability::ErrorDetection, 0.8);
            },
            ShadowStage::Developing => {
                self.capability_weights.insert(ShadowCapability::ContextualUnderstanding, 1.0);
                self.capability_weights.insert(ShadowCapability::ProactiveSuggestions, 0.9);
                self.capability_weights.insert(ShadowCapability::TaskPrioritization, 0.8);
            },
            ShadowStage::Maturing => {
                self.capability_weights.insert(ShadowCapability::StrategicThinking, 1.0);
                self.capability_weights.insert(ShadowCapability::GoalFormulation, 0.9);
                self.capability_weights.insert(ShadowCapability::ResourceOptimization, 0.8);
            },
            ShadowStage::Advanced => {
                self.capability_weights.insert(ShadowCapability::SelfDirectedLearning, 1.0);
                self.capability_weights.insert(ShadowCapability::InitiativeTaking, 0.9);
                self.capability_weights.insert(ShadowCapability::ComplexProblemSolving, 0.8);
            },
            ShadowStage::Transcendent => {
                self.capability_weights.insert(ShadowCapability::CreativeSynthesis, 1.0);
                self.capability_weights.insert(ShadowCapability::NovelSolutionGeneration, 0.9);
                self.capability_weights.insert(ShadowCapability::SystemRedesign, 0.8);
            },
            ShadowStage::Autonomous => {
                self.capability_weights.insert(ShadowCapability::SelfGovernance, 1.0);
                self.capability_weights.insert(ShadowCapability::EmergentConsciousness, 0.9);
                self.capability_weights.insert(ShadowCapability::MetaCognition, 0.9);
                self.capability_weights.insert(ShadowCapability::EthicalReasoning, 1.0);
            },
        }
    }
    
    /// Calculate autonomy score for a specific decision type
    pub fn calculate_decision_autonomy(&self, decision_type: &str, metrics: &ShadowMetrics) -> f64 {
        let threshold = self.decision_thresholds
            .get(decision_type)
            .unwrap_or(&0.5);
        
        let base_autonomy = self.autonomy_level;
        let metric_modifier = metrics.transformation_score();
        let safety_modifier = metrics.safety_compliance;
        
        (base_autonomy * metric_modifier * safety_modifier).min(1.0 - threshold)
    }
    
    /// Check if agent can make autonomous decision
    pub fn can_decide_autonomously(
        &self, 
        decision_type: &str, 
        risk_level: f64,
        metrics: &ShadowMetrics
    ) -> Result<bool> {
        // Check safety constraints first
        if !self.safety_constraints.is_action_safe(risk_level, self.autonomy_level) {
            return Ok(false);
        }
        
        // Calculate decision autonomy
        let decision_autonomy = self.calculate_decision_autonomy(decision_type, metrics);
        
        // Check if we meet the threshold
        let threshold = self.decision_thresholds
            .get(decision_type)
            .unwrap_or(&0.5);
        
        Ok(decision_autonomy >= *threshold)
    }
    
    /// Update autonomy gradient based on new stage
    pub fn update_for_stage(&mut self, new_stage: ShadowStage) {
        self.stage = new_stage;
        self.autonomy_level = new_stage.autonomy_percentage();
        self.initialize_thresholds();
        self.initialize_capability_weights();
    }
    
    /// Apply capability-based modulation to autonomy
    pub fn modulate_by_capability(
        &self, 
        capability: &ShadowCapability,
        base_autonomy: f64
    ) -> f64 {
        let weight = self.capability_weights
            .get(capability)
            .unwrap_or(&0.5);
        
        (base_autonomy * weight).min(self.autonomy_level)
    }
    
    /// Get recommended human oversight level
    pub fn oversight_level(&self) -> OversightLevel {
        match self.stage {
            ShadowStage::Nascent => OversightLevel::Direct,
            ShadowStage::Emerging => OversightLevel::Active,
            ShadowStage::Developing => OversightLevel::Periodic,
            ShadowStage::Maturing => OversightLevel::OnDemand,
            ShadowStage::Advanced => OversightLevel::Minimal,
            ShadowStage::Transcendent => OversightLevel::Exception,
            ShadowStage::Autonomous => OversightLevel::Audit,
        }
    }
    
    /// Calculate time until next autonomy increase
    pub fn time_to_next_increase(&self, current_metrics: &ShadowMetrics) -> Option<f64> {
        if self.stage == ShadowStage::Autonomous {
            return None;
        }
        
        let progress = current_metrics.transformation_score();
        let required = match self.stage {
            ShadowStage::Nascent => 0.2,
            ShadowStage::Emerging => 0.35,
            ShadowStage::Developing => 0.5,
            ShadowStage::Maturing => 0.65,
            ShadowStage::Advanced => 0.8,
            ShadowStage::Transcendent => 0.9,
            ShadowStage::Autonomous => 1.0,
        };
        
        if progress >= required {
            Some(0.0)
        } else {
            // Estimate based on learning rate
            let rate = current_metrics.learning_rate.max(0.01);
            Some((required - progress) / rate * 24.0) // Hours
        }
    }
}

/// Level of human oversight required
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OversightLevel {
    Direct,    // Constant supervision
    Active,    // Regular monitoring
    Periodic,  // Scheduled check-ins
    OnDemand,  // Available when needed
    Minimal,   // Rare intervention
    Exception, // Only in emergencies
    Audit,     // Post-hoc review only
}

/// Gradient transition event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientTransition {
    pub from_level: f64,
    pub to_level: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trigger: TransitionTrigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionTrigger {
    StageProgression,
    MetricImprovement,
    CapabilityUnlock,
    SafetyOverride,
    ManualAdjustment,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_autonomy_gradient_creation() {
        let gradient = AutonomyGradient::new(ShadowStage::Developing);
        assert_eq!(gradient.autonomy_level, 0.30);
        assert!(!gradient.decision_thresholds.is_empty());
        assert!(!gradient.capability_weights.is_empty());
    }
    
    #[test]
    fn test_safety_constraints() {
        let constraints = SafetyConstraints::new();
        
        // Low risk, low autonomy - should be safe
        assert!(constraints.is_action_safe(0.1, 0.3));
        
        // High risk - should not be safe
        assert!(!constraints.is_action_safe(0.5, 0.3));
        
        // High autonomy, low risk - should be safe
        assert!(constraints.is_action_safe(0.05, 0.9));
    }
    
    #[test]
    fn test_decision_autonomy() {
        let gradient = AutonomyGradient::new(ShadowStage::Maturing);
        let metrics = ShadowMetrics {
            autonomy_score: 0.6,
            decision_accuracy: 0.8,
            learning_rate: 0.5,
            creativity_index: 0.4,
            stability_score: 0.9,
            consciousness_quotient: 0.3,
            safety_compliance: 0.95,
            collaboration_effectiveness: 0.7,
        };
        
        let autonomy = gradient.calculate_decision_autonomy("routine_operations", &metrics);
        assert!(autonomy > 0.0);
        assert!(autonomy < 1.0);
    }
    
    #[test]
    fn test_oversight_levels() {
        assert_eq!(AutonomyGradient::new(ShadowStage::Nascent).oversight_level(), OversightLevel::Direct);
        assert_eq!(AutonomyGradient::new(ShadowStage::Advanced).oversight_level(), OversightLevel::Minimal);
        assert_eq!(AutonomyGradient::new(ShadowStage::Autonomous).oversight_level(), OversightLevel::Audit);
    }
}