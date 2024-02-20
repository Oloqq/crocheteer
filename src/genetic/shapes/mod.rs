use crate::common::Point;

#[derive(Clone)]
pub struct Slice {
    points: Vec<Point>,
}

impl Slice {
    pub fn from(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn compare(&self, other: &Self) -> f32 {
        assert!(self.points.len() >= other.points.len()); // one of the shapes will be the dense, interpolated one, for efficiency it's assumed it's `self`

        let mut result = 0.0;
        for point in &other.points {
            let my_closest = self.find_closest(point);
            println!("closest to {point:?} is {my_closest:?}");
            result += (my_closest - point).magnitude();
        }

        result
    }

    fn find_closest(&self, p: &Point) -> &Point {
        let mut closest = self.points.first().unwrap();
        let mut best_dist = f32::INFINITY;
        for point in &self.points {
            let dist = (p - point).magnitude();
            if dist < best_dist {
                closest = point;
                best_dist = dist;
            }
        }

        closest
    }
}

#[derive(Clone)]
pub struct Shape {
    slices: Vec<Slice>,
}

impl Shape {
    fn compare(&self, other: &Self) -> f32 {
        assert!(self.slices.len() == other.slices.len());

        self.slices
            .iter()
            .zip(&other.slices)
            .fold(0.0, |acc, (my_slice, their_slice)| {
                let x = my_slice.compare(their_slice);
                println!("ada {x}");
                acc + x
            })
    }
}

pub fn compare_shapes(original: &Shape, other: &Shape) -> f32 {
    original.compare(other)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diamond(x: f32) -> Shape {
        Shape {
            slices: vec![
                Slice::from(vec![Point::origin()]),
                Slice::from(vec![
                    Point::new(-x, 1.0, -x),
                    Point::new(-x, 1.0, x),
                    Point::new(x, 1.0, x),
                    Point::new(x, 1.0, -x),
                ]),
                Slice::from(vec![Point::new(0.0, 2.0, 0.0)]),
            ],
        }
    }

    #[test]
    fn test_same_shape_error_is_0() {
        let s1 = diamond(1.0);
        let s2 = s1.clone();
        assert_eq!(compare_shapes(&s1, &s2), 0.0);
    }

    #[test]
    fn test_differing_shape_error_not_0() {
        let s1 = diamond(1.0);
        let s2 = diamond(0.9);
        let diff1 = compare_shapes(&s1, &s2);
        assert!(diff1 > 0.0);

        let s2 = diamond(1.1);
        let diff2 = compare_shapes(&s1, &s2);
        assert!(diff2 > 0.0);

        assert_eq!(diff1, diff2);
    }

    #[test]
    fn test_error_grows_correctly() {
        let s1 = diamond(1.0);
        let s2 = diamond(0.9);
        let s3 = diamond(2.0);
        let diff1 = compare_shapes(&s1, &s2);
        let diff2 = compare_shapes(&s1, &s3);
        assert!(diff1 < diff2);
    }
}
