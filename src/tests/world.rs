use super::super::world::*;
use super::assert_approx_equal;
use std::f64::consts::PI;

#[test]
fn test_svector_get_state() {
    let svector = SVector::new(0.0, 2.0, 5.0);
    assert_approx_equal(svector.get_state(0.0), (5, 0));

    let svector = SVector::new(0.927295218, 2.0, 5.0);
    assert_approx_equal(svector.get_state(0.0), (3, 4));

    let svector = SVector::new(PI, 1.0/8.0, 1.414213562);
    assert_approx_equal(svector.get_state(3.0), (1, -1));
}

#[test]
fn test_world_get_state() {
    let world = World::new(vec![
        SVector::new(0.0, 2.0, 5.0),
        SVector::new(0.927295218, 2.0, 5.0)
    ]);
    let points = world.get_state(0.0);
    assert_eq!(points.len(), 3);
    assert_approx_equal(points[0], (0, 0));
    assert_approx_equal(points[1], (5, 0));
    assert_approx_equal(points[2], (8, 4));

    let world = World::new(vec![
        SVector::new(PI, 1.0/8.0, 1.414213562),
        SVector::new(PI, 1.0/4.0, 3.0),
        SVector::new(0.0, 2.0, 2.0)
    ]);
    let points = world.get_state(3.0);
    assert_eq!(points.len(), 4);
    assert_approx_equal(points[0], (0, 0));
    assert_approx_equal(points[1], (1, -1));
    assert_approx_equal(points[2], (1, 2));
    assert_approx_equal(points[3], (3, 2));
}