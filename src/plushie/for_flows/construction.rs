mod hook;
mod hook_result;

use std::collections::{HashMap, HashSet};

use self::hook::Hook;
pub use self::hook_result::{Peculiarity, PointsOnPushPlane};
use super::centroid::Centroids;
use super::nodes::Nodes;
use super::Plushie;
use crate::common::*;
use crate::flow::Flow;

fn is_uniq(vec: &Vec<Point>) -> bool {
    let uniq = vec
        .into_iter()
        .map(|v| format!("{:?}", v.coords))
        .collect::<HashSet<_>>();
    uniq.len() == vec.len()
}

impl Plushie {
    fn new(
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
            params: Default::default(),
            centroids: Centroids::new(0, approximate_height),
            displacement,
        }
    }

    pub fn from_flow(flow: impl Flow) -> Result<Self, String> {
        let hook_result = Hook::parse(flow)?;

        if SANITY_CHECKS {
            assert!(
                is_uniq(&hook_result.nodes),
                "hook created duplicate positions"
            );
        }
        log::debug!(
            "edges: {:?}, len: {}",
            hook_result.edges,
            hook_result.edges.len()
        );
        log::debug!("nodes len: {}", hook_result.nodes.len());

        Ok(Plushie::new(
            hook_result.peculiarities,
            hook_result.colors,
            hook_result.edges.into(),
            hook_result.approximate_height,
        ))
    }

    pub fn parse(_pattern: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn _position_based_on(&mut self, _other: &Self) {
        todo!()
    }
}
