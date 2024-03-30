use super::Plushie;
use crate::common::*;

use std::mem;

impl Plushie {
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
        mem::swap(&mut self.edges[index], &mut self.edges_goal[index]);
        let node = self.new_node_position(&self.edges[index]);
        self.nodes.points.push(node);
        self.displacement.push(V::zeros());
    }

    pub fn handle_adding_new_nodes(&mut self) {
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
