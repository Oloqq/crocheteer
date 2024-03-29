mod hook;
mod hook_result;

use std::collections::HashSet;

use self::hook::Hook;
pub use self::hook_result::{Peculiarity, PointsOnPushPlane};
use super::animation::centroid::Centroids;
use super::nodes::Nodes;
use super::Plushie;
use crate::common::*;
use crate::flow::actions::Action;
use crate::flow::Flow;

fn is_uniq(vec: &Vec<Point>) -> bool {
    let uniq = vec
        .into_iter()
        .map(|v| format!("{:?}", v.coords))
        .collect::<HashSet<_>>();
    uniq.len() == vec.len()
}

const USE_APPROXIMATE_POSITIONS: bool = true;

impl Plushie {
    pub fn from_flow(flow: impl Flow) -> Result<Self, String> {
        let starting_stitches = {
            match flow.peek() {
                Some(Action::MR(x) | Action::Ch(x)) => x,
                None => panic!("Empty flow?"),
                _ => panic!("Wrong starter?"),
            }
        };
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

        let initial_positions = if USE_APPROXIMATE_POSITIONS {
            hook_result.nodes
        } else {
            hook_result.nodes[0..starting_stitches].to_owned()
        };

        Ok(Plushie {
            nodes: Nodes::new(
                initial_positions,
                hook_result.peculiarities,
                hook_result.colors,
            ),
            edges: hook_result.edges.into(),
            params: Default::default(),
            centroids: Centroids::new(2, hook_result.approximate_height),
        })
    }

    pub fn parse(_pattern: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn _position_based_on(&mut self, _other: &Self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::flow::simple_flow::SimpleFlow;

    use super::*;

    #[test]
    fn test_open_shape() {
        use crate::flow::actions::Action;
        use Action::*;
        let mut actions: Vec<Action> = vec![MR(6)];
        actions.append(&mut vec![Sc; 6]);

        let flow = SimpleFlow::new(actions);
        let plushie = Plushie::from_flow(flow).unwrap();

        assert_eq!(plushie.nodes.len(), 13)
    }

    #[test]
    fn test_closed_shape() {
        use crate::flow::actions::Action;
        use Action::*;
        let mut actions: Vec<Action> = vec![MR(6)];
        actions.append(&mut vec![Sc; 6]);
        actions.append(&mut vec![FO]);

        let flow = SimpleFlow::new(actions);
        let plushie = Plushie::from_flow(flow).unwrap();

        assert_eq!(plushie.nodes.len(), 14)
    }
}
