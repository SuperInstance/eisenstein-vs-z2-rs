# eisenstein-vs-z2-rs

Rust benchmark comparing Eisenstein (hexagonal A₂) vs square (Z²) lattice for constraint quantization — snap error, packing density, convergence rate, and spectral properties.

## What This Gives You

- **Dual-lattice snap** — snap points to both Eisenstein A₂ and Z² with error measurement
- **Statistical benchmarks** — RMS error, max error, and error distribution across thousands of points
- **Convergence analysis** — how quickly each lattice converges under constraint dynamics
- **Packing comparison** — lattice packing density and covering radius
- **20 tests** — verified snap accuracy and metric correctness

## Quick Start

```rust
use eisenstein_vs_z2::{LatticePoint, Benchmark, EisensteinInt};

// Snap to Eisenstein lattice
let e = EisensteinInt::snap(0.7, 0.3);
println!("Eisenstein: ({}, {}), norm={}", e.a, e.b, e.norm());

// Snap to Z²
let z = LatticePoint::snap_z2(0.7, 0.3);
println!("Z²: ({}, {})", z.x, z.y);

// Run comparison benchmark
let bench = Benchmark::new(10000);
let result = bench.run();
println!("Eisenstein RMS error: {:.4}", result.eisenstein_rms);
println!("Z² RMS error: {:.4}", result.z2_rms);
println!("Eisenstein wins: {:.1}%", result.eisenstein_wins_pct());
```

## API Reference

| Type | Description |
|---|---|
| `EisensteinInt` | Eisenstein integer with snap and norm |
| `LatticePoint` | Z² lattice point with snap |
| `Benchmark` | Statistical comparison runner |
| `TrialResult` | Single comparison result |
| `AggregatedResult` | Summary statistics |
| `ConvergenceAnalysis` | Convergence rate comparison |

## Key Results

The Eisenstein A₂ lattice outperforms Z² for constraint quantization because:
- **Closer packing** — hexagonal lattice has higher packing density (π/2√3 vs π/4)
- **Smaller covering radius** — ρ = 1/√3 ≈ 0.577 vs √2/2 ≈ 0.707
- **6-fold symmetry** — more rotational symmetry preserves more structure

## How It Fits

The **benchmark suite** for lattice comparison:

- [eisenstein-vs-z2-c](https://github.com/SuperInstance/eisenstein-vs-z2-c) — C port
- [eisenstein-triples](https://github.com/SuperInstance/eisenstein-triples) — Eisenstein triple number theory
- [eisenstein-embed](https://github.com/SuperInstance/eisenstein-embed) — Eisenstein embeddings
- [constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core) — uses A₂ lattice

## Testing

```bash
cargo test  # 20 tests
```

## Installation

```bash
cargo add eisenstein-vs-z2
```

## License

MIT
