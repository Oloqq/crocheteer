use std::time::Instant;

use super::{perf, Plushie};
use crate::{common::*, sanity};

impl Plushie {
    pub fn step(&mut self, time: f32) -> V {
        let mut perf = self
            .params
            .track_performance
            .then_some(perf::Iteration::zeros());

        self.displacement.fill(V::zeros());
        self.add_link_forces();
        self.add_stuffing_force(&mut perf);
        self.add_gravity();

        self.last_total_displacement =
            self.nodes
                .apply_forces(&mut self.displacement, time, &self.params);

        perf.map(|p| self.perf.push(p));

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

    fn add_stuffing_force(&mut self, perf: &mut Option<perf::Iteration>) {
        let start = Instant::now();
        self.centroids
            .stuff(&self.params.centroids, &self.nodes, &mut self.displacement);
        perf.as_mut().map(|p| p.stuffing = start.elapsed());
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
                        perf,
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
