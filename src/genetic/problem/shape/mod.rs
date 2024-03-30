use serde_derive::{Deserialize, Serialize};

use crate::common::Point;
mod comparison_naive;
mod construction;

type Point2 = na::Point2<f32>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Slice {
    points: Vec<Point2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    slices: Vec<Slice>,
}

impl Slice {
    pub fn from_3d(points: Vec<Point>) -> Self {
        let points2d: Vec<Point2> = points.iter().map(|p3d| Point2::from(p3d.xz())).collect();
        Self { points: points2d }
    }
}

impl Shape {
    pub fn point_count(&self) -> usize {
        self.slices.iter().fold(0, |acc, s| acc + s.points.len())
    }

    pub fn serialize(&self) -> String {
        serde_lexpr::to_string(&self).unwrap()
    }

    // #[allow(unused)]
    pub fn deserialize(s: &str) -> Self {
        serde_lexpr::from_str(s).unwrap()
    }
}

pub fn compare_shapes(original: &Shape, other: &Shape) -> f32 {
    original.compare(other)
}
