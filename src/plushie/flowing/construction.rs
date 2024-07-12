mod hook;
mod hook_result;

use self::hook::Hook;
use self::hook_result::HookResult;
pub use self::hook_result::{Peculiarity, PointsOnPushPlane};
use super::centroid::Centroids;
use super::nodes::Nodes;
use super::Plushie;
use super::{Initializer, Params};
use crate::common::*;
use crate::flow::pest_parser::Pattern;
use crate::flow::Flow;
use std::collections::HashMap;

impl Plushie {
    fn for_one_by_one(
        params: Params,
        peculiarities: HashMap<usize, Peculiarity>,
        colors: Vec<Color>,
        edges: Vec<Vec<usize>>,
    ) -> Self {
        let mut displacement = Vec::with_capacity(edges.len());
        displacement.push(V::zeros());
        Self {
            nodes: Nodes::new(vec![Point::new(0.0, 0.0, 0.0)], peculiarities, colors),
            edges: vec![vec![]],
            edges_goal: edges,
            params,
            centroids: Centroids::new(0, 0.0),
            displacement,
            force_node_construction_timer: 0.0,
            // initializing with INF so it won't come as relaxed before first step by accident
            last_total_displacement: V::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
        }
    }

    fn with_initial_positions(params: Params, hook_result: HookResult) -> Self {
        let edges: Vec<Vec<usize>> = hook_result.edges.into();
        let nodes = Nodes::new(
            hook_result.nodes,
            hook_result.peculiarities,
            hook_result.colors,
        );
        Self {
            displacement: vec![V::zeros(); edges.len()],
            edges_goal: edges.clone(),
            edges,
            centroids: Centroids::new(params.centroids.number, hook_result.approximate_height),
            params,
            nodes,
            force_node_construction_timer: 0.0,
            // initializing with INF so it won't come as relaxed before first step by accident
            last_total_displacement: V::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
        }
    }

    pub fn from_flow(flow: impl Flow, params: Params) -> Result<Self, String> {
        let hook_result = Hook::parse(flow)?;

        Ok(match params.initializer {
            Initializer::OneByOne(_) => Plushie::for_one_by_one(
                params,
                hook_result.peculiarities,
                hook_result.colors,
                hook_result.edges.into(),
            ),
            Initializer::Cylinder => Plushie::with_initial_positions(params, hook_result),
        })
    }

    pub fn parse(src: &str) -> Result<Self, String> {
        let pattern = Pattern::parse(src)?;
        let mut params = Params::default();
        params.update(&pattern.meta);
        Ok(Self::from_flow(pattern, params)?)
    }

    pub fn _position_based_on(&mut self, _other: &Self) {
        todo!()
    }
}
