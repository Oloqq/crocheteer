mod animation;
mod construction;
mod nodes;

use self::{animation::centroid::Centroids, nodes::Nodes};
use super::{Params, PlushieTrait};
use crate::common::*;
use serde_derive::Serialize;

use std::mem::swap;

type Edges = Vec<Vec<usize>>;

#[derive(Clone, Serialize)]
pub struct Plushie {
    // keep in mind that those field names are important in the frontend in current communication
    nodes: Nodes,
    edges: Edges,
    edges_goal: Vec<Vec<usize>>,
    pub params: Params,
    pub centroids: Centroids,
}

impl Plushie {
    fn is_relaxed(&self, displacement: &V) -> bool {
        // TODO: elbow method
        let tension: f32 = displacement.magnitude();
        tension <= self.params.acceptable_tension
    }

    fn construct_node(&self, based_on: &Vec<usize>) -> Point {
        if based_on.len() == 0 {
            panic!("Node should be attached to something");
        } else if based_on.len() == 1 {
            let base = self.nodes.points[based_on[0]];
            let coords = base.coords + V::new(0.1, 0.1, 0.1);
            Point::from(coords)
        } else {
            let mut avg = V::zeros();
            for base in based_on {
                let point = self.nodes.points[*base];
                avg += point.coords;
            }
            avg /= based_on.len() as f32;
            Point::from(avg)
        }
    }

    fn construct_next(&mut self) {
        let index = self.edges.len();
        self.edges.push(vec![]);
        swap(&mut self.edges[index], &mut self.edges_goal[index]);
        let node = self.construct_node(&self.edges[index]);
        self.nodes.points.push(node);
    }
}

impl PlushieTrait for Plushie {
    fn animate(&mut self) {
        for _ in 0..self.params.max_relaxing_iterations {
            let total_displacement = self.step(1.0);
            if self.is_relaxed(&total_displacement) {
                break;
            }
        }
    }

    fn step(&mut self, time: f32) {
        if self.edges.len() < self.edges_goal.len() {
            self.construct_next();
        }
        self.step(time);
        self.nodes.assert_no_nans();
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
