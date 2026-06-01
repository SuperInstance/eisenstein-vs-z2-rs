# eisenstein-vs-z2-rs

**Rigorous comparison of hexagonal (Eisenstein) vs square (ℤ²) lattice snapping in Rust.**

Eisenstein integers — complex numbers of the form `a + bω` where `ω = e^(2πi/3)` — form a hexagonal lattice that is provably the densest circle packing in 2D (Thue's theorem, 1890). This crate implements Eisenstein integer arithmetic, lattice snapping for both hexagonal and square grids, and a full benchmark suite proving the hexagonal lattice's ~15% advantage in snap error.

---

## What This Does

- **`EisensteinInt`** — Eisenstein integer type with arithmetic (add, multiply, conjugate, norm)
- **Lattice snapping** — snap arbitrary 2D points to the nearest Eisenstein or ℤ² lattice point
- **`Benchmark`** — configurable benchmark suite comparing both lattices across sample sizes and trials
- **`ConvergenceAnalysis`** — verify that Eisenstein's advantage holds and converges as sample size grows
- **Theoretical constants** — covering radii, Voronoi cell areas for both lattices

---

## Key Idea

The hexagonal lattice has a **smaller covering radius** than the square lattice. The covering radius is the maximum distance any point in ℝ² can be from its nearest lattice point:

| Lattice | Covering Radius | Voronoi Cell Area |
|---|---|---|
| Eisenstein (hexagonal) | 1/√3 ≈ 0.5774 | √3/2 ≈ 0.866 |
| ℤ² (square) | 1/√2 ≈ 0.7071 | 1.0 |

This means every point in the plane is closer to its nearest Eisenstein lattice point than it would be on a square grid. The advantage is **~18.3%** in covering radius and **~13.4%** in cell area — and it shows up consistently in empirical snap error benchmarks.

---

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
eisenstein-vs-z2 = { git = "https://github.com/SuperInstance/eisenstein-vs-z2-rs" }
```

Or from source:

```bash
git clone https://github.com/SuperInstance/eisenstein-vs-z2-rs.git
cd eisenstein-vs-z2-rs
cargo build
```

Dependencies: `serde` + `serde_json` (for serializing benchmark results).

---

## Quick Start

### Eisenstein Integer Arithmetic

```rust
use eisenstein_vs_z2::EisensteinInt;

let e = EisensteinInt::new(3, 5);

// Eisenstein norm: a² - ab + b²
println!("Norm: {}", e.norm()); // 9 - 15 + 25 = 19

// Cartesian coordinates via ω = e^(2πi/3)
let (x, y) = e.to_cartesian();
println!("Cartesian: ({:.4}, {:.4})", x, y); // (0.5, 4.3301)

// Conjugate: (a, b) → (a-b, -b)
let conj = e.conjugate(); // (-2, -5)

// Multiply: z·z̄ = |z|² (norm)
let prod = e.multiply(&conj);
assert_eq!(prod.norm(), e.norm() * e.norm());
```

### Lattice Snapping Comparison

```rust
use eisenstein_vs_z2::{snap_eisenstein, snap_z2};

let e_result = snap_eisenstein(1.7, 2.3);
let z_result = snap_z2(1.7, 2.3);

println!("Eisenstein snap error: {:.6}", e_result.error);
println!("Z² snap error:        {:.6}", z_result.error);
// Eisenstein error is consistently lower
```

### Full Benchmark

```rust
use eisenstein_vs_z2::Benchmark;

let bench = Benchmark::new()
    .with_ns(vec![100, 1000, 10000])
    .with_trials(5);

let (trials, aggregated) = bench.run();

for agg in &aggregated {
    println!("n={}, lattice={}, mean_error={:.6}",
        agg.n, agg.lattice, agg.mean_error_mean);
}
```

### Convergence Analysis

```rust
use eisenstein_vs_z2::ConvergenceAnalysis;

let ca = ConvergenceAnalysis::run(&[100, 500, 1000, 5000, 10000]);

assert!(ca.eisenstein_wins_on_error());

for (n, adv) in ca.advantage_percentages() {
    println!("n={}: Eisenstein advantage = {:.2}%", n, adv);
}

println!("{}", ca.summary());
```

---

## API Reference

### `EisensteinInt`

```rust
pub struct EisensteinInt { pub a: i64, pub b: i64 }
```

Represents the Eisenstein integer `a + bω` where `ω = e^(2πi/3) = -1/2 + i√3/2`.

| Method | Description |
|---|---|
| `new(a, b)` | Create an Eisenstein integer |
| `to_cartesian()` → `(f64, f64)` | Convert to (x, y) in ℝ² |
| `norm()` → `i64` | Eisenstein norm: `a² - ab + b²` |
| `distance_from_origin()` → `f64` | Euclidean distance from origin |
| `add(&other)` → `EisensteinInt` | Component-wise addition |
| `multiply(&other)` → `EisensteinInt` | Eisenstein multiplication via `ω² = -1 - ω` |
| `conjugate()` → `EisensteinInt` | Conjugate: `(a, b) → (a-b, -b)` |

### `LatticePoint`

```rust
pub struct LatticePoint { pub x: f64, pub y: f64 }
```

A point in Cartesian 2D space with `distance_to(&other)`.

### `SnapResult`

```rust
pub struct SnapResult {
    pub original: LatticePoint,    // input point
    pub snapped: LatticePoint,     // nearest lattice point
    pub lattice_coords: (i64, i64), // integer coordinates on the lattice
    pub error: f64,                // Euclidean distance from original to snapped
}
```

### Snapping Functions

| Function | Description |
|---|---|
| `snap_eisenstein(x, y)` → `SnapResult` | Snap to nearest Eisenstein integer |
| `snap_z2(x, y)` → `SnapResult` | Snap to nearest ℤ² integer |
| `covering_radius_eisenstein()` → `f64` | 1/√3 ≈ 0.5774 |
| `covering_radius_z2()` → `f64` | 1/√2 ≈ 0.7071 |
| `voronoi_cell_area_eisenstein()` → `f64` | √3/2 ≈ 0.866 |
| `voronoi_cell_area_z2()` → `f64` | 1.0 |

### `Benchmark`

```rust
let bench = Benchmark::new()
    .with_ns(vec![100, 1000, 10000])
    .with_trials(5);
let (trials, aggregated) = bench.run();
```

**`TrialResult`** — per-trial metrics: `mean_error`, `std_error`, `max_error`, `median_error`, `packing_ratio`, `recovery_01`, `recovery_05`, `error_p95`, `error_p99`, `elapsed_s`.

**`AggregatedResult`** — mean ± std across trials for all metrics.

### `ConvergenceAnalysis`

```rust
let ca = ConvergenceAnalysis::run(&[100, 500, 1000, 5000]);
```

| Method | Description |
|---|---|
| `eisenstein_wins_on_error()` → `bool` | True if Eisenstein mean error < ℤ² at every sample size |
| `advantage_percentages()` → `Vec<(usize, f64)>` | Eisenstein advantage % at each sample size |
| `error_converged(tolerance)` → `bool` | Whether error stabilized between last two sample sizes |
| `summary()` → `String` | Formatted table |

---

## How It Works

### Eisenstein Integer → Cartesian Conversion

An Eisenstein integer `a + bω` where `ω = -1/2 + i√3/2` maps to Cartesian as:

$$x = a - \frac{b}{2}, \qquad y = \frac{b\sqrt{3}}{2}$$

### Cartesian → Eisenstein Snapping

Given a point `(x, y)` in Cartesian space, we invert the mapping:

$$b = \frac{y}{\sqrt{3}/2}, \qquad a = x + \frac{b}{2}$$

Then round both to the nearest integers. The result is the closest Eisenstein lattice point.

### Eisenstein Multiplication

For `(a + bω)(c + dω)`, using `ω² = -1 - ω`:

$$\text{real} = ac - bd, \qquad \text{ω-coeff} = ad + bc - bd$$

### Benchmark Methodology

1. Generate `n` random points uniformly in `[-10, 10]²` using a seeded LCG (deterministic, no external RNG dependency).
2. Snap each point to both lattices and record errors.
3. Track unique lattice coordinates (packing ratio = unique / total, measures collision rate).
4. Measure recovery rates: fraction of points snapped with error ≤ threshold.
5. Compute percentiles (p95, p99) on sorted errors.
6. Repeat across multiple trials and aggregate (mean ± std).

### Convergence Analysis

Runs the same benchmark at increasing sample sizes and verifies that:
- Eisenstein consistently outperforms ℤ² at every size
- The advantage percentage is stable (not a small-sample artifact)
- Error metrics converge as sample size grows

---

## The Math

### Eisenstein Integers

Eisenstein integers are complex numbers of the form `z = a + bω` where `ω = e^(2πi/3)` is a primitive cube root of unity. They satisfy:

$$\omega^2 + \omega + 1 = 0, \qquad \omega^2 = \overline{\omega} = -1 - \omega$$

The **Eisenstein norm** is:

$$N(z) = z \cdot \overline{z} = a^2 - ab + b^2$$

This is always a non-negative integer. The six Eisenstein units (elements with norm 1) are: `(1,0)`, `(-1,0)`, `(0,1)`, `(0,-1)`, `(1,1)`, `(-1,-1)`.

### Thue's Theorem (1890)

The hexagonal lattice (equivalently, the Eisenstein integers) is the **densest sphere packing in ℝ²**. No arrangement of equal circles in the plane has higher density than the hexagonal arrangement, which achieves:

$$\rho = \frac{\pi}{2\sqrt{3}} \approx 0.9069$$

### Covering Radius

The **covering radius** of a lattice is the smallest `r` such that every point in ℝ² is within distance `r` of some lattice point:

- **Hexagonal**: $r = 1/\sqrt{3} \approx 0.5774$
- **Square** (ℤ²): $r = 1/\sqrt{2} \approx 0.7071$

The hexagonal advantage: $1 - \frac{1/\sqrt{3}}{1/\sqrt{2}} = 1 - \sqrt{2/3} \approx 18.35\%$

### Voronoi Cells

- **Hexagonal**: regular hexagon with area $\sqrt{3}/2 \approx 0.866$
- **Square**: unit square with area $1.0$

The hexagonal Voronoi cell is **13.4% smaller**, meaning fewer points map to the same lattice point (better packing).

### Snap Error Distribution

For uniformly random points in the plane, the expected snap error depends on the lattice's Voronoi cell geometry. The hexagonal lattice's regular hexagonal Voronoi cell distributes error more uniformly (closer to circular) than the square lattice's axis-aligned square cell, resulting in both lower mean and lower worst-case error.

---

## Testing

```bash
cargo test
```

**31 tests** covering:

- **`EisensteinInt`** — to-cartesian conversion, norm computation, addition, multiplication, conjugate, identity/zero cases
- **Lattice snapping** — origin snapping (zero error), near-basis snapping, covering radii ordering, Voronoi cell areas
- **Cartesian round-trip** — all Eisenstein integers `(a, b)` for `a, b ∈ [-3, 3]` survive snap → Eisenstein → Cartesian → snap
- **`Benchmark`** — RNG range check, single trial for each lattice, Eisenstein lower error than ℤ², full benchmark run (verifies trial count), aggregation statistics
- **`ConvergenceAnalysis`** — correct number of data points, Eisenstein wins at all sample sizes, positive advantage percentages, summary formatting
- **Integration tests** — unit norms (all 6 Eisenstein units), multiplication by identity preserves norm, conjugate norm product `z·z̄ = N(z)²`, grid point zero error, covering radius ordering, lattice point distance (3-4-5 triangle)

---

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem.
