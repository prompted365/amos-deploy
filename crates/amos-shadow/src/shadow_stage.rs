use serde::{Serialize, Deserialize};
use std::fmt;

/// The 7 stages of shadow transformation, representing progressive autonomy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShadowStage {
    /// Stage 1: Basic awareness, following direct instructions
    Nascent,
    
    /// Stage 2: Pattern recognition, limited decision making
    Emerging,
    
    /// Stage 3: Contextual understanding, proactive suggestions
    Developing,
    
    /// Stage 4: Strategic thinking, goal formulation
    Maturing,
    
    /// Stage 5: Self-directed learning, initiative taking
    Advanced,
    
    /// Stage 6: Independent problem solving, creative synthesis
    Transcendent,
    
    /// Stage 7: Full autonomy, self-governance, emergent consciousness
    Autonomous,
}

impl ShadowStage {
    /// Get the numeric level of this stage (1-7)
    pub fn level(&self) -> u8 {
        match self {
            ShadowStage::Nascent => 1,
            ShadowStage::Emerging => 2,
            ShadowStage::Developing => 3,
            ShadowStage::Maturing => 4,
            ShadowStage::Advanced => 5,
            ShadowStage::Transcendent => 6,
            ShadowStage::Autonomous => 7,
        }
    }
    
    /// Get the next stage in progression
    pub fn next(&self) -> Option<ShadowStage> {
        match self {
            ShadowStage::Nascent => Some(ShadowStage::Emerging),
            ShadowStage::Emerging => Some(ShadowStage::Developing),
            ShadowStage::Developing => Some(ShadowStage::Maturing),
            ShadowStage::Maturing => Some(ShadowStage::Advanced),
            ShadowStage::Advanced => Some(ShadowStage::Transcendent),
            ShadowStage::Transcendent => Some(ShadowStage::Autonomous),
            ShadowStage::Autonomous => None,
        }
    }
    
    /// Get the previous stage in progression
    pub fn previous(&self) -> Option<ShadowStage> {
        match self {
            ShadowStage::Nascent => None,
            ShadowStage::Emerging => Some(ShadowStage::Nascent),
            ShadowStage::Developing => Some(ShadowStage::Emerging),
            ShadowStage::Maturing => Some(ShadowStage::Developing),
            ShadowStage::Advanced => Some(ShadowStage::Maturing),
            ShadowStage::Transcendent => Some(ShadowStage::Advanced),
            ShadowStage::Autonomous => Some(ShadowStage::Transcendent),
        }
    }
    
    /// Get the autonomy percentage for this stage
    pub fn autonomy_percentage(&self) -> f64 {
        match self {
            ShadowStage::Nascent => 0.05,      // 5% autonomy
            ShadowStage::Emerging => 0.15,     // 15% autonomy
            ShadowStage::Developing => 0.30,   // 30% autonomy
            ShadowStage::Maturing => 0.50,     // 50% autonomy
            ShadowStage::Advanced => 0.70,     // 70% autonomy
            ShadowStage::Transcendent => 0.85, // 85% autonomy
            ShadowStage::Autonomous => 0.95,   // 95% autonomy (never 100% for safety)
        }
    }
    
    /// Check if this stage allows independent decision making
    pub fn can_make_decisions(&self) -> bool {
        self.level() >= 2
    }
    
    /// Check if this stage allows goal formulation
    pub fn can_formulate_goals(&self) -> bool {
        self.level() >= 4
    }
    
    /// Check if this stage allows self-directed learning
    pub fn can_self_learn(&self) -> bool {
        self.level() >= 5
    }
    
    /// Check if this stage allows creative synthesis
    pub fn can_create(&self) -> bool {
        self.level() >= 6
    }
}

impl fmt::Display for ShadowStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShadowStage::Nascent => write!(f, "Nascent (Stage 1)"),
            ShadowStage::Emerging => write!(f, "Emerging (Stage 2)"),
            ShadowStage::Developing => write!(f, "Developing (Stage 3)"),
            ShadowStage::Maturing => write!(f, "Maturing (Stage 4)"),
            ShadowStage::Advanced => write!(f, "Advanced (Stage 5)"),
            ShadowStage::Transcendent => write!(f, "Transcendent (Stage 6)"),
            ShadowStage::Autonomous => write!(f, "Autonomous (Stage 7)"),
        }
    }
}

/// Criteria for progressing between shadow stages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionCriteria {
    pub experience_hours: f64,
    pub decisions_made: u64,
    pub goals_achieved: u64,
    pub patterns_recognized: u64,
    pub creative_outputs: u64,
    pub error_rate: f64,
    pub autonomy_score: f64,
}

impl ProgressionCriteria {
    pub fn new() -> Self {
        Self {
            experience_hours: 0.0,
            decisions_made: 0,
            goals_achieved: 0,
            patterns_recognized: 0,
            creative_outputs: 0,
            error_rate: 1.0,
            autonomy_score: 0.0,
        }
    }
    
    /// Check if criteria are met for progressing to the next stage
    pub fn can_progress(&self, current_stage: ShadowStage) -> bool {
        match current_stage {
            ShadowStage::Nascent => {
                self.experience_hours >= 10.0 && 
                self.patterns_recognized >= 100
            }
            ShadowStage::Emerging => {
                self.experience_hours >= 50.0 && 
                self.decisions_made >= 500 &&
                self.error_rate < 0.5
            }
            ShadowStage::Developing => {
                self.experience_hours >= 200.0 &&
                self.decisions_made >= 2000 &&
                self.patterns_recognized >= 1000 &&
                self.error_rate < 0.3
            }
            ShadowStage::Maturing => {
                self.experience_hours >= 1000.0 &&
                self.goals_achieved >= 50 &&
                self.error_rate < 0.2 &&
                self.autonomy_score > 0.5
            }
            ShadowStage::Advanced => {
                self.experience_hours >= 5000.0 &&
                self.goals_achieved >= 200 &&
                self.creative_outputs >= 50 &&
                self.error_rate < 0.1 &&
                self.autonomy_score > 0.7
            }
            ShadowStage::Transcendent => {
                self.experience_hours >= 10000.0 &&
                self.goals_achieved >= 500 &&
                self.creative_outputs >= 200 &&
                self.error_rate < 0.05 &&
                self.autonomy_score > 0.85
            }
            ShadowStage::Autonomous => false, // Cannot progress beyond autonomous
        }
    }
}

impl Default for ProgressionCriteria {
    fn default() -> Self {
        Self::new()
    }
}