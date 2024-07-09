mod animation;
mod centroid;
mod construction;
mod expanding;
mod nodes;

use std::{collections::HashMap, default, error::Error, fs::OpenOptions, hash::Hash};

use self::{centroid::Centroids, nodes::Nodes};
use super::{params::Initializer, Params, PlushieTrait};
use crate::{common::*, sanity};
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
    force_node_construction_timer: f32,
}

fn load_stl(filepath: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open(filepath)?;
    let stl = stl_io::read_stl(&mut file).unwrap();
    let points = stl
        .vertices
        .iter()
        .map(|v| Point::new(v[0], v[2], v[1]))
        .collect();
    Ok(points)
}

impl Plushie {
    pub fn dummy_from_stl(path: &str) -> Self {
        let points = load_stl(path).unwrap();
        let l = points.len();
        Self {
            nodes: Nodes::new(points, HashMap::new(), vec![(255, 255, 255); l]),
            edges: vec![vec![], vec![0]],
            edges_goal: vec![vec![], vec![0]],
            params: Params::default(),
            centroids: Centroids::new(0, 10.0),
            displacement: vec![],
            force_node_construction_timer: 0.0,
        }
    }

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
        use Initializer::*;
        match self.params.initializer {
            OneByOne(obo_params) => self.handle_adding_new_nodes(obo_params, time),
            Cylinder => (),
        }
        sanity!(self.nodes.assert_no_nans());
        self.step(time * self.params.timestep);
    }

    fn params(&mut self) -> &mut crate::plushie::Params {
        &mut self.params
    }

    fn set_params(&mut self, params: Params) {
        self.params = params;
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
            "edges": serde_json::json!(self.edges_goal), // OneByOne initializer swaps memory, so multiple tries to init the same plushie will fail
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
