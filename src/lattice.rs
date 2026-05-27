//! Eisenstein integer lattice and Z² lattice operations.

use std::f64::consts::SQRT_2;

const SQRT_3: f64 = 1.7320508075688772;
const SQRT3_2: f64 = SQRT_3 / 2.0;

/// An Eisenstein integer (a + bω where ω = e^(2πi/3)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EisensteinInt {
    pub a: i64,
    pub b: i64,
}

impl EisensteinInt {
    pub fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    /// Convert to Cartesian coordinates.
    pub fn to_cartesian(&self) -> (f64, f64) {
        let x = self.a as f64 - self.b as f64 / 2.0;
        let y = self.b as f64 * SQRT3_2;
        (x, y)
    }

    /// Eisenstein norm: a² - ab + b².
    pub fn norm(&self) -> i64 {
        self.a * self.a - self.a * self.b + self.b * self.b
    }

    /// Distance from origin in Cartesian space.
    pub fn distance_from_origin(&self) -> f64 {
        let (x, y) = self.to_cartesian();
        (x * x + y * y).sqrt()
    }

    /// Add two Eisenstein integers.
    pub fn add(&self, other: &EisensteinInt) -> EisensteinInt {
        EisensteinInt::new(self.a + other.a, self.b + other.b)
    }

    /// Multiply two Eisenstein integers.
    pub fn multiply(&self, other: &EisensteinInt) -> EisensteinInt {
        // (a + bω)(c + dω) = ac + (ad + bc)ω + bdω²
        // ω² = -1 - ω
        // = ac - bd + (ad + bc - bd)ω
        EisensteinInt::new(
            self.a * other.a - self.b * other.b,
            self.a * other.b + self.b * other.a - self.b * other.b,
        )
    }

    /// Conjugate: (a, b) → (a - b, -b).
    pub fn conjugate(&self) -> EisensteinInt {
        EisensteinInt::new(self.a - self.b, -self.b)
    }
}

impl std::fmt::Display for EisensteinInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

/// A point in Cartesian 2D space.
#[derive(Debug, Clone, Copy)]
pub struct LatticePoint {
    pub x: f64,
    pub y: f64,
}

impl LatticePoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &LatticePoint) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Result of snapping a point to a lattice.
#[derive(Debug, Clone)]
pub struct SnapResult {
    pub original: LatticePoint,
    pub snapped: LatticePoint,
    pub lattice_coords: (i64, i64),
    pub error: f64,
}

/// Snap a point to the nearest Eisenstein integer.
pub fn snap_eisenstein(x: f64, y: f64) -> SnapResult {
    // Inverse: x = a - b/2, y = b*√3/2
    let b_cont = y / SQRT3_2;
    let a_cont = x + b_cont / 2.0;

    let a = a_cont.round() as i64;
    let b = b_cont.round() as i64;

    let (sx, sy) = EisensteinInt::new(a, b).to_cartesian();
    let snapped = LatticePoint::new(sx, sy);

    SnapResult {
        original: LatticePoint::new(x, y),
        snapped,
        lattice_coords: (a, b),
        error: LatticePoint::new(x, y).distance_to(&snapped),
    }
}

/// Snap a point to the nearest Z² integer.
pub fn snap_z2(x: f64, y: f64) -> SnapResult {
    let a = x.round() as i64;
    let b = y.round() as i64;
    let snapped = LatticePoint::new(a as f64, b as f64);
    SnapResult {
        original: LatticePoint::new(x, y),
        snapped,
        lattice_coords: (a, b),
        error: LatticePoint::new(x, y).distance_to(&snapped),
    }
}

/// Maximum theoretical snap error (covering radius).
pub fn covering_radius_eisenstein() -> f64 {
    1.0 / SQRT_3 // ≈ 0.5774
}

pub fn covering_radius_z2() -> f64 {
    1.0 / SQRT_2 // ≈ 0.7071
}

/// Voronoi cell area.
pub fn voronoi_cell_area_eisenstein() -> f64 {
    SQRT3_2 // ≈ 0.8660
}

pub fn voronoi_cell_area_z2() -> f64 {
    1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eisenstein_to_cartesian() {
        let e = EisensteinInt::new(1, 0);
        let (x, y) = e.to_cartesian();
        assert!((x - 1.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_eisenstein_norm() {
        let e = EisensteinInt::new(1, 0);
        assert_eq!(e.norm(), 1);
        let e2 = EisensteinInt::new(1, 1);
        assert_eq!(e2.norm(), 1); // 1 - 1 + 1 = 1
    }

    #[test]
    fn test_eisenstein_add() {
        let a = EisensteinInt::new(1, 2);
        let b = EisensteinInt::new(3, 4);
        let c = a.add(&b);
        assert_eq!(c.a, 4);
        assert_eq!(c.b, 6);
    }

    #[test]
    fn test_eisenstein_multiply() {
        let a = EisensteinInt::new(1, 0);
        let b = EisensteinInt::new(0, 1);
        let c = a.multiply(&b);
        assert_eq!(c.a, 0);
        assert_eq!(c.b, 1);
    }

    #[test]
    fn test_eisenstein_conjugate() {
        let e = EisensteinInt::new(3, 5);
        let conj = e.conjugate();
        assert_eq!(conj.a, -2);
        assert_eq!(conj.b, -5);
    }

    #[test]
    fn test_snap_eisenstein_origin() {
        let result = snap_eisenstein(0.0, 0.0);
        assert!(result.error.abs() < 1e-10);
        assert_eq!(result.lattice_coords, (0, 0));
    }

    #[test]
    fn test_snap_z2_origin() {
        let result = snap_z2(0.0, 0.0);
        assert!(result.error.abs() < 1e-10);
    }

    #[test]
    fn test_snap_eisenstein_near_basis() {
        // Point near (1, 0) in Eisenstein coords
        let result = snap_eisenstein(0.9, 0.1);
        assert!(result.error < 0.5);
    }

    #[test]
    fn test_covering_radii() {
        let e = covering_radius_eisenstein();
        let z = covering_radius_z2();
        assert!(e < z); // Eisenstein has smaller covering radius
    }

    #[test]
    fn test_voronoi_cell_areas() {
        let e = voronoi_cell_area_eisenstein();
        let z = voronoi_cell_area_z2();
        assert!(e < z); // Eisenstein has smaller cell
    }
}
