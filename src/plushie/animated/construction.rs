pub mod hook;
mod hook_result;
mod initializer;

use self::hook::Hook;
pub use self::hook_result::{Peculiarity, PointsOnPushPlane};
use super::{centroid::Centroids, Params, Plushie};
use crate::{
    acl::{pest_parser::Pattern, Flow},
    common::*,
    plushie::params::NodeParam,
};

impl Plushie {
    pub fn from_flow(flow: impl Flow, params: Params) -> Result<Self, String> {
        //TEMP
        let params = {
            let mut params = params;
            params.nodes.insert(
                "part_first_hump".into(),
                NodeParam {
                    lock_x: Some(0.0),
                    lock_y: Some(0.0),
                    lock_z: Some(-2.0),
                },
            );
            params.nodes.insert(
                "part_second_hump".into(),
                NodeParam {
                    lock_x: Some(0.0),
                    lock_y: Some(0.0),
                    lock_z: Some(2.0),
                },
            );
            params
        };

        let hook_result = Hook::parse(flow, &params.hook_leniency)?;
        let mark_to_node = hook_result.mark_to_node.clone();
        let _part_limits = hook_result.part_limits.clone();
        let (nodes, edges, edges_goal, displacement) = params.initializer.apply_to(hook_result);

        let mut plushie = Self {
            displacement,
            edges_goal,
            edges,
            centroids: Centroids::new(0, 0.0),
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

        Ok(Self::from_flow(pattern, params)?)
    }
}
