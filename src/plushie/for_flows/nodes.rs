use super::{construction::Peculiarity, Params};
use crate::common::*;
use serde_derive::Serialize;
use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

pub const ROOT_INDEX: usize = 0;

#[derive(Clone, Serialize)]
pub struct Nodes {
    pub points: Vec<Point>,
    pub peculiarities: HashMap<usize, Peculiarity>,
}

impl Nodes {
    pub fn new(points: Vec<Point>, peculiarities: HashMap<usize, Peculiarity>) -> Self {
        Self {
            points,
            peculiarities,
        }
    }

    pub fn as_vec(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn apply_forces(&mut self, displacement: &Vec<V>, time: f32, params: &Params) -> V {
        let mut total = V::zeros();
        let root_move = match params.keep_root_at_origin {
            true => displacement[ROOT_INDEX],
            false => V::zeros(),
        };

        const SKIP_ROOT: usize = {
            assert!(ROOT_INDEX == 0);
            1
        };

        for (i, point) in self.points.iter_mut().enumerate().skip(SKIP_ROOT) {
            total += displacement[i];
            *point += (displacement[i] - root_move) * time;
            if params.sitting {
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
