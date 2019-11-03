use super::super::common::*;

#[test]
fn test_point_from() {
    let point = Point::from((1, -2));
    assert_eq!(point, Point::new(1.0, -2.0));

    let point = Point::from((-1.5, 200));
    assert_eq!(point, Point::new(-1.5, 200.0));
}

#[test]
fn test_point_add() {
    let mut point1 = Point::new( 3.0,  7.0);
    let point2     = Point::new( 8.0,  9.0);
    let answer     = Point::new(11.0, 16.0);
    assert_eq!(point1 + point2, answer);

    point1 += point2;
    assert_eq!(point1, answer);
}

#[test]
fn test_point_sub() {
    let mut point1 = Point::new(11.0, 16.0);
    let point2     = Point::new( 8.0,  9.0);
    let answer     = Point::new( 3.0,  7.0);
    assert_eq!(point1 - point2, answer);

    point1 -= point2;
    assert_eq!(point1, answer);
}

#[test]
fn test_point_neg() {
    let point = Point::new(4.0, -5.0);
    assert_eq!(-point, (-4, 5));
}