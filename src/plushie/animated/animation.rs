use super::Plushie;
use crate::{common::*, sanity};
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

        self.last_total_displacement =
            self.nodes
                .apply_forces(&mut self.displacement, time, &self.params);

        self.last_total_displacement
    }

    fn add_link_forces(&mut self) {
        for (i, point) in self.nodes.points.iter().enumerate() {
            for neibi in &self.edges[i] {
                if *neibi >= self.nodes.points.len() {
                    continue; // assert that it doesn't happen?
                }
                let neib = &self.nodes[*neibi];
                let diff: V = attract(point, neib, self.params.desired_stitch_distance);
                self.displacement[i] += diff;
                self.displacement[*neibi] -= diff;
            }
        }
        sanity!(self.displacement.assert_no_nan("link forces"));
    }

    fn add_stuffing_force(&mut self) {
        self.centroids
            .stuff(&self.params.centroids, &self.nodes, &mut self.displacement);
        sanity!(self.displacement.assert_no_nan("stuffing"));

        if self.params.skelet_stuffing.enable {
            if self.params.skelet_stuffing.autoskelet {
                if self.params.skelet_stuffing.interval_left == 0 {
                    self.params.skelet_stuffing.interval_left =
                        self.params.skelet_stuffing.interval;
                    self.params.skelet_stuffing.bones = crate::skeletonization::get_skelet(
                        &self,
                        self.params.skelet_stuffing.centroid_number,
                        self.params.skelet_stuffing.must_include_points,
                        self.params.skelet_stuffing.allowed_overlap,
                    );
                } else {
                    self.params.skelet_stuffing.interval_left -= 1;
                }
            }
            self.params.centroids.number = self.params.skelet_stuffing.bones.len();
            self.centroids.points = self.params.skelet_stuffing.bones.clone();
        }
    }

    fn add_gravity(&mut self) {
        for (i, _point) in self.nodes.points.iter().enumerate() {
            self.displacement[i].y -= self.params.gravity;
        }
        sanity!(self.displacement.assert_no_nan("gravity"));
    }
}

fn attract(this: &Point, other: &Point, desired_distance: f32) -> V {
    let diff = this - other;
    let x = diff.magnitude();
    let d = desired_distance;

    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    let res = -diff.normalize() * fx;
    sanity!(res.assert_no_nan(format!("attract {this:?} to {other:?}").as_str()));
    res
}
