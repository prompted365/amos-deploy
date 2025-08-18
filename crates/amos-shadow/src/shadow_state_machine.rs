use crate::{
    ShadowStage, ShadowState, ShadowTransformation, Decision, Goal, CreativeOutput,
    TransformationEvent, ProgressionCriteria,
    DecisionOutcome, GoalStatus, ShadowMetrics, MetricsTracker, AutonomyGradient,
    CapabilityManager
};
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;

/// State machine implementation for shadow transformation
pub struct ShadowStateMachine {
    state: Arc<RwLock<ShadowState>>,
    metrics: Arc<RwLock<ShadowMetrics>>,
    metrics_tracker: Arc<RwLock<MetricsTracker>>,
    autonomy_gradient: Arc<RwLock<AutonomyGradient>>,
    capability_manager: Arc<RwLock<CapabilityManager>>,
}

impl ShadowStateMachine {
    pub fn new() -> Self {
        let initial_stage = ShadowStage::Nascent;
        
        Self {
            state: Arc::new(RwLock::new(ShadowState::new())),
            metrics: Arc::new(RwLock::new(ShadowMetrics::new())),
            metrics_tracker: Arc::new(RwLock::new(MetricsTracker::new())),
            autonomy_gradient: Arc::new(RwLock::new(AutonomyGradient::new(initial_stage))),
            capability_manager: Arc::new(RwLock::new(CapabilityManager::new())),
        }
    }
    
    /// Initialize with a specific stage (for testing or restoration)
    pub fn with_stage(stage: ShadowStage) -> Self {
        let machine = Self::new();
        
        // Update all components to the specified stage
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let mut state = machine.state.write().await;
            state.current_stage = stage;
            
            let mut gradient = machine.autonomy_gradient.write().await;
            gradient.update_for_stage(stage);
            
            let mut capabilities = machine.capability_manager.write().await;
            capabilities.update_for_stage(stage);
        });
        
        machine
    }
    
    /// Process a stage transition attempt
    pub async fn process_transition(&self) -> Result<bool> {
        let mut state = self.state.write().await;
        let metrics = self.metrics.read().await;
        
        // Check if ready for progression
        if !metrics.ready_for_progression(state.current_stage) {
            return Ok(false);
        }
        
        // Attempt progression
        let progressed = state.try_progress();
        
        if progressed {
            // Update other components
            drop(state);
            drop(metrics);
            
            let state = self.state.read().await;
            let mut gradient = self.autonomy_gradient.write().await;
            gradient.update_for_stage(state.current_stage);
            
            let mut capabilities = self.capability_manager.write().await;
            capabilities.update_for_stage(state.current_stage);
            
            // Record metrics snapshot
            let mut tracker = self.metrics_tracker.write().await;
            let metrics = self.metrics.read().await;
            tracker.record(
                metrics.clone(),
                state.current_stage,
                vec![format!("Progressed to {}", state.current_stage)]
            );
        }
        
        Ok(progressed)
    }
    
    /// Update metrics based on agent performance
    pub async fn update_metrics(&self, update: MetricsUpdate) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        
        match update {
            MetricsUpdate::DecisionAccuracy(delta) => {
                metrics.decision_accuracy = (metrics.decision_accuracy + delta).max(0.0).min(1.0);
            },
            MetricsUpdate::LearningRate(delta) => {
                metrics.learning_rate = (metrics.learning_rate + delta).max(0.0).min(1.0);
            },
            MetricsUpdate::CreativityIndex(delta) => {
                metrics.creativity_index = (metrics.creativity_index + delta).max(0.0).min(1.0);
            },
            MetricsUpdate::StabilityScore(delta) => {
                metrics.stability_score = (metrics.stability_score + delta).max(0.0).min(1.0);
            },
            MetricsUpdate::SafetyCompliance(delta) => {
                metrics.safety_compliance = (metrics.safety_compliance + delta).max(0.0).min(1.0);
            },
            MetricsUpdate::AutonomyScore(delta) => {
                metrics.autonomy_score = (metrics.autonomy_score + delta).max(0.0).min(1.0);
            },
        }
        
        // Check for anomalies
        let mut tracker = self.metrics_tracker.write().await;
        let state = self.state.read().await;
        tracker.record(
            metrics.clone(),
            state.current_stage,
            vec![format!("Metrics updated: {:?}", update)]
        );
        
        let anomalies = tracker.detect_anomalies();
        if !anomalies.is_empty() {
            // Handle anomalies (could trigger safety measures)
            for anomaly in anomalies {
                if anomaly.severity == crate::AnomalySeverity::Critical {
                    // Reduce autonomy temporarily
                    metrics.autonomy_score *= 0.8;
                }
            }
        }
        
        Ok(())
    }
    
    /// Get current shadow information
    pub async fn get_shadow_info(&self) -> ShadowInfo {
        let state = self.state.read().await;
        let metrics = self.metrics.read().await;
        let gradient = self.autonomy_gradient.read().await;
        let capabilities = self.capability_manager.read().await;
        
        ShadowInfo {
            shadow_id: state.id,
            current_stage: state.current_stage,
            autonomy_level: state.current_stage.autonomy_percentage(),
            transformation_score: metrics.transformation_score(),
            experience_hours: state.experience_hours(),
            enabled_capabilities: capabilities.enabled_count(),
            safety_violations: state.safety_violations,
            autonomy_overrides: state.autonomy_overrides,
            oversight_level: gradient.oversight_level(),
        }
    }
    
    /// Record a human override of autonomous decision
    pub async fn record_override(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.record_override();
        Ok(())
    }
}

#[async_trait]
impl ShadowTransformation for ShadowStateMachine {
    fn shadow_stage(&self) -> ShadowStage {
        // This is a blocking operation, but it's quick
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.state.read().await.current_stage
            })
        })
    }
    
    fn shadow_id(&self) -> Uuid {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.state.read().await.id
            })
        })
    }
    
    fn progression_criteria(&self) -> &ProgressionCriteria {
        // This doesn't work well with async, so we'll need to refactor
        // For now, return a static reference
        unimplemented!("This method needs refactoring for async access")
    }
    
    async fn attempt_progression(&mut self) -> Result<bool> {
        self.process_transition().await
    }
    
    async fn record_decision(&mut self, decision: Decision) -> Result<()> {
        let mut state = self.state.write().await;
        state.criteria.decisions_made += 1;
        
        // Update metrics based on decision outcome
        if let Some(outcome) = &decision.outcome {
            let accuracy_delta = match outcome {
                DecisionOutcome::Success => 0.01,
                DecisionOutcome::PartialSuccess => 0.005,
                DecisionOutcome::Failure => -0.01,
                DecisionOutcome::Unknown => 0.0,
            };
            
            drop(state);
            self.update_metrics(MetricsUpdate::DecisionAccuracy(accuracy_delta)).await?;
        }
        
        Ok(())
    }
    
    async fn record_goal_achievement(&mut self, goal: Goal) -> Result<()> {
        if goal.status == GoalStatus::Achieved {
            let mut state = self.state.write().await;
            state.criteria.goals_achieved += 1;
            
            // Boost autonomy score for achieving goals
            drop(state);
            self.update_metrics(MetricsUpdate::AutonomyScore(0.02)).await?;
        }
        
        Ok(())
    }
    
    async fn record_pattern_recognition(&mut self, _pattern_id: Uuid) -> Result<()> {
        let mut state = self.state.write().await;
        state.criteria.patterns_recognized += 1;
        
        // Improve learning rate
        drop(state);
        self.update_metrics(MetricsUpdate::LearningRate(0.005)).await?;
        
        Ok(())
    }
    
    async fn record_creative_output(&mut self, output: CreativeOutput) -> Result<()> {
        let mut state = self.state.write().await;
        state.criteria.creative_outputs += 1;
        
        // Update creativity index based on novelty and value
        let creativity_boost = (output.novelty_score * 0.6 + output.value_score * 0.4) * 0.02;
        drop(state);
        self.update_metrics(MetricsUpdate::CreativityIndex(creativity_boost)).await?;
        
        Ok(())
    }
    
    async fn update_autonomy_score(&mut self, delta: f64) -> Result<()> {
        self.update_metrics(MetricsUpdate::AutonomyScore(delta)).await
    }
    
    fn transformation_history(&self) -> &[TransformationEvent] {
        // This doesn't work well with async, so we'll need to refactor
        unimplemented!("This method needs refactoring for async access")
    }
}

/// Types of metrics updates
#[derive(Debug, Clone)]
pub enum MetricsUpdate {
    DecisionAccuracy(f64),
    LearningRate(f64),
    CreativityIndex(f64),
    StabilityScore(f64),
    SafetyCompliance(f64),
    AutonomyScore(f64),
}

/// Shadow information snapshot
#[derive(Debug, Clone)]
pub struct ShadowInfo {
    pub shadow_id: Uuid,
    pub current_stage: ShadowStage,
    pub autonomy_level: f64,
    pub transformation_score: f64,
    pub experience_hours: f64,
    pub enabled_capabilities: usize,
    pub safety_violations: u32,
    pub autonomy_overrides: u32,
    pub oversight_level: crate::OversightLevel,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_state_machine_creation() {
        let machine = ShadowStateMachine::new();
        let info = machine.get_shadow_info().await;
        
        assert_eq!(info.current_stage, ShadowStage::Nascent);
        assert_eq!(info.autonomy_level, 0.05);
        assert_eq!(info.safety_violations, 0);
    }
    
    #[tokio::test]
    async fn test_metrics_update() {
        let machine = ShadowStateMachine::new();
        
        machine.update_metrics(MetricsUpdate::DecisionAccuracy(0.1)).await.unwrap();
        machine.update_metrics(MetricsUpdate::LearningRate(0.2)).await.unwrap();
        
        let metrics = machine.metrics.read().await;
        assert!(metrics.decision_accuracy > 0.0);
        assert!(metrics.learning_rate > 0.0);
    }
    
    #[tokio::test]
    async fn test_override_recording() {
        let machine = ShadowStateMachine::new();
        
        for _ in 0..5 {
            machine.record_override().await.unwrap();
        }
        
        let info = machine.get_shadow_info().await;
        assert_eq!(info.autonomy_overrides, 5);
    }
}