use crate::traits::plushie::PlushieTrait;

use self::{animation::centroid::Centroids, nodes::Nodes, params::Params};
use super::common::*;

use serde_derive::Serialize;

mod animation;
mod construction;
mod conversions;
pub mod examples;
mod nodes;
pub mod params;

type Edges = Vec<Vec<usize>>;

#[derive(Clone, Serialize)]
pub enum Stuffing {
    None,
    Centroids,
}

#[derive(Clone, Serialize)]
pub struct Plushie {
    // keep in mind that those field names are important in the frontend in current communication
    nodes: Nodes,
    edges: Edges,
    pub params: Params,

    pub centroids: Centroids,
    pub stuffing: Stuffing,
}

impl Plushie {
    pub fn new(points: Nodes, edges: Edges, params: Params, centroids: Centroids) -> Self {
        Self {
            stuffing: Stuffing::Centroids,
            nodes: points,
            edges,
            params,
            centroids,
        }
    }

    fn is_relaxed(&self, displacement: &Vec<V>) -> bool {
        // TODO: elbow method
        let tension: f32 = displacement.iter().map(|v| v.magnitude()).sum();
        tension <= self.params.acceptable_tension
    }

    pub fn get_points_vec(&self) -> &Vec<Point> {
        self.nodes.as_vec()
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

    fn set_point_position(&mut self, i: usize, pos: Point) {
        if i >= self.nodes.len() {
            // using websockets, this could theoretically happen with reloading and some network delays
            panic!("Point index greater than vector size");
        }
        self.nodes[i] = pos;
    }

    fn set_centroid_num(&mut self, num: usize) {
        self.centroids.set_centroid_num(num, &self.nodes)
    }
}
