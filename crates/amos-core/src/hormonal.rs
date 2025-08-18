use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HormoneType {
    Cortisol,    // Stress
    Dopamine,    // Reward
    Serotonin,   // Mood
    Oxytocin,    // Bonding
    Adrenaline,  // Fight or flight
}

#[derive(Debug, Clone)]
pub struct HormonalBurst {
    pub id: Uuid,
    pub hormone: HormoneType,
    pub intensity: f64, // 0.0 to 1.0
    pub triggered_at: DateTime<Utc>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone)]
pub struct HormonalState {
    cortisol_level: f64,
    dopamine_level: f64,
    serotonin_level: f64,
    oxytocin_level: f64,
    adrenaline_level: f64,
}

impl HormonalState {
    pub fn new() -> Self {
        Self {
            cortisol_level: 0.5,
            dopamine_level: 0.5,
            serotonin_level: 0.5,
            oxytocin_level: 0.5,
            adrenaline_level: 0.5,
        }
    }

    pub fn apply_burst(&mut self, burst: &HormonalBurst) {
        match burst.hormone {
            HormoneType::Cortisol => self.cortisol_level = (self.cortisol_level + burst.intensity).min(1.0),
            HormoneType::Dopamine => self.dopamine_level = (self.dopamine_level + burst.intensity).min(1.0),
            HormoneType::Serotonin => self.serotonin_level = (self.serotonin_level + burst.intensity).min(1.0),
            HormoneType::Oxytocin => self.oxytocin_level = (self.oxytocin_level + burst.intensity).min(1.0),
            HormoneType::Adrenaline => self.adrenaline_level = (self.adrenaline_level + burst.intensity).min(1.0),
        }
    }

    pub fn decay(&mut self, decay_rate: f64) {
        self.cortisol_level = (self.cortisol_level - decay_rate).max(0.0);
        self.dopamine_level = (self.dopamine_level - decay_rate).max(0.0);
        self.serotonin_level = (self.serotonin_level - decay_rate).max(0.0);
        self.oxytocin_level = (self.oxytocin_level - decay_rate).max(0.0);
        self.adrenaline_level = (self.adrenaline_level - decay_rate).max(0.0);
    }

    pub fn get_level(&self, hormone: &HormoneType) -> f64 {
        match hormone {
            HormoneType::Cortisol => self.cortisol_level,
            HormoneType::Dopamine => self.dopamine_level,
            HormoneType::Serotonin => self.serotonin_level,
            HormoneType::Oxytocin => self.oxytocin_level,
            HormoneType::Adrenaline => self.adrenaline_level,
        }
    }
}

impl Default for HormonalState {
    fn default() -> Self {
        Self::new()
    }
}