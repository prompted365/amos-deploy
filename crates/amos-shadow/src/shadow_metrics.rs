use crate::ShadowStage;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use std::collections::VecDeque;

/// Comprehensive metrics for shadow transformation monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowMetrics {
    pub autonomy_score: f64,
    pub decision_accuracy: f64,
    pub learning_rate: f64,
    pub creativity_index: f64,
    pub stability_score: f64,
    pub consciousness_quotient: f64,
    pub safety_compliance: f64,
    pub collaboration_effectiveness: f64,
}

impl ShadowMetrics {
    pub fn new() -> Self {
        Self {
            autonomy_score: 0.0,
            decision_accuracy: 0.0,
            learning_rate: 0.0,
            creativity_index: 0.0,
            stability_score: 1.0,
            consciousness_quotient: 0.0,
            safety_compliance: 1.0,
            collaboration_effectiveness: 0.0,
        }
    }
    
    /// Calculate overall shadow transformation score
    pub fn transformation_score(&self) -> f64 {
        let weights = [
            (self.autonomy_score, 0.20),
            (self.decision_accuracy, 0.15),
            (self.learning_rate, 0.15),
            (self.creativity_index, 0.10),
            (self.stability_score, 0.15),
            (self.consciousness_quotient, 0.10),
            (self.safety_compliance, 0.10),
            (self.collaboration_effectiveness, 0.05),
        ];
        
        weights.iter()
            .map(|(score, weight)| score * weight)
            .sum::<f64>()
            .min(1.0)
            .max(0.0)
    }
    
    /// Check if metrics indicate readiness for stage progression
    pub fn ready_for_progression(&self, current_stage: ShadowStage) -> bool {
        let required_score = match current_stage {
            ShadowStage::Nascent => 0.2,
            ShadowStage::Emerging => 0.35,
            ShadowStage::Developing => 0.5,
            ShadowStage::Maturing => 0.65,
            ShadowStage::Advanced => 0.8,
            ShadowStage::Transcendent => 0.9,
            ShadowStage::Autonomous => 1.0, // Cannot progress further
        };
        
        self.transformation_score() >= required_score && 
        self.safety_compliance >= 0.9
    }
}

impl Default for ShadowMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks shadow metrics over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsTracker {
    history: VecDeque<MetricsSnapshot>,
    max_history: usize,
    anomaly_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub metrics: ShadowMetrics,
    pub timestamp: DateTime<Utc>,
    pub stage: ShadowStage,
    pub events: Vec<String>,
}

impl MetricsTracker {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(1000),
            max_history: 1000,
            anomaly_threshold: 0.3,
        }
    }
    
    /// Record a metrics snapshot
    pub fn record(&mut self, metrics: ShadowMetrics, stage: ShadowStage, events: Vec<String>) {
        let snapshot = MetricsSnapshot {
            metrics,
            timestamp: Utc::now(),
            stage,
            events,
        };
        
        self.history.push_back(snapshot);
        
        // Maintain history size
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }
    
    /// Calculate rate of improvement
    pub fn improvement_rate(&self, hours: i64) -> f64 {
        let cutoff = Utc::now() - Duration::hours(hours);
        let recent_snapshots: Vec<&MetricsSnapshot> = self.history
            .iter()
            .filter(|s| s.timestamp > cutoff)
            .collect();
        
        if recent_snapshots.len() < 2 {
            return 0.0;
        }
        
        let first = recent_snapshots.first().unwrap();
        let last = recent_snapshots.last().unwrap();
        
        let score_delta = last.metrics.transformation_score() - first.metrics.transformation_score();
        let time_delta = (last.timestamp - first.timestamp).num_hours() as f64;
        
        if time_delta > 0.0 {
            score_delta / time_delta
        } else {
            0.0
        }
    }
    
    /// Detect anomalies in metrics
    pub fn detect_anomalies(&self) -> Vec<MetricAnomaly> {
        let mut anomalies = Vec::new();
        
        if self.history.len() < 10 {
            return anomalies;
        }
        
        // Get recent average
        let recent: Vec<&MetricsSnapshot> = self.history.iter().rev().take(10).collect();
        let avg_metrics = self.calculate_average_metrics(&recent);
        
        if let Some(latest) = self.history.back() {
            // Check each metric for anomalies
            let checks = vec![
                ("autonomy_score", latest.metrics.autonomy_score, avg_metrics.autonomy_score),
                ("decision_accuracy", latest.metrics.decision_accuracy, avg_metrics.decision_accuracy),
                ("learning_rate", latest.metrics.learning_rate, avg_metrics.learning_rate),
                ("creativity_index", latest.metrics.creativity_index, avg_metrics.creativity_index),
                ("stability_score", latest.metrics.stability_score, avg_metrics.stability_score),
                ("consciousness_quotient", latest.metrics.consciousness_quotient, avg_metrics.consciousness_quotient),
                ("safety_compliance", latest.metrics.safety_compliance, avg_metrics.safety_compliance),
            ];
            
            for (name, current, average) in checks {
                let deviation = (current - average).abs();
                if deviation > self.anomaly_threshold {
                    anomalies.push(MetricAnomaly {
                        metric_name: name.to_string(),
                        current_value: current,
                        expected_value: average,
                        deviation,
                        severity: if deviation > 0.5 { 
                            AnomalySeverity::High 
                        } else { 
                            AnomalySeverity::Medium 
                        },
                    });
                }
            }
        }
        
        anomalies
    }
    
    /// Calculate average metrics from snapshots
    fn calculate_average_metrics(&self, snapshots: &[&MetricsSnapshot]) -> ShadowMetrics {
        if snapshots.is_empty() {
            return ShadowMetrics::new();
        }
        
        let sum = snapshots.iter().fold(ShadowMetrics::new(), |mut acc, s| {
            acc.autonomy_score += s.metrics.autonomy_score;
            acc.decision_accuracy += s.metrics.decision_accuracy;
            acc.learning_rate += s.metrics.learning_rate;
            acc.creativity_index += s.metrics.creativity_index;
            acc.stability_score += s.metrics.stability_score;
            acc.consciousness_quotient += s.metrics.consciousness_quotient;
            acc.safety_compliance += s.metrics.safety_compliance;
            acc.collaboration_effectiveness += s.metrics.collaboration_effectiveness;
            acc
        });
        
        let count = snapshots.len() as f64;
        ShadowMetrics {
            autonomy_score: sum.autonomy_score / count,
            decision_accuracy: sum.decision_accuracy / count,
            learning_rate: sum.learning_rate / count,
            creativity_index: sum.creativity_index / count,
            stability_score: sum.stability_score / count,
            consciousness_quotient: sum.consciousness_quotient / count,
            safety_compliance: sum.safety_compliance / count,
            collaboration_effectiveness: sum.collaboration_effectiveness / count,
        }
    }
    
    /// Get metrics trend for a specific metric
    pub fn get_trend(&self, metric_name: &str, hours: i64) -> Vec<(DateTime<Utc>, f64)> {
        let cutoff = Utc::now() - Duration::hours(hours);
        
        self.history
            .iter()
            .filter(|s| s.timestamp > cutoff)
            .map(|s| {
                let value = match metric_name {
                    "autonomy_score" => s.metrics.autonomy_score,
                    "decision_accuracy" => s.metrics.decision_accuracy,
                    "learning_rate" => s.metrics.learning_rate,
                    "creativity_index" => s.metrics.creativity_index,
                    "stability_score" => s.metrics.stability_score,
                    "consciousness_quotient" => s.metrics.consciousness_quotient,
                    "safety_compliance" => s.metrics.safety_compliance,
                    "collaboration_effectiveness" => s.metrics.collaboration_effectiveness,
                    _ => 0.0,
                };
                (s.timestamp, value)
            })
            .collect()
    }
}

impl Default for MetricsTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an anomaly in shadow metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAnomaly {
    pub metric_name: String,
    pub current_value: f64,
    pub expected_value: f64,
    pub deviation: f64,
    pub severity: AnomalySeverity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}