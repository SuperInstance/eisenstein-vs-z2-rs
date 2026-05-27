# eisenstein-vs-z2-rs

<<<<<<< HEAD
Rust port of [eisenstein-vs-z2](https://github.com/SuperInstance/eisenstein-vs-z2) — rigorous comparison of hexagonal (Eisenstein) vs square lattice snapping.

## Features

- **`EisensteinInt`** — Eisenstein integer arithmetic (add, multiply, conjugate, norm)
- **Lattice snapping** — Snap 2D points to nearest Eisenstein or Z² lattice point
- **`Benchmark`** — Full benchmark suite with configurable sample sizes and trials
- **`ConvergenceAnalysis`** — Verify Eisenstein advantage scales across sample sizes
- **Theoretical constants** — Covering radii, Voronoi cell areas

## Why Eisenstein?

The hexagonal lattice (Eisenstein integers) is provably the densest packing in 2D (Thue's theorem). This means:
- **Lower snap error**: Every point in ℝ² is closer to its nearest Eisenstein lattice point
- **Better packing**: Fewer collisions when snapping to integer coordinates
- **~13-17% advantage** across all metrics
=======
Rust port of [eisenstein-vs-z2](https://github.com/SuperInstance/eisenstein-vs-z2) — benchmarking hexagonal vs square lattice snapping.

## Why Eisenstein > Z²?

Eisenstein integers ℤ[ω] (hexagonal lattice) are the densest packing in 2D. This means:
- **Smaller covering radius**: 1/√3 ≈ 0.577 vs 1/√2 ≈ 0.707
- **Better snap accuracy**: mean error ~13% lower
- **Better packing density**: more unique lattice points per random sample
>>>>>>> 28dd04f (Initial Rust port: Eisenstein vs Z² lattice benchmark)

## Usage

```rust
<<<<<<< HEAD
use eisenstein_vs_z2::{EisensteinInt, snap_eisenstein, snap_z2, Benchmark, ConvergenceAnalysis};

// Eisenstein integer arithmetic
let e = EisensteinInt::new(3, 5);
println!("Norm: {}", e.norm()); // a² - ab + b²
println!("Cartesian: {:?}", e.to_cartesian());

// Compare snap errors
let e_result = snap_eisenstein(1.7, 2.3);
let z_result = snap_z2(1.7, 2.3);
println!("Eisenstein error: {:.4}", e_result.error);
println!("Z² error: {:.4}", z_result.error);

// Full benchmark
let bench = Benchmark::new();
let (trials, aggregated) = bench.run();

// Convergence analysis
let ca = ConvergenceAnalysis::run(&[100, 1000, 10000]);
assert!(ca.eisenstein_wins_on_error());
println!("{}", ca.summary());
=======
use eisenstein_vs_z2::{snap_eisenstein, snap_z2, run_eisenstein_trial, run_z2_trial};

// Snap a point to nearest lattice point
let e = snap_eisenstein(1.4, 2.6);  // EisensteinInt { a, b }
let z = snap_z2(1.4, 2.6);          // Z2Int { x, y }

// Benchmark 10k random points
let points: Vec<(f64, f64)> = /* ... */;
let e_result = run_eisenstein_trial(&points);
let z_result = run_z2_trial(&points);
assert!(e_result.mean_error < z_result.mean_error);
>>>>>>> 28dd04f (Initial Rust port: Eisenstein vs Z² lattice benchmark)
```

## License

MIT
