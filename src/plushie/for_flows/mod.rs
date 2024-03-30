mod animation;
mod centroid;
mod construction;
mod expanding;
mod nodes;

use self::{centroid::Centroids, nodes::Nodes};
use super::{Params, PlushieTrait};
use crate::common::*;
use serde_derive::Serialize;

type Edges = Vec<Vec<usize>>;

#[derive(Clone, Serialize)]
pub struct Plushie {
    nodes: Nodes,
    edges: Edges,
    edges_goal: Vec<Vec<usize>>, // ideally this would be replaced with a Queue, but right now frontend gets list of edges just once at the beginning
    pub params: Params,
    pub centroids: Centroids,
    displacement: Vec<V>,
    force_node_construction_timer: i32,
}

impl Plushie {
    fn is_relaxed(&self, displacement: &V) -> bool {
        // TODO: elbow method
        let tension: f32 = displacement.magnitude();
        tension <= self.params.autostop.acceptable_tension
    }
}

impl PlushieTrait for Plushie {
    fn animate(&mut self) {
        for _ in 0..self.params.autostop.max_relaxing_iterations {
            let total_displacement = self.step(self.params.timestep);
            if self.is_relaxed(&total_displacement) {
                break;
            }
        }
    }

    fn step(&mut self, time: f32) {
        self.handle_adding_new_nodes();
        self.nodes.assert_no_nans(); // TODO macro
        self.step(time);
    }

    fn params(&mut self) -> &mut crate::plushie::Params {
        &mut self.params
    }

    fn nodes_to_json(&self) -> JSON {
        serde_json::json!(self.nodes.as_vec())
    }

    fn centroids_to_json(&self) -> JSON {
        serde_json::json!(self.centroids)
    }

    fn init_data(&self) -> JSON {
        serde_json::json!({
            "nodes": serde_json::json!(self.nodes),
            "edges": serde_json::json!(self.edges_goal),
            "centroids": serde_json::json!(self.centroids)
        })
    }

    fn set_point_position(&mut self, i: usize, pos: Point) {
        if i >= self.nodes.len() {
            // using websockets, this could theoretically happen with reloading and some network delays
            panic!("Point index greater than vector size");
        }
        self.nodes[i] = pos;
    }

    fn clone(&self) -> Box<dyn PlushieTrait> {
        Box::new(Clone::clone(self))
    }
}
