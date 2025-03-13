mod animation;
mod centroid;
mod construction;
mod expanding;
mod nodes;
pub mod perf;

use std::collections::HashMap;

use serde_derive::Serialize;

use self::{centroid::Centroids, nodes::Nodes};
use super::{params::Initializer, Params, PlushieTrait};
use crate::{common::*, sanity};

type Edges = Vec<Vec<usize>>;

#[derive(Clone, Serialize)]
pub struct Plushie {
    pub nodes: Nodes,
    pub edges: Edges,
    edges_goal: Vec<Vec<usize>>, // ideally this would be replaced with a Queue, but right now frontend gets list of edges just once at the beginning
    pub params: Params,
    displacement: Vec<V>,
    force_node_construction_timer: f32,
    last_total_displacement: V,
    pub perf: Vec<perf::Iteration>,
    mark_to_node: HashMap<String, usize>,
    limbs: Vec<Limb>,
}

#[derive(Clone, Serialize)]
struct Limb {
    skin_start: usize,
    skin_end: usize,
    centroids: Centroids,
}

impl PlushieTrait for Plushie {
    fn step(&mut self, time: f32) {
        use Initializer::*;
        match self.params.initializer {
            OneByOne(obo_params) => self.handle_adding_new_nodes(obo_params, time),
            Cylinder => (),
        }
        sanity!(self.nodes.assert_no_nans());
        self.step(time * self.params.timestep);
    }

    fn params(&self) -> &Params {
        &self.params
    }

    fn params_mut(&mut self) -> &mut crate::plushie::Params {
        &mut self.params
    }

    fn set_params(&mut self, params: Params) {
        self.params = params;
    }

    fn nodes_to_json(&self) -> JSON {
        serde_json::json!(self.nodes.as_vec())
    }

    fn centroids_to_json(&self) -> JSON {
        let all_bones: Vec<&Point> = self
            .limbs
            .iter()
            .flat_map(|limb| &limb.centroids.points)
            .collect();
        serde_json::json!({ "points": all_bones })
    }

    fn init_data(&self) -> JSON {
        let all_bones: Vec<&Point> = self
            .limbs
            .iter()
            .flat_map(|limb| &limb.centroids.points)
            .collect();
        serde_json::json!({
            "nodes": serde_json::json!(self.nodes),
            "edges": serde_json::json!(self.edges_goal), // OneByOne initializer swaps memory, so multiple tries to init the same plushie will fail
            "centroids": serde_json::json!({ "points": all_bones })
        })
    }

    fn set_point_position(&mut self, i: usize, pos: Point) {
        if i >= self.nodes.len() {
            // using websockets, this could theoretically happen with reloading and some network delays
            panic!("Point index greater than vector size");
        }
        self.nodes[i] = pos;
    }

    fn clonebox(&self) -> Box<dyn PlushieTrait> {
        Box::new(Clone::clone(self))
    }

    fn get_points(&self) -> &Vec<Point> {
        &self.nodes.as_vec()
    }

    fn as_animated(&self) -> Option<&self::Plushie> {
        Some(&self)
    }
}
