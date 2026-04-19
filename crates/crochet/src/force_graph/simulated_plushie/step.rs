use glam::Vec3;

use crate::force_graph::{
    centroid_stuffing::centroid_stuffing, link_force::link_forces, single_loop::single_loop_forces,
};

pub struct SimulationParams {
    pub force_multiplier: f32,
    pub single_loop_force: f32,
}

impl super::SimulatedPlushie {
    pub fn step(&mut self, params: &SimulationParams) {
        self.compute_forces(params);

        for (node, displacement) in self.nodes.iter_mut().zip(&self.displacement) {
            if !node.rooted {
                node.position += displacement * params.force_multiplier;
            }
        }
    }

    fn compute_forces(&mut self, params: &SimulationParams) {
        while self.displacement.len() > self.nodes.len() {
            self.displacement.pop();
        }
        for d in &mut self.displacement {
            *d = Vec3::ZERO;
        }
        if self.displacement.len() < self.nodes.len() {
            self.displacement.append(&mut vec![
                Vec3::ZERO;
                self.nodes.len() - self.displacement.len()
            ]);
        }

        link_forces(
            &self.nodes,
            &self.edges,
            self.hook_size,
            &mut self.displacement,
            &mut self.tensions,
        );

        for part in &mut self.parts {
            while part.centroids.len() < part.centroids_wanted {
                part.centroids.push(Vec3::ZERO);
            }
            while part.centroids.len() > part.centroids_wanted {
                part.centroids.pop();
            }

            centroid_stuffing(
                &self.nodes[part.start..part.end],
                &mut part.centroids,
                self.hook_size,
                &mut self.displacement[part.start..part.end],
            );
        }

        // shouldn't it take hook_size into account?
        single_loop_forces(
            &self.nodes,
            params.single_loop_force,
            &mut self.displacement,
        );
    }
}
