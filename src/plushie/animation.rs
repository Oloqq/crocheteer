pub mod centroid;

use std::time::{SystemTime, UNIX_EPOCH};

use super::{Plushie, Stuffing};
use crate::common::*;

use self::centroid::centroid_stuffing;

impl Plushie {
    pub fn step(&mut self, time: f32) -> Vec<V> {
        let mut displacement: Vec<V> = vec![V::zeros(); self.points.len()];
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        self.add_link_forces(&mut displacement);
        self.add_stuffing_force(&mut displacement);
        self.add_gravity(&mut displacement);

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let elapsed = end - start;
        log::trace!("Elapsed: {}", elapsed.as_nanos());

        let _total = self.points.apply_forces(&displacement, time);

        displacement
    }

    fn add_link_forces(&self, displacement: &mut Vec<V>) {
        for (i, point) in self.points.all() {
            for neibi in &self.edges[i] {
                let neib = &self.points[*neibi];
                let diff: V = attract(point, neib, self.params.desired_stitch_distance);
                displacement[i] += diff;
                displacement[*neibi] -= diff;
            }
        }
    }

    fn add_stuffing_force(&mut self, displacement: &mut Vec<V>) {
        match &self.stuffing {
            Stuffing::None => (),
            Stuffing::Centroids => centroid_stuffing(
                &self.points.as_vec(),
                &mut self.centroids,
                self.centroid_force,
                displacement,
            ),
        }
    }

    fn add_gravity(&self, displacement: &mut Vec<V>) {
        for (i, _point) in self.points.all() {
            displacement[i].y -= self.params.gravity;
        }
    }
}

fn attract(this: &Point, other: &Point, desired_distance: f32) -> V {
    let diff = this - other;
    let x = diff.magnitude();
    let d = desired_distance;

    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    -diff.normalize() * fx
}
