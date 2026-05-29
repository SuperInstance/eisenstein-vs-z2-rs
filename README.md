# eisenstein-vs-z2-rs

Rust port of [eisenstein-vs-z2](https://github.com/SuperInstance/eisenstein-vs-z2) — rigorous comparison of hexagonal (Eisenstein) vs square lattice snapping.

## What This Gives You

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

## Quick Start

```rust
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
```

## How It Fits

Part of the [SuperInstance](https://github.com/SuperInstance) Eisenstein lattice ecosystem:

- **Original**: [eisenstein-vs-z2](https://github.com/SuperInstance/eisenstein-vs-z2) — Python reference
- **This repo**: Rust port for production use
- **C port**: [eisenstein-vs-z2-c](https://github.com/SuperInstance/eisenstein-vs-z2-c) — Embedded/bare-metal
- **Embeddings**: [eisenstein-embed](https://github.com/SuperInstance/eisenstein-embed) — Higher-dimensional embeddings
- **Triples**: [eisenstein-triples](https://github.com/SuperInstance/eisenstein-triples) — Eisenstein integer triples

## Testing

```bash
cargo test
cargo bench
```

## Installation

```toml
[dependencies]
eisenstein-vs-z2 = { git = "https://github.com/SuperInstance/eisenstein-vs-z2-rs" }
```

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem.
