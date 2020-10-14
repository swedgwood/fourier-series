use super::common::Point;

pub fn point_average<T>(points: T) -> Point where T: Iterator<Item=Point> {
    let mut total = Point::ZERO;
    let mut count = 0;

    for point in points {
        total += point;
        count += 1;
    }

    total.scale(1.0/count as f64)
}