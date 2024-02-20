use crate::common::Point;
mod comparison_naive;
mod construction;

type Point2 = na::Point2<f32>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Slice {
    points: Vec<Point2>,
}

#[derive(Clone)]
pub struct Shape {
    slices: Vec<Slice>,
}

impl Slice {
    pub fn from_3d(points: Vec<Point>) -> Self {
        let points2d = points.iter().map(|p3d| Point2::from(p3d.xz())).collect();
        Self { points: points2d }
    }
}

pub fn compare_shapes(original: &Shape, other: &Shape) -> f32 {
    original.compare(other)
}