pub mod hook;
mod hook_result;
mod initializer;

use self::hook::Hook;
pub use self::hook_result::{Peculiarity, PointsOnPushPlane};
use super::{centroid::Centroids, Limb, Params, Plushie};
use crate::{
    acl::{pest_parser::Pattern, Flow},
    common::*,
    plushie::params::Initializer,
};

impl Plushie {
    pub fn from_flow(flow: impl Flow, params: Params) -> Result<Self, String> {
        let hook_result = Hook::parse(flow, &params.hook)?;
        let mark_to_node = hook_result.mark_to_node.clone();
        let limbs = {
            hook_result
                .part_limits
                .windows(2)
                .map(|win| Limb {
                    skin_start: win[0],
                    skin_end: win[1],
                    centroids: Centroids::new(0, 0.0),
                })
                .collect()
        };
        let (nodes, edges, edges_goal, displacement) = params.initializer.apply_to(hook_result);

        let mut plushie = Self {
            limbs,
            displacement,
            edges_goal,
            edges,
            params,
            nodes,
            force_node_construction_timer: 0.0,
            // initializing with INF so it won't come as relaxed before first step by accident
            last_total_displacement: V::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            perf: vec![],
            mark_to_node,
        };
        plushie.apply_node_params();

        Ok(plushie)
    }

    pub fn parse(src: &str) -> Result<Self, String> {
        let pattern = Pattern::parse(src)?;
        let mut params = Params::default();
        let update_errors = params.update(&pattern.parameters);
        if update_errors.len() > 0 {
            return Err(update_errors[0].clone());
        }

        if !params.reflect_locked {
            // TODO ensure at least one point is locked
        }

        if params.skelet_stuffing.enable {
            return Err("Skeletonization with multipart has not been tested".into());
        }

        params.limbs = pattern.limbs.clone();

        Ok(Self::from_flow(pattern, params)?)
    }

    /// Initializes node positions based on existing Plushie
    /// Assumptions:
    /// - edges_goal was not modified after reading the pattern
    /// - any initializer fills edges_goal
    pub fn inherit_from(&mut self, parent: &Plushie) {
        let parent_nodes = parent.nodes.len();
        assert_eq!(parent_nodes, parent.edges.len());
        let mut i = 0;
        while i < parent_nodes {
            if i >= self.edges_goal.len() {
                break;
            }
            if self.edges_goal[i] != parent.edges_goal[i] {
                break;
            }
            assert_eq!(parent.edges_goal[i], parent.edges[i]);

            let position = parent.nodes.points[i];
            match self.params.initializer {
                Initializer::Cylinder => {
                    self.nodes.points[i] = position;
                }
                Initializer::OneByOne(_) => {
                    self.construct_node(i, position);
                }
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inheritance_cylinder_same_pattern() {
        let marker = Point::new(15.0, 15.0, 15.0);
        let mut parent = Plushie::parse("MR(6)").unwrap();
        parent.nodes.points[0] = marker.clone();
        parent.nodes.points[5] = marker.clone();

        let mut new = Plushie::parse("MR(6)").unwrap();
        new.inherit_from(&parent);

        assert_eq!(new.nodes.points[0], parent.nodes.points[0]);
        assert_eq!(new.nodes.points[5], parent.nodes.points[5]);
        assert_eq!(new.nodes.points, parent.nodes.points);
    }

    #[test]
    fn test_inheritance_obo_no_nodes_same_pattern_no_panic() {
        let parent = Plushie::parse(
            "@initializer = obo
            MR(6)",
        )
        .unwrap();

        let mut new = Plushie::parse(
            "@initializer = obo
            MR(6)",
        )
        .unwrap();
        new.inherit_from(&parent);
    }

    #[test]
    fn test_inheritance_obo_some_nodes_same_pattern() {
        let marker = Point::new(15.0, 15.0, 15.0);
        let mut parent = Plushie::parse(
            "@initializer = obo
            MR(6)",
        )
        .unwrap();
        parent.construct_node(0, marker);
        parent.construct_node(1, marker);

        let mut new = Plushie::parse(
            "@initializer = obo
            MR(6)",
        )
        .unwrap();
        new.inherit_from(&parent);

        assert_eq!(parent.nodes.points.len(), 2);
        assert_eq!(new.nodes.points.len(), 2);
        assert_eq!(parent.edges.len(), 2);
        assert_eq!(new.edges.len(), 2);
        assert_eq!(new.nodes.points[0], parent.nodes.points[0]);
        assert_eq!(new.nodes.points[1], parent.nodes.points[1]);
    }
}
