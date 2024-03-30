mod hook;
mod hook_result;

use std::collections::HashSet;

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

        Ok(Plushie {
            nodes: Nodes::new(
                vec![Point::new(0.0, 0.0, 0.0)],
                hook_result.peculiarities,
                hook_result.colors,
            ),
            edges: vec![vec![]],
            edges_goal: hook_result.edges.into(),
            params: Default::default(),
            centroids: Centroids::new(0, hook_result.approximate_height),
        })
    }

    pub fn parse(_pattern: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn _position_based_on(&mut self, _other: &Self) {
        todo!()
    }
}
