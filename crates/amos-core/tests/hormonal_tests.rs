use amos_core::hormonal::*;
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_hormonal_state_creation() {
    let state = HormonalState::new();
    
    // All hormones should start at baseline (0.5)
    assert_eq!(state.get_level(&HormoneType::Cortisol), 0.5);
    assert_eq!(state.get_level(&HormoneType::Dopamine), 0.5);
    assert_eq!(state.get_level(&HormoneType::Serotonin), 0.5);
    assert_eq!(state.get_level(&HormoneType::Oxytocin), 0.5);
    assert_eq!(state.get_level(&HormoneType::Adrenaline), 0.5);
}

#[test]
fn test_hormonal_burst_application() {
    let mut state = HormonalState::new();
    
    let burst = HormonalBurst {
        id: Uuid::new_v4(),
        hormone: HormoneType::Dopamine,
        intensity: 0.3,
        triggered_at: Utc::now(),
        duration_ms: 5000,
    };
    
    state.apply_burst(&burst);
    
    assert_eq!(state.get_level(&HormoneType::Dopamine), 0.8); // 0.5 + 0.3
    assert_eq!(state.get_level(&HormoneType::Cortisol), 0.5); // Unchanged
}

#[test]
fn test_hormonal_burst_clamping() {
    let mut state = HormonalState::new();
    
    let burst = HormonalBurst {
        id: Uuid::new_v4(),
        hormone: HormoneType::Adrenaline,
        intensity: 0.7, // Would exceed 1.0
        triggered_at: Utc::now(),
        duration_ms: 5000,
    };
    
    state.apply_burst(&burst);
    
    assert_eq!(state.get_level(&HormoneType::Adrenaline), 1.0); // Clamped at 1.0
}

#[test]
fn test_hormonal_decay() {
    let mut state = HormonalState::new();
    
    // First apply a burst
    let burst = HormonalBurst {
        id: Uuid::new_v4(),
        hormone: HormoneType::Cortisol,
        intensity: 0.4,
        triggered_at: Utc::now(),
        duration_ms: 5000,
    };
    
    state.apply_burst(&burst);
    assert_eq!(state.get_level(&HormoneType::Cortisol), 0.9);
    
    // Apply decay
    state.decay(0.2);
    
    assert_eq!(state.get_level(&HormoneType::Cortisol), 0.7); // 0.9 - 0.2
    assert_eq!(state.get_level(&HormoneType::Dopamine), 0.3); // 0.5 - 0.2
}

#[test]
fn test_hormonal_decay_floor() {
    let mut state = HormonalState::new();
    
    // Apply large decay
    state.decay(0.7); // Would go below 0.0
    
    // All hormones should be floored at 0.0
    assert_eq!(state.get_level(&HormoneType::Cortisol), 0.0);
    assert_eq!(state.get_level(&HormoneType::Dopamine), 0.0);
    assert_eq!(state.get_level(&HormoneType::Serotonin), 0.0);
    assert_eq!(state.get_level(&HormoneType::Oxytocin), 0.0);
    assert_eq!(state.get_level(&HormoneType::Adrenaline), 0.0);
}

#[test]
fn test_hormone_type_equality() {
    assert_eq!(HormoneType::Dopamine, HormoneType::Dopamine);
    assert_ne!(HormoneType::Dopamine, HormoneType::Serotonin);
}

#[test]
fn test_multiple_bursts() {
    let mut state = HormonalState::new();
    
    // Apply multiple bursts
    let burst1 = HormonalBurst {
        id: Uuid::new_v4(),
        hormone: HormoneType::Dopamine,
        intensity: 0.2,
        triggered_at: Utc::now(),
        duration_ms: 5000,
    };
    
    let burst2 = HormonalBurst {
        id: Uuid::new_v4(),
        hormone: HormoneType::Dopamine,
        intensity: 0.1,
        triggered_at: Utc::now(),
        duration_ms: 5000,
    };
    
    state.apply_burst(&burst1);
    state.apply_burst(&burst2);
    
    assert!((state.get_level(&HormoneType::Dopamine) - 0.8).abs() < 0.0001); // 0.5 + 0.2 + 0.1
}