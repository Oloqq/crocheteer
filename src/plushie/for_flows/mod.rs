mod animation;
mod construction;
mod nodes;

use self::{animation::centroid::Centroids, nodes::Nodes};
use super::{Params, PlushieTrait};
use crate::common::*;
use serde_derive::Serialize;

type Edges = Vec<Vec<usize>>;

#[derive(Clone, Serialize)]
pub struct Plushie {
    // keep in mind that those field names are important in the frontend in current communication
    nodes: Nodes,
    edges: Edges,
    pub params: Params,

    pub centroids: Centroids,
}

impl Plushie {
    fn is_relaxed(&self, displacement: &Vec<V>) -> bool {
        // TODO: elbow method
        let tension: f32 = displacement.iter().map(|v| v.magnitude()).sum();
        tension <= self.params.acceptable_tension
    }
}

impl PlushieTrait for Plushie {
    fn animate(&mut self) {
        for _ in 0..self.params.max_relaxing_iterations {
            let displacement = self.step(1.0);
            if self.is_relaxed(&displacement) {
                break;
            }
        }
    }

    fn step(&mut self, time: f32) {
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

    fn whole_to_json(&self) -> JSON {
        serde_json::json!(self)
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