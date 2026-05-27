//! Convergence analysis — how metrics scale with sample size.

use crate::benchmark::{run_trial, SimpleRng, TrialResult};

/// Convergence analysis result for a single lattice type.
#[derive(Debug, Clone)]
pub struct ConvergencePoint {
    pub n: usize,
    pub mean_error: f64,
    pub max_error: f64,
    pub packing_ratio: f64,
    pub recovery_01: f64,
}

/// Full convergence analysis.
pub struct ConvergenceAnalysis {
    pub eisenstein: Vec<ConvergencePoint>,
    pub z2: Vec<ConvergencePoint>,
}

impl ConvergenceAnalysis {
    /// Run convergence analysis across a range of sample sizes.
    pub fn run(ns: &[usize]) -> Self {
        let mut rng = SimpleRng::new(42);
        let mut eisenstein = Vec::new();
        let mut z2 = Vec::new();

        for &n in ns {
            let e = run_trial(n, 0, "eisenstein", &mut rng);
            eisenstein.push(ConvergencePoint {
                n,
                mean_error: e.mean_error,
                max_error: e.max_error,
                packing_ratio: e.packing_ratio,
                recovery_01: e.recovery_01,
            });

            let z = run_trial(n, 0, "z2", &mut rng);
            z2.push(ConvergencePoint {
                n,
                mean_error: z.mean_error,
                max_error: z.max_error,
                packing_ratio: z.packing_ratio,
                recovery_01: z.recovery_01,
            });
        }

        Self { eisenstein, z2 }
    }

    /// Check if Eisenstein consistently outperforms Z².
    pub fn eisenstein_wins_on_error(&self) -> bool {
        self.eisenstein.iter().zip(self.z2.iter())
            .all(|(e, z)| e.mean_error < z.mean_error)
    }

    /// Get advantage percentage at each sample size.
    pub fn advantage_percentages(&self) -> Vec<(usize, f64)> {
        self.eisenstein.iter().zip(self.z2.iter())
            .map(|(e, z)| {
                let adv = (z.mean_error - e.mean_error) / z.mean_error * 100.0;
                (e.n, adv)
            })
            .collect()
    }

    /// Check convergence of mean error (should stabilize).
    pub fn error_converged(&self, tolerance: f64) -> bool {
        if self.eisenstein.len() < 2 { return false; }
        let last = &self.eisenstein[self.eisenstein.len() - 1];
        let prev = &self.eisenstein[self.eisenstein.len() - 2];
        (last.mean_error - prev.mean_error).abs() < tolerance
    }

    /// Summary string.
    pub fn summary(&self) -> String {
        let mut lines = vec!["Convergence Analysis".to_string()];
        lines.push(format!("{:>10} {:>12} {:>12} {:>10}", "N", "Eisenstein", "Z²", "Advantage"));
        for (e, z) in self.eisenstein.iter().zip(self.z2.iter()) {
            let adv = (z.mean_error - e.mean_error) / z.mean_error * 100.0;
            lines.push(format!("{:>10} {:>12.6} {:>12.6} {:>9.2}%", e.n, e.mean_error, z.mean_error, adv));
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_analysis() {
        let ca = ConvergenceAnalysis::run(&[100, 500, 1000]);
        assert_eq!(ca.eisenstein.len(), 3);
        assert_eq!(ca.z2.len(), 3);
    }

    #[test]
    fn test_eisenstein_wins() {
        let ca = ConvergenceAnalysis::run(&[100, 500, 1000, 5000]);
        assert!(ca.eisenstein_wins_on_error());
    }

    #[test]
    fn test_advantage_percentages() {
        let ca = ConvergenceAnalysis::run(&[100, 1000]);
        let advs = ca.advantage_percentages();
        assert_eq!(advs.len(), 2);
        // All advantages should be positive (Eisenstein better)
        for (_, adv) in &advs {
            assert!(*adv > 0.0);
        }
    }

    #[test]
    fn test_summary() {
        let ca = ConvergenceAnalysis::run(&[100, 1000]);
        let summary = ca.summary();
        assert!(summary.contains("Convergence Analysis"));
        assert!(summary.contains("Eisenstein"));
    }
}
