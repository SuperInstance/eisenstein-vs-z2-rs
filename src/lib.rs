//! eisenstein-vs-z2 — Eisenstein integer lattice vs Z² lattice benchmarking.

pub mod benchmark;
pub mod convergence;
pub mod lattice;

pub use benchmark::{AggregatedResult, Benchmark, TrialResult};
pub use convergence::ConvergenceAnalysis;
pub use lattice::{EisensteinInt, LatticePoint, SnapResult};
