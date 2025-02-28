use std::mem;

use super::Plushie;
use crate::{common::*, plushie::params::OneByOneParams};

impl Plushie {
    fn new_node_position(&self, index: usize, based_on: &Vec<usize>) -> Point {
        let dsd = self.params.desired_stitch_distance;

        if based_on.len() == 0 {
            panic!()
            // let mark_of_this = self.mark_to_node.iter().find(|x| *x.1 == index);
            // let mut ret = Point::from(V::zeros());
            // if let Some((name, _)) = mark_of_this {
            //     if let Some(node_param) = self.params.nodes.get(name) {
            //         if let Some(x) = node_param.lock_x {
            //             ret.x = x;
            //         }
            //         if let Some(y) = node_param.lock_y {
            //             ret.y = y;
            //         }
            //         if let Some(z) = node_param.lock_z {
            //             ret.z = z;
            //         }
            //     }
            // }
            // ret
        } else if self.nodes.points.len() == 1 {
            Point::from(self.nodes.points[0] + V::new(dsd, dsd * 0.1, 0.0))
        } else if self.nodes.points.len() == 2 {
            Point::from(self.nodes.points[0] + V::new(0.0, dsd * 0.2, dsd))
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
        let node = self.new_node_position(index, &self.edges[index]);
        self.nodes.points.push(node);
        self.displacement.push(V::zeros());
    }

    pub fn handle_adding_new_nodes(&mut self, params: OneByOneParams, time: f32) {
        assert!(self.nodes.len() > 0, "Nodes don't even have a root?");
        let small_displacement = || -> bool {
            let last_index = self.nodes.len() - 1;
            let last_displacement = self.displacement[last_index];
            last_displacement.magnitude() < params.acceptable_displacement_for_expanding
        };
        let force_construction = self.force_node_construction_timer <= 0.0;

        if self.edges.len() < self.edges_goal.len() && (small_displacement() || force_construction)
        {
            self.construct_next_node();
            self.force_node_construction_timer = params.force_expansion_after_time;
        } else {
            self.force_node_construction_timer -= time;
        }
    }
}
