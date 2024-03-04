use std::ops::{Index, IndexMut};

use serde_derive::Serialize;

use crate::common::{Point, V};

pub const FIXED_POINTS_NUM: usize = 2;

#[derive(Clone, Serialize)]
pub struct Points {
    /// treat first N elements of `points` as fixed
    fixed: usize,
    points: Vec<Point>,
}

impl Points {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            fixed: FIXED_POINTS_NUM,
            points,
        }
    }

    pub fn as_vec(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn all<'a>(&'a self) -> impl Iterator<Item = (usize, &'a Point)> {
        self.points.iter().enumerate()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    fn movable<'a>(&'a mut self) -> impl Iterator<Item = (usize, &'a mut Point)> {
        self.points.iter_mut().enumerate().skip(self.fixed)
    }

    pub fn apply_forces(&mut self, displacement: &Vec<V>, time: f32) -> V {
        let mut total = V::zeros();
        let root_move = displacement[0] * time;
        for (i, point) in self.movable() {
            total += displacement[i];
            *point += displacement[i] * time - root_move;
        }
        self.points[1] += displacement[1].normalize() / 16.0 * time - root_move;
        // self.points[1].y += displacement[1].y.clamp(-1.0, 1.0) * time;
        total
    }
}

impl Index<usize> for Points {
    type Output = Point;
    fn index<'a>(&'a self, i: usize) -> &'a Point {
        &self.points[i]
    }
}

impl IndexMut<usize> for Points {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Point {
        &mut self.points[i]
    }
}
