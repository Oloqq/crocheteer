use super::{
    construction::{Peculiarity, PointsOnPushPlane},
    Params,
};
use crate::common::*;
use serde_derive::Serialize;
use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Serialize)]
pub struct Nodes {
    pub points: Vec<Point>,
    pub peculiarities: HashMap<usize, Peculiarity>,
    pub colors: Vec<Color>,
}

impl Nodes {
    pub fn new(
        points: Vec<Point>,
        peculiarities: HashMap<usize, Peculiarity>,
        colors: Vec<Color>,
    ) -> Self {
        let new = Self {
            points,
            peculiarities,
            colors,
        };
        new.assert_no_nans();
        new
    }

    pub fn as_vec(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn assert_no_nans(&self) {
        for p in &self.points {
            assert!(!p.x.is_nan());
            assert!(!p.y.is_nan());
            assert!(!p.z.is_nan());
        }
    }

    fn apply_single_loop(&self, affected: &mut V, plane_spec: &PointsOnPushPlane, direction: f32) {
        return;
        let (ia, ib, ic) = plane_spec;
        let a = self.points[*ia];
        let b = self.points[*ib];
        let c = self.points[*ic];
        let ab = b - a;
        let ac = c - a;
        let cross = ab.cross(&ac);
        if cross.magnitude() != 0.0 {
            let normal = cross.normalize() * direction;
            const SINGLE_LOOP_FORCE: f32 = 0.05;
            *affected += normal * SINGLE_LOOP_FORCE;
        } else {
            log::warn!("Colinear points prevent applying single loop force");
        }
    }

    fn apply_peculiarities(&self, displacement: &mut Vec<V>, params: &Params) -> V {
        let mut root_index = None;
        for (i, peculiarity) in self.peculiarities.iter() {
            if *i >= displacement.len() {
                continue;
            }

            use Peculiarity::*;
            match peculiarity {
                Root => {
                    assert!(root_index.is_none(), "Multiple nodes got marked as root");
                    root_index = Some(i);
                }
                Tip => (),
                Constrained(v) => displacement[*i].component_mul_assign(&v),
                BLO(plane_spec) => self.apply_single_loop(&mut displacement[*i], plane_spec, 1.0),
                FLO(plane_spec) => self.apply_single_loop(&mut displacement[*i], plane_spec, -1.0),
            }
        }

        match (params.keep_root_at_origin, root_index) {
            (true, Some(i)) => displacement[*i],
            // (true, None) => todo!("Keep plushies started from a chain in the middle somehow"),
            (true, None) => V::zeros(),
            (false, _) => V::zeros(),
        }
    }

    pub fn apply_forces(&mut self, displacement: &mut Vec<V>, time: f32, params: &Params) -> V {
        let mut total = V::zeros();

        let translation_by_root = self.apply_peculiarities(displacement, params);

        for (i, point) in self.points.iter_mut().enumerate() {
            total += displacement[i];
            *point += (displacement[i] - translation_by_root) * time;
            if params.floor {
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
