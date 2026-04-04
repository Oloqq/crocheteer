// TODO refactor everything in here

use std::{
    collections::HashMap,
    f32::consts::PI,
    ops::{Index, IndexMut},
};

use crate::{acl::Color, hook::hook_result::Peculiarity, params::Params};

use super::hook_result::InitialGraph;

type V = glam::Vec3;
type Point = glam::Vec3;
pub type PointsOnPushPlane = (usize, usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OneByOneParams {
    /// Plushie will wait with expansion until the previous node is stabilized.
    /// Parameter sets the maximum displacement where the next node shall be added.
    pub acceptable_displacement_for_expanding: f32,
    /// If previous node cannot be stabilized, next one shall be added after set time.
    pub force_expansion_after_time: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Initializer {
    /// Start with a few stitches, and build the plushie while simulation is running.
    OneByOne(OneByOneParams),
    /// Start with points arranged roughly in the shape of a cylinder
    Cylinder,
}

#[derive(Debug, Clone)]
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
        // sanity!(new.assert_no_nans());
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
        let cross = ab.cross(ac);
        if cross.length() != 0.0 {
            let normal = cross.normalize() * direction;
            *affected += normal * params.single_loop_force;
        } else {
            log::warn!("Colinear points prevent applying single loop force");
        }
    }

    fn apply_peculiarities(&self, displacement: &mut Vec<V>, params: &Params) -> V {
        let mut global_shift = V::ZERO;
        const FULL_SINGLE_LOOP_FORCE_AFTER: usize = 20;
        for (i, peculiarity) in self.peculiarities.iter() {
            // with OneByOne initializer, nodes are introduced gradually while peculiarities are complete from the first step
            if *i >= displacement.len() {
                continue;
            }

            let dist_from_last_created = displacement.len() - i;
            let single_loop_constraint = if dist_from_last_created > FULL_SINGLE_LOOP_FORCE_AFTER {
                1.0 // TODO or plushie already has all the points
            } else {
                0.0
            };

            use Peculiarity::*;
            match peculiarity {
                Tip => (),
                Locked => {
                    if params.reflect_locked {
                        global_shift += displacement[*i];
                    }
                    displacement[*i] = V::ZERO;
                }
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

        global_shift
    }

    pub fn apply_forces(&mut self, displacement: &mut Vec<V>, time: f32, params: &Params) -> V {
        let mut total = V::ZERO;

        let global_shift = self.apply_peculiarities(displacement, params);

        for (i, point) in self.points.iter_mut().enumerate() {
            if displacement[i].length() > params.minimum_displacement {
                total += displacement[i];
                *point += (displacement[i] - global_shift) * time;
                if params.floor {
                    point.y = point.y.max(0.0);
                }
            }
        }
        total
    }

    fn apply_peculiarities_old(&self, displacement: &mut Vec<V>, params: &Params) -> V {
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
                Locked => {
                    assert!(
                        root_index.is_none(),
                        "Multiple nodes got locked without multipart"
                    );
                    root_index = Some(i);
                }
                Tip => (),
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

        match (params.reflect_locked, root_index) {
            (true, Some(i)) => displacement[*i],
            // (true, None) => todo!("Keep plushies started from a chain in the middle somehow"),
            (true, None) => V::ZERO,
            (false, _) => V::ZERO,
        }
    }

    pub fn apply_forces_old(&mut self, displacement: &mut Vec<V>, time: f32, params: &Params) -> V {
        let mut total = V::ZERO;

        let root_displacement = self.apply_peculiarities_old(displacement, params);
        let translation_by_root = if params.reflect_locked {
            root_displacement
        } else {
            V::ZERO
        };

        for (i, point) in self.points.iter_mut().enumerate() {
            if displacement[i].length() > params.minimum_displacement {
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

impl Initializer {
    pub fn apply_to(
        &self,
        graph: InitialGraph,
    ) -> (Nodes, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<V>) {
        let nodes = Nodes::new(
            match self {
                Initializer::OneByOne(_) => vec![],
                Initializer::Cylinder => unimplemented!(),
            },
            graph.peculiarities,
            graph.colors,
        );

        let edges: Vec<Vec<usize>>;
        let edges_goal: Vec<Vec<usize>>;
        let mut displacement = Vec::with_capacity(graph.edges.len());
        match self {
            Initializer::OneByOne(_) => {
                edges = vec![];
                edges_goal = graph.edges.into();
                displacement.push(V::ZERO);
            }
            Initializer::Cylinder => {
                edges = graph.edges.into();
                edges_goal = edges.clone();
                displacement = vec![V::ZERO; edges.len()];
            }
        }

        (nodes, edges, edges_goal, displacement)
    }
}
