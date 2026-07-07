use eisenstein_vs_z2::*;

#[test]
fn test_eisenstein_zero() {
    let z = EisensteinInt::new(0, 0);
    assert_eq!(z.norm(), 0);
}

#[test]
fn test_eisenstein_unit_norms() {
    let units = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1)];
    for (a, b) in units {
        assert_eq!(EisensteinInt::new(a, b).norm(), 1);
    }
}

#[test]
fn test_eisenstein_multiply_unit() {
    let z = EisensteinInt::new(3, 5);
    let one = EisensteinInt::new(1, 0);
    let result = z.multiply(&one);
    assert_eq!(result.norm(), z.norm());
}

#[test]
fn test_eisenstein_conjugate_norm() {
    let z = EisensteinInt::new(3, 5);
    let conj = z.conjugate();
    let prod = z.multiply(&conj);
    assert_eq!(prod.norm(), z.norm() * z.norm());
}

#[test]
fn test_eisenstein_add_identity() {
    let z = EisensteinInt::new(3, 5);
    let zero = EisensteinInt::new(0, 0);
    let result = z.add(&zero);
    assert_eq!(result.norm(), z.norm());
}

#[test]
fn test_snap_z2_grid_points() {
    let result = lattice::snap_z2(3.0, 4.0);
    assert!(result.error < 1e-6);
}

#[test]
fn test_eisenstein_better_than_z2() {
    let e_radius = lattice::covering_radius_eisenstein();
    let z_radius = lattice::covering_radius_z2();
    assert!(e_radius < z_radius);
}

#[test]
fn test_lattice_point_distance() {
    let a = LatticePoint::new(0.0, 0.0);
    let b = LatticePoint::new(3.0, 4.0);
    assert!((a.distance_to(&b) - 5.0).abs() < 1e-10);
}

#[test]
fn test_voronoi_cell_areas() {
    let e_area = lattice::voronoi_cell_area_eisenstein();
    let z_area = lattice::voronoi_cell_area_z2();
    assert!(e_area > 0.0);
    assert!(z_area > 0.0);
}

#[test]
fn test_eisenstein_to_cartesian_roundtrip() {
    for a in -3i64..=3 {
        for b in -3i64..=3 {
            let z = EisensteinInt::new(a, b);
            let (x, y) = z.to_cartesian();
            let result = lattice::snap_eisenstein(x, y);
            assert!(
                result.error < 1e-6,
                "roundtrip failed for ({a},{b}): error={}",
                result.error
            );
        }
    }
}

#[test]
fn test_eisenstein_distance_from_origin() {
    let origin = EisensteinInt::new(0, 0);
    assert!((origin.distance_from_origin()) < 1e-10);
}
