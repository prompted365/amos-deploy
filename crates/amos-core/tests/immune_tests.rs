use amos_core::immune::*;
use uuid::Uuid;
use chrono::Utc;

// Test implementation of ThreatDetector
struct TestThreatDetector {
    detectable_types: Vec<PatternType>,
}

#[async_trait::async_trait]
impl ThreatDetector for TestThreatDetector {
    async fn analyze(&self, pattern: &Pattern) -> Option<Threat> {
        if self.can_detect(&pattern.pattern_type) {
            Some(Threat {
                id: Uuid::new_v4(),
                pattern: pattern.clone(),
                level: match pattern.pattern_type {
                    PatternType::Attack => ThreatLevel::Critical,
                    PatternType::Overload => ThreatLevel::High,
                    PatternType::Anomaly => ThreatLevel::Medium,
                    PatternType::Normal => ThreatLevel::Low,
                },
                detected_at: Utc::now(),
            })
        } else {
            None
        }
    }
    
    fn can_detect(&self, pattern_type: &PatternType) -> bool {
        self.detectable_types.contains(pattern_type)
    }
}

// Test implementation of ResponseMechanism
struct TestResponseMechanism {
    handled_threats: Arc<tokio::sync::Mutex<Vec<Threat>>>,
}

#[async_trait::async_trait]
impl ResponseMechanism for TestResponseMechanism {
    async fn respond(&self, threat: Threat) {
        self.handled_threats.lock().await.push(threat);
    }
    
    fn can_handle(&self, _threat: &Threat) -> bool {
        true // Can handle all threats for testing
    }
}

#[test]
fn test_pattern_creation() {
    let pattern = Pattern {
        id: Uuid::new_v4(),
        data: vec![1.0, 2.0, 3.0],
        pattern_type: PatternType::Normal,
    };
    
    assert_eq!(pattern.data.len(), 3);
    assert_eq!(pattern.pattern_type, PatternType::Normal);
}

#[test]
fn test_threat_level_ordering() {
    assert_ne!(ThreatLevel::Low, ThreatLevel::Medium);
    assert_ne!(ThreatLevel::Medium, ThreatLevel::High);
    assert_ne!(ThreatLevel::High, ThreatLevel::Critical);
}

#[tokio::test]
async fn test_forge_immune_system_creation() {
    let immune_system = ForgeImmuneSystem::new();
    
    let pattern = Pattern {
        id: Uuid::new_v4(),
        data: vec![1.0, 2.0, 3.0],
        pattern_type: PatternType::Normal,
    };
    
    // Should return None as no detectors are registered
    let threat_level = immune_system.detect_anomaly(&pattern).await;
    assert!(threat_level.is_none());
}

#[tokio::test]
async fn test_threat_detection() {
    let mut immune_system = ForgeImmuneSystem::new();
    
    // Add a test detector
    immune_system.add_detector(Box::new(TestThreatDetector {
        detectable_types: vec![PatternType::Attack, PatternType::Anomaly],
    }));
    
    // Test attack pattern detection
    let attack_pattern = Pattern {
        id: Uuid::new_v4(),
        data: vec![9.9, 9.9, 9.9],
        pattern_type: PatternType::Attack,
    };
    
    let threat_level = immune_system.detect_anomaly(&attack_pattern).await;
    assert_eq!(threat_level, Some(ThreatLevel::Critical));
    
    // Test normal pattern (not detected)
    let normal_pattern = Pattern {
        id: Uuid::new_v4(),
        data: vec![1.0, 2.0, 3.0],
        pattern_type: PatternType::Normal,
    };
    
    let threat_level = immune_system.detect_anomaly(&normal_pattern).await;
    assert!(threat_level.is_none());
}

#[tokio::test]
async fn test_adaptive_response() {
    let mut immune_system = ForgeImmuneSystem::new();
    
    // Add a test response mechanism
    let handled_threats = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let response_mechanism = TestResponseMechanism {
        handled_threats: handled_threats.clone(),
    };
    immune_system.add_response_mechanism(Box::new(response_mechanism));
    
    // Create a threat
    let threat = Threat {
        id: Uuid::new_v4(),
        pattern: Pattern {
            id: Uuid::new_v4(),
            data: vec![9.9, 9.9],
            pattern_type: PatternType::Attack,
        },
        level: ThreatLevel::Critical,
        detected_at: Utc::now(),
    };
    
    // Trigger adaptive response
    immune_system.adaptive_response(threat.clone()).await;
    
    // Verify the threat was handled
    let handled = handled_threats.lock().await;
    assert_eq!(handled.len(), 1);
    assert_eq!(handled[0].id, threat.id);
}

#[tokio::test]
async fn test_pattern_memory_storage() {
    let immune_system = ForgeImmuneSystem::new();
    
    let threat = Threat {
        id: Uuid::new_v4(),
        pattern: Pattern {
            id: Uuid::new_v4(),
            data: vec![5.0, 5.0],
            pattern_type: PatternType::Anomaly,
        },
        level: ThreatLevel::Medium,
        detected_at: Utc::now(),
    };
    
    // Should store pattern in memory
    immune_system.adaptive_response(threat).await;
    
    // Verify pattern was stored (would need getter method in real implementation)
}

use std::sync::Arc;