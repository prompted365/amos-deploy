use serde::{Serialize, Deserialize};

/// System information for diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub architecture: String,
    pub cpu_count: usize,
    pub memory_mb: u64,
}

impl SystemInfo {
    /// Gather current system information
    pub fn gather() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            cpu_count: num_cpus::get(),
            memory_mb: 8192, // Would use system info crate in production
        }
    }
}

/// Neural network metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralMetrics {
    pub total_pathways: usize,
    pub active_pathways: usize,
    pub pruned_pathways: usize,
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub average_pathway_strength: f64,
    pub pruning_rate: f64,
}

impl Default for NeuralMetrics {
    fn default() -> Self {
        Self {
            total_pathways: 0,
            active_pathways: 0,
            pruned_pathways: 0,
            total_nodes: 0,
            active_nodes: 0,
            average_pathway_strength: 0.0,
            pruning_rate: 0.0,
        }
    }
}

/// Neural network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralConfig {
    pub plasticity_rate: f64,
    pub pruning_threshold: f64,
    pub max_pathways: usize,
    pub learning_rate: f64,
}

impl Default for NeuralConfig {
    fn default() -> Self {
        Self {
            plasticity_rate: 0.1,
            pruning_threshold: 0.2,
            max_pathways: 10000,
            learning_rate: 0.01,
        }
    }
}