use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use serde_derive::Serialize;

use super::{
    construction::{Peculiarity, PointsOnPushPlane},
    Params,
};
use crate::{
    common::{colors::*, *},
    sanity,
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
        sanity!(new.assert_no_nans());
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

    fn apply_single_loop(
        &self,
        affected: &mut V,
        plane_spec: &PointsOnPushPlane,
        direction: f32,
        params: &Params,
    ) {
        let (ia, ib, ic) = plane_spec;
        let a = self.points[*ia];
        let b = self.points[*ib];
        let c = self.points[*ic];
        let ab = b - a;
        let ac = c - a;
        let cross = ab.cross(&ac);
        if cross.magnitude() != 0.0 {
            let normal = cross.normalize() * direction;
            *affected += normal * params.single_loop_force;
        } else {
            log::warn!("Colinear points prevent applying single loop force");
        }
    }

    fn apply_peculiarities(&self, displacement: &mut Vec<V>, params: &Params) -> V {
        let mut root_index = None;
        const FULL_SINGLE_LOOP_FORCE_AFTER: usize = 20;
        for (i, peculiarity) in self.peculiarities.iter() {
            if *i >= displacement.len() {
                continue;
            }

            let dist_from_end = displacement.len() - i;
            let single_loop_constraint = if dist_from_end > FULL_SINGLE_LOOP_FORCE_AFTER {
                // TODO or plushie already has all the points
                1.0
            } else {
                // dist_from_end as f32 / FULL_SINGLE_LOOP_FORCE_AFTER as f32
                0.0
            };

            use Peculiarity::*;
            match peculiarity {
                Root => {
                    assert!(root_index.is_none(), "Multiple nodes got marked as root");
                    root_index = Some(i);
                }
                Tip => (),
                Constrained(v) => displacement[*i].component_mul_assign(&v),
                BLO(plane_spec) => self.apply_single_loop(
                    &mut displacement[*i],
                    plane_spec,
                    single_loop_constraint,
                    params,
                ),
                FLO(plane_spec) => self.apply_single_loop(
                    &mut displacement[*i],
                    plane_spec,
                    -single_loop_constraint,
                    params,
                ),
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

        let root_displacement = self.apply_peculiarities(displacement, params);
        let translation_by_root = if params.keep_root_at_origin {
            root_displacement
        } else {
            V::zeros()
        };

        for (i, point) in self.points.iter_mut().enumerate() {
            if displacement[i].magnitude() > params.minimum_displacement {
                total += displacement[i];
                *point += (displacement[i] - translation_by_root) * time;
                if params.floor {
                    point.y = point.y.max(0.0);
                }
            }
        }
        total
    }

    fn apply_peculiarities_multipart(&self, displacement: &mut Vec<V>, params: &Params) -> V {
        let mut root_index = None;
        const FULL_SINGLE_LOOP_FORCE_AFTER: usize = 20;
        for (i, peculiarity) in self.peculiarities.iter() {
            if *i >= displacement.len() {
                continue;
            }

            let dist_from_end = displacement.len() - i;
            let single_loop_constraint = if dist_from_end > FULL_SINGLE_LOOP_FORCE_AFTER {
                // TODO or plushie already has all the points
                1.0
            } else {
                // dist_from_end as f32 / FULL_SINGLE_LOOP_FORCE_AFTER as f32
                0.0
            };

            use Peculiarity::*;
            match peculiarity {
                Root => {
                    assert!(root_index.is_none(), "Multiple nodes got marked as root");
                    root_index = Some(i);
                }
                Tip => (),
                Constrained(v) => displacement[*i].component_mul_assign(&v),
                BLO(plane_spec) => self.apply_single_loop(
                    &mut displacement[*i],
                    plane_spec,
                    single_loop_constraint,
                    params,
                ),
                FLO(plane_spec) => self.apply_single_loop(
                    &mut displacement[*i],
                    plane_spec,
                    -single_loop_constraint,
                    params,
                ),
            }
        }

        match (params.keep_root_at_origin, root_index) {
            (true, Some(i)) => displacement[*i],
            // (true, None) => todo!("Keep plushies started from a chain in the middle somehow"),
            (true, None) => V::zeros(),
            (false, _) => V::zeros(),
        }
    }

    pub fn apply_forces_multipart(
        &mut self,
        displacement: &mut Vec<V>,
        time: f32,
        params: &Params,
    ) -> V {
        let mut total = V::zeros();

        let root_displacement = self.apply_peculiarities(displacement, params);
        let translation_by_root = if params.keep_root_at_origin {
            root_displacement
        } else {
            V::zeros()
        };

        for (i, point) in self.points.iter_mut().enumerate() {
            if displacement[i].magnitude() > params.minimum_displacement {
                total += displacement[i];
                *point += (displacement[i] - translation_by_root) * time;
                if params.floor {
                    point.y = point.y.max(0.0);
                }
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
