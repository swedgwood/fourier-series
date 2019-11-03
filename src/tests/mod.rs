mod common;
mod world;

use super::common::Point;

pub fn get_error_in_point<T1: Into<Point>, T2: Into<Point>>(approx: T1, exact: T2) -> f64 {
    let difference = exact.into() - approx.into();
    difference.mag()
}

pub fn assert_approx_equal<T1: Into<Point>, T2: Into<Point>>(approx: T1, exact: T2) {
    let exact = exact.into();
    let approx = approx.into();
    let error = get_error_in_point(exact, approx);
    assert!(error < 0.0001,
        "assertion failed, error > 0.0001: `(left ~= right)`\n  left: `{:?}`,\n right: `{:?}`\n error: `{:?}`",
        exact, approx, error);
}