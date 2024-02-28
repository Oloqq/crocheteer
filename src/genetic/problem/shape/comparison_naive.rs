use super::{Point2, Shape, Slice};

impl Slice {
    fn compare(&self, other: &Self) -> f32 {
        // assert!(
        //     self.points.len() >= other.points.len(),
        //     "{} >= {}",
        //     self.points.len(),
        //     other.points.len()
        // ); // assumption: self is the denser one

        let mut result = 0.0;
        for point in &other.points {
            let my_closest = self.find_closest(point);
            result += (my_closest - point).magnitude();
        }

        result
    }

    fn find_closest(&self, p: &Point2) -> &Point2 {
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

// TODO implement comparison using k-d trees, this is stupid
impl Shape {
    pub fn compare(&self, other: &Self) -> f32 {
        assert!(
            self.slices.len() == other.slices.len(),
            "{} == {}",
            self.slices.len(),
            other.slices.len()
        );

        let result = self
            .slices
            .iter()
            .zip(&other.slices)
            .fold(0.0, |acc, (my_slice, their_slice)| {
                acc + my_slice.compare(their_slice)
            });

        assert!(
            !result.is_nan(),
            "Got NaN in comparison. \nCompared {:?}\nand\n{:?}",
            self,
            other
        );

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Point;

    fn diamond(x: f32) -> Shape {
        Shape {
            slices: vec![
                Slice::from_3d(vec![Point::origin()]),
                Slice::from_3d(vec![
                    Point::new(-x, 1.0, -x),
                    Point::new(-x, 1.0, x),
                    Point::new(x, 1.0, x),
                    Point::new(x, 1.0, -x),
                ]),
                Slice::from_3d(vec![Point::new(0.0, 2.0, 0.0)]),
            ],
        }
    }

    #[test]
    fn test_same_shape_error_is_0() {
        let s1 = diamond(1.0);
        let s2 = s1.clone();
        assert_eq!(s1.compare(&s2), 0.0);
    }

    #[test]
    fn test_differing_shape_error_not_0() {
        let s1 = diamond(1.0);
        let s2 = diamond(0.9);
        let diff1 = s1.compare(&s2);
        assert!(diff1 > 0.0);

        let s2 = diamond(1.1);
        let diff2 = s1.compare(&s2);
        assert!(diff2 > 0.0);

        assert_eq!(diff1, diff2);
    }

    #[test]
    fn test_error_grows_correctly() {
        let s1 = diamond(1.0);
        let s2 = diamond(0.9);
        let s3 = diamond(2.0);
        let diff1 = s1.compare(&s2);
        let diff2 = s1.compare(&s3);
        assert!(diff1 < diff2);
    }
}
