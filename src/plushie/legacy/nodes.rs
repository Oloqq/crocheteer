use std::ops::{Index, IndexMut};

use serde_derive::Serialize;

use crate::common::{Point, V};

pub const ROOT_INDEX: usize = 0;

#[derive(Clone, Serialize)]
pub struct Nodes {
    /// Constraints of the first points. The calculated movement will be multiplied by them.
    constraints: Vec<V>,
    /// All points in the shape
    pub points: Vec<Point>,
    /// true => the whole shape will be translated by displacement applied to root, so that root stays at (0, 0, 0).
    ///     `constrains[0]` is ignored
    ///     keep in mind, `constraints[0]` still corresponds to root, and `constraints[1]` to the next one
    /// false => the root is treated accordingly to `constraints`
    keep_root_at_origin: bool,
}

impl Nodes {
    pub fn new(points: Vec<Point>, constraints: Vec<V>) -> Self {
        let keep_root_at_origin = true;
        if keep_root_at_origin {
            assert!(
                constraints.len() == 0 || constraints[0] == V::zeros(),
                "Root's constraint should be 0 if it is to be kept at origin"
            );
        }

        Self {
            constraints,
            points,
            keep_root_at_origin,
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

    fn freely_movable<'a>(&'a mut self) -> impl Iterator<Item = (usize, &'a mut Point)> {
        self.points
            .iter_mut()
            .enumerate()
            .skip(self.constraints.len())
    }

    pub fn apply_forces(&mut self, displacement: &Vec<V>, time: f32) -> V {
        let mut total = V::zeros();
        let root_move = match self.keep_root_at_origin {
            true => displacement[ROOT_INDEX],
            false => V::zeros(),
        };

        const SKIP_ROOT: usize = 1;
        const SITTING: bool = true;

        for ((i, point), constraint) in self
            .points
            .iter_mut()
            .enumerate()
            .zip(&self.constraints)
            .skip(SKIP_ROOT)
        {
            let adjusted: V = displacement[i].component_mul(&constraint);
            total += adjusted;
            *point += (adjusted - root_move) * time;
            if SITTING {
                point.y = point.y.max(0.0);
            }
        }

        for (i, point) in self.freely_movable() {
            total += displacement[i];
            *point += (displacement[i] - root_move) * time;
            if SITTING {
                point.y = point.y.max(0.0);
            }
        }
        total
    }
}

impl Index<usize> for Nodes {
    type Output = Point;
    fn index<'a>(&'a self, i: usize) -> &'a Point {
        &self.points[i]
    }
}

impl IndexMut<usize> for Nodes {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Point {
        &mut self.points[i]
    }
}
