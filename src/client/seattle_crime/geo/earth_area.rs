use geo_types::{LineString, Point, Polygon};

use std::f64;

const EARTH_RADIUS_KM: f64 = 6_371.008;

pub trait EarthArea {
    fn area(&self) -> f64;
}

impl EarthArea for Polygon<f64> {
    fn area(&self) -> f64 {
        let exterior = self.exterior.area();
        let holes: f64 = self.interiors.iter().map(EarthArea::area).sum();
        exterior - holes
    }
}

impl EarthArea for LineString<f64> {
    fn area(&self) -> f64 {
        if self.0.len() < 3 {
            return 0.0;
        }

        let radian_points: Vec<_> = self
            .points_iter()
            .chain(self.points_iter().take(2))
            .map(|p| Point::new(p.x().to_radians(), p.y().to_radians()))
            .collect();
        let degree_area: f64 = radian_points
            .windows(3)
            .map(|w| {
                let (p1, p2, p3) = match w {
                    [p1, p2, p3] => (p1, p2, p3),
                    _ => unreachable!(),
                };
                (p3.x() - p1.x()) * f64::sin(p2.y())
            })
            .sum();
        return (EARTH_RADIUS_KM * EARTH_RADIUS_KM * degree_area / 2.).abs();
    }
}

#[cfg(all(test, not(feature = "integration")))]
mod test {
    use super::*;
    use approx::{__assert_approx, assert_relative_eq, relative_eq};
    use geo_types::Coordinate;

    #[test]
    fn line_string_area() {
        let line_string = LineString(vec![
            Coordinate { x: -180., y: -90. },
            Coordinate { x: -180., y: 90. },
            Coordinate { x: 180., y: 90. },
            Coordinate { x: 180., y: -90. },
            Coordinate { x: -180., y: -90. },
        ]);
        assert_relative_eq!(line_string.area(), 510_072_000., max_relative = 0.003);
    }
}
