//! eisenstein-vs-z2 — Eisenstein integer lattice vs Z² lattice benchmarking.

pub mod lattice;
pub mod benchmark;
pub mod convergence;

pub use lattice::{EisensteinInt, LatticePoint, SnapResult};
pub use benchmark::{Benchmark, TrialResult, AggregatedResult};
pub use convergence::ConvergenceAnalysis;
