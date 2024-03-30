use super::Plushie;
use crate::common::*;
use std::time::{SystemTime, UNIX_EPOCH};

impl Plushie {
    pub fn step(&mut self, time: f32) -> V {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        log::trace!("Nodes: {:?}", self.nodes);

        self.displacement.fill(V::zeros());
        self.add_link_forces();
        self.add_stuffing_force();
        self.add_gravity();

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let elapsed = end - start;
        log::trace!("Elapsed: {}", elapsed.as_nanos());

        let total = self
            .nodes
            .apply_forces(&mut self.displacement, time, &self.params);

        total
    }

    fn add_link_forces(&mut self) {
        for (i, point) in self.nodes.points.iter().enumerate() {
            for neibi in &self.edges[i] {
                if *neibi >= self.nodes.points.len() {
                    continue;
                }
                let neib = &self.nodes[*neibi];
                let diff: V = attract(point, neib, self.params.desired_stitch_distance);
                self.displacement[i] += diff;
                self.displacement[*neibi] -= diff;
            }
        }
        self.displacement.sanity_assert_no_nan("link forces");
    }

    fn add_stuffing_force(&mut self) {
        self.centroids
            .stuff(&self.params.centroids, &self.nodes, &mut self.displacement);
        self.displacement.sanity_assert_no_nan("stuffing");
    }

    fn add_gravity(&mut self) {
        for (i, _point) in self.nodes.points.iter().enumerate() {
            self.displacement[i].y -= self.params.gravity;
        }
        self.displacement.sanity_assert_no_nan("gravity");
    }
}

fn attract(this: &Point, other: &Point, desired_distance: f32) -> V {
    let diff = this - other;
    let x = diff.magnitude();
    let d = desired_distance;

    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    let res = -diff.normalize() * fx;
    res.sanity_assert_no_nan(format!("attract {this:?} to {other:?}").as_str());
    res
}
