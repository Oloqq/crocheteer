mod hook;
mod hook_result;

use self::hook::Hook;
pub use self::hook_result::{Peculiarity, PointsOnPushPlane};
use super::centroid::Centroids;
use super::nodes::Nodes;
use super::Plushie;
use super::{Initializer, Params};
use crate::common::*;
use crate::flow::Flow;
use std::collections::HashMap;

impl Plushie {
    fn for_one_by_one(
        params: Params,
        peculiarities: HashMap<usize, Peculiarity>,
        colors: Vec<Color>,
        edges: Vec<Vec<usize>>,
        approximate_height: f32,
    ) -> Self {
        let mut displacement = Vec::with_capacity(edges.len());
        displacement.push(V::zeros());
        Self {
            nodes: Nodes::new(vec![Point::new(0.0, 0.0, 0.0)], peculiarities, colors),
            edges: vec![vec![]],
            edges_goal: edges,
            params,
            centroids: Centroids::new(0, approximate_height),
            displacement,
            force_node_construction_timer: 0.0,
        }
    }

    fn with_initial_positions(
        params: Params,
        nodes: Nodes,
        edges: Vec<Vec<usize>>,
        approximate_height: f32,
    ) -> Self {
        Self {
            displacement: vec![V::zeros(); edges.len()],
            edges_goal: edges.clone(),
            edges,
            centroids: Centroids::new(params.centroids.number, approximate_height),
            params,
            nodes,
            force_node_construction_timer: 0.0,
        }
    }

    pub fn from_flow(flow: impl Flow) -> Result<Self, String> {
        let hook_result = Hook::parse(flow)?;
        let params: Params = Default::default();

        Ok(match params.initializer {
            Initializer::OneByOne(_) => Plushie::for_one_by_one(
                params,
                hook_result.peculiarities,
                hook_result.colors,
                hook_result.edges.into(),
                hook_result.approximate_height,
            ),
            Initializer::Cylinder => {
                let nodes = Nodes::new(
                    hook_result.nodes,
                    hook_result.peculiarities,
                    hook_result.colors,
                );
                Plushie::with_initial_positions(
                    params,
                    nodes,
                    hook_result.edges.into(),
                    hook_result.approximate_height,
                )
            }
        })
    }

    pub fn parse(_pattern: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn _position_based_on(&mut self, _other: &Self) {
        todo!()
    }
}
