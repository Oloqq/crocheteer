use crate::common::Point;

pub fn local_surface_normals_per_point(_points: &Vec<Point>) -> Vec<Point> {
    vec![Point::new(1.0, 0.0, 0.0)]
}
