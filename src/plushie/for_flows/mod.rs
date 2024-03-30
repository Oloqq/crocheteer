mod animation;
mod centroid;
mod construction;
mod nodes;

use self::{centroid::Centroids, nodes::Nodes};
use super::{Params, PlushieTrait};
use crate::common::*;
use serde_derive::Serialize;

use std::mem::swap;

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
        tension <= self.params.acceptable_tension
    }

    fn new_node_position(&self, based_on: &Vec<usize>) -> Point {
        let dsd = self.params.desired_stitch_distance;

        if based_on.len() == 0 {
            panic!("Node should be attached to something");
        } else if self.nodes.points.len() == 1 {
            Point::from(self.nodes.points[0] + V::new(0.0, dsd * 0.1, dsd))
        } else if self.nodes.points.len() == 2 {
            Point::from(self.nodes.points[0] + V::new(dsd, dsd * 0.2, 0.0))
        } else if based_on.len() == 1 {
            let base = self.nodes.points[based_on[0]];
            let coords = base.coords + V::new(0.0, dsd, 0.0);
            Point::from(coords)
        } else {
            let mut avg = V::zeros();
            for base in based_on {
                let point = self.nodes.points[*base];
                avg += point.coords;
            }
            avg /= based_on.len() as f32;
            avg += V::new(0.0, dsd, 0.0);
            Point::from(avg)
        }
    }

    fn construct_next_node(&mut self) {
        let index = self.edges.len();
        self.edges.push(vec![]);
        swap(&mut self.edges[index], &mut self.edges_goal[index]);
        let node = self.new_node_position(&self.edges[index]);
        self.nodes.points.push(node);
        self.displacement.push(V::zeros());
    }

    fn handle_adding_new_nodes(&mut self) {
        assert!(self.nodes.len() > 0, "Nodes don't even have a root?");
        let small_displacement = || -> bool {
            let last_index = self.nodes.len() - 1;
            let last_displacement = self.displacement[last_index];
            last_displacement.magnitude() < 0.03
        };
        let force = self.force_node_construction_timer <= 0;

        if self.edges.len() < self.edges_goal.len() && (small_displacement() || force) {
            self.construct_next_node();
            self.force_node_construction_timer = 100;
        } else {
            self.force_node_construction_timer -= 1;
        }
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
