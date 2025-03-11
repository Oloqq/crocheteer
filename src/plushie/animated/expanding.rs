use std::mem;

use super::Plushie;
use crate::{common::*, plushie::params::OneByOneParams};

impl Plushie {
    fn new_node_position(&self, based_on: &Vec<usize>) -> Point {
        let dsd = self.params.desired_stitch_distance;

        if based_on.len() == 0 {
            unreachable!()
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

    fn mr_starter_positions(&mut self, index: usize) -> [Point; 3] {
        let mark_of_this = self.mark_to_node.iter().find(|x| *x.1 == index);
        let mut ring_root = V::zeros();
        if let Some((name, _)) = mark_of_this {
            if let Some(node_param) = self.params.limbs.get(name) {
                if let Some(x) = node_param.lock_x {
                    ring_root.x = x;
                }
                if let Some(y) = node_param.lock_y {
                    ring_root.y = y;
                }
                if let Some(z) = node_param.lock_z {
                    ring_root.z = z;
                }
            }
        }

        let dsd = self.params.desired_stitch_distance;
        [
            Point::from(ring_root),
            Point::from(ring_root + V::new(dsd, dsd * 0.1, 0.0)),
            Point::from(ring_root + V::new(0.0, dsd * 0.2, dsd)),
        ]
    }

    fn construct_node(&mut self, index: usize, position: Point) {
        self.edges.push(vec![]);
        mem::swap(&mut self.edges[index], &mut self.edges_goal[index]);
        self.nodes.points.push(position);
        self.displacement.push(V::zeros());
    }

    fn construct_node_or_3(&mut self) {
        let index = self.edges.len();
        let basis = &self.edges_goal[index];
        if basis.len() == 0 {
            let positions = self.mr_starter_positions(index);
            self.construct_node(index, positions[0]);
            self.construct_node(index + 1, positions[1]);
            self.construct_node(index + 2, positions[2]);
        } else {
            self.construct_node(index, self.new_node_position(basis));
        }
    }

    pub fn handle_adding_new_nodes(&mut self, params: OneByOneParams, time: f32) {
        let small_displacement = || -> bool {
            assert!(self.nodes.len() > 0);
            let last_index = self.nodes.len() - 1;
            let last_displacement = self.displacement[last_index];
            last_displacement.magnitude() < params.acceptable_displacement_for_expanding
        };
        let construction_unfinished = self.edges.len() < self.edges_goal.len();
        let force_construction = self.force_node_construction_timer <= 0.0;

        if construction_unfinished
            && (self.nodes.len() == 0 || small_displacement() || force_construction)
        {
            self.construct_node_or_3();
            self.force_node_construction_timer = params.force_expansion_after_time;
        } else {
            self.force_node_construction_timer -= time;
        }
    }
}
