//! Benchmark runner for lattice snapping comparison.

use std::collections::HashSet;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::lattice::{snap_eisenstein, snap_z2};

/// Result of a single benchmark trial.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialResult {
    pub n: usize,
    pub trial: usize,
    pub lattice: String,
    pub mean_error: f64,
    pub std_error: f64,
    pub max_error: f64,
    pub median_error: f64,
    pub packing_unique: usize,
    pub packing_ratio: f64,
    pub recovery_001: f64,
    pub recovery_01: f64,
    pub recovery_05: f64,
    pub error_p95: f64,
    pub error_p99: f64,
    pub elapsed_s: f64,
}

/// Aggregated results across trials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedResult {
    pub n: usize,
    pub lattice: String,
    pub mean_error_mean: f64,
    pub mean_error_std: f64,
    pub median_error_mean: f64,
    pub max_error_mean: f64,
    pub packing_ratio_mean: f64,
    pub packing_ratio_std: f64,
    pub recovery_01_mean: f64,
    pub recovery_01_std: f64,
    pub recovery_05_mean: f64,
    pub recovery_05_std: f64,
    pub error_p95_mean: f64,
    pub error_p99_mean: f64,
    pub elapsed_s_mean: f64,
}

/// Simple LCG random number generator (no external deps).
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_f64(&mut self) -> f64 {
        // LCG parameters (same as glibc)
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state >> 33) as f64 / (1u64 << 31) as f64
    }

    pub fn uniform(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * self.next_f64()
    }
}

/// Run a single trial.
pub fn run_trial(
    n: usize,
    trial_idx: usize,
    lattice_type: &str,
    rng: &mut SimpleRng,
) -> TrialResult {
    // Generate random points in [-10, 10]²
    let points: Vec<(f64, f64)> = (0..n)
        .map(|_| (rng.uniform(-10.0, 10.0), rng.uniform(-10.0, 10.0)))
        .collect();

    let start = Instant::now();

    let errors: Vec<f64> = points
        .iter()
        .map(|&(x, y)| match lattice_type {
            "eisenstein" => snap_eisenstein(x, y).error,
            _ => snap_z2(x, y).error,
        })
        .collect();

    let mut unique_coords: HashSet<(i64, i64)> = HashSet::new();
    for &(x, y) in &points {
        match lattice_type {
            "eisenstein" => {
                let r = snap_eisenstein(x, y);
                unique_coords.insert(r.lattice_coords);
            }
            _ => {
                let r = snap_z2(x, y);
                unique_coords.insert(r.lattice_coords);
            }
        }
    }

    let elapsed = start.elapsed().as_secs_f64();

    let mut sorted_errors = errors.clone();
    sorted_errors.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mean = errors.iter().sum::<f64>() / errors.len() as f64;
    let variance = errors.iter().map(|e| (e - mean).powi(2)).sum::<f64>() / errors.len() as f64;
    let std_dev = variance.sqrt();
    let median = sorted_errors[sorted_errors.len() / 2];
    let max = sorted_errors.last().copied().unwrap_or(0.0);

    let p95_idx = (errors.len() as f64 * 0.95) as usize;
    let p99_idx = (errors.len() as f64 * 0.99) as usize;
    let p95 = sorted_errors
        .get(p95_idx.min(&sorted_errors.len() - 1))
        .copied()
        .unwrap_or(0.0);
    let p99 = sorted_errors
        .get(p99_idx.min(&sorted_errors.len() - 1))
        .copied()
        .unwrap_or(0.0);

    let recovery_001 = errors.iter().filter(|&&e| e <= 0.01).count() as f64 / errors.len() as f64;
    let recovery_01 = errors.iter().filter(|&&e| e <= 0.1).count() as f64 / errors.len() as f64;
    let recovery_05 = errors.iter().filter(|&&e| e <= 0.5).count() as f64 / errors.len() as f64;

    TrialResult {
        n,
        trial: trial_idx,
        lattice: lattice_type.into(),
        mean_error: mean,
        std_error: std_dev,
        max_error: max,
        median_error: median,
        packing_unique: unique_coords.len(),
        packing_ratio: unique_coords.len() as f64 / n as f64,
        recovery_001,
        recovery_01,
        recovery_05,
        error_p95: p95,
        error_p99: p99,
        elapsed_s: elapsed,
    }
}

/// Aggregate multiple trials.
pub fn aggregate(trials: &[TrialResult]) -> AggregatedResult {
    let n = trials[0].n;
    let lattice = trials[0].lattice.clone();
    let count = trials.len() as f64;

    let mean_of =
        |f: &dyn Fn(&TrialResult) -> f64| -> f64 { trials.iter().map(f).sum::<f64>() / count };

    let std_of = |f: &dyn Fn(&TrialResult) -> f64, mean: f64| -> f64 {
        let var = trials.iter().map(|t| (f(t) - mean).powi(2)).sum::<f64>() / count;
        var.sqrt()
    };

    let mean_error_mean = mean_of(&|t| t.mean_error);
    let packing_ratio_mean = mean_of(&|t| t.packing_ratio);
    let recovery_01_mean = mean_of(&|t| t.recovery_01);
    let recovery_05_mean = mean_of(&|t| t.recovery_05);

    AggregatedResult {
        n,
        lattice,
        mean_error_mean,
        mean_error_std: std_of(&|t| t.mean_error, mean_error_mean),
        median_error_mean: mean_of(&|t| t.median_error),
        max_error_mean: mean_of(&|t| t.max_error),
        packing_ratio_mean,
        packing_ratio_std: std_of(&|t| t.packing_ratio, packing_ratio_mean),
        recovery_01_mean,
        recovery_01_std: std_of(&|t| t.recovery_01, recovery_01_mean),
        recovery_05_mean,
        recovery_05_std: std_of(&|t| t.recovery_05, recovery_05_mean),
        error_p95_mean: mean_of(&|t| t.error_p95),
        error_p99_mean: mean_of(&|t| t.error_p99),
        elapsed_s_mean: mean_of(&|t| t.elapsed_s),
    }
}

/// Full benchmark suite.
pub struct Benchmark {
    pub ns: Vec<usize>,
    pub num_trials: usize,
    pub lattices: Vec<String>,
}

impl Default for Benchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark {
    pub fn new() -> Self {
        Self {
            ns: vec![100, 1000, 10000],
            num_trials: 5,
            lattices: vec!["eisenstein".into(), "z2".into()],
        }
    }

    pub fn with_ns(mut self, ns: Vec<usize>) -> Self {
        self.ns = ns;
        self
    }
    pub fn with_trials(mut self, n: usize) -> Self {
        self.num_trials = n;
        self
    }

    pub fn run(&self) -> (Vec<TrialResult>, Vec<AggregatedResult>) {
        let mut rng = SimpleRng::new(42);
        let mut all_trials = Vec::new();
        let mut aggregated = Vec::new();

        for &n in &self.ns {
            for lattice in &self.lattices {
                let mut trials = Vec::new();
                for t in 0..self.num_trials {
                    let result = run_trial(n, t, lattice, &mut rng);
                    trials.push(result.clone());
                    all_trials.push(result);
                }
                aggregated.push(aggregate(&trials));
            }
        }

        (all_trials, aggregated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rng() {
        let mut rng = SimpleRng::new(42);
        let v = rng.next_f64();
        assert!((0.0..=1.0).contains(&v));
    }

    #[test]
    fn test_run_trial_eisenstein() {
        let mut rng = SimpleRng::new(42);
        let result = run_trial(100, 0, "eisenstein", &mut rng);
        assert_eq!(result.n, 100);
        assert!(result.mean_error > 0.0);
        assert!(result.mean_error < 1.0);
        assert!(result.packing_ratio > 0.0);
    }

    #[test]
    fn test_run_trial_z2() {
        let mut rng = SimpleRng::new(42);
        let result = run_trial(100, 0, "z2", &mut rng);
        assert_eq!(result.n, 100);
        assert!(result.mean_error > 0.0);
    }

    #[test]
    fn test_eisenstein_lower_error() {
        let mut rng = SimpleRng::new(42);
        let e_result = run_trial(1000, 0, "eisenstein", &mut rng);
        let z_result = run_trial(1000, 0, "z2", &mut rng);
        // Eisenstein should have lower mean snap error
        assert!(e_result.mean_error < z_result.mean_error);
    }

    #[test]
    fn test_benchmark_run() {
        let bench = Benchmark::new().with_ns(vec![100]).with_trials(2);
        let (trials, agg) = bench.run();
        assert_eq!(trials.len(), 4); // 1 n × 2 lattices × 2 trials
        assert_eq!(agg.len(), 2); // 1 n × 2 lattices
    }

    #[test]
    fn test_aggregate() {
        let mut rng = SimpleRng::new(42);
        let trials: Vec<TrialResult> = (0..3)
            .map(|i| run_trial(100, i, "eisenstein", &mut rng))
            .collect();
        let agg = aggregate(&trials);
        assert_eq!(agg.n, 100);
        assert!(agg.mean_error_mean > 0.0);
    }
}
