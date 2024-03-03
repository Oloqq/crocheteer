use std::time::{SystemTime, UNIX_EPOCH};

use serde_derive::Serialize;

use self::centroid_stuffing::centroid_stuffing;
use self::per_round_stuffing::{per_round_stuffing, Rounds};

use super::common::*;

mod centroid_stuffing;
mod construction;
mod conversions;
pub mod examples;
mod per_round_stuffing;

#[derive(Clone, Serialize)]
pub enum Stuffing {
    None,
    PerRound,
    Centroids,
}

/* Things to consider next
- working the back or the front loop only (BLO/FLO)
- switching "working position" like working the front loop, then going back and working the back loop
- chains
- attaching a chain to a set point
    - how to calculate the anchors now available?
- non-uniform stuffing
- creations that are not closed at the top (like the vase)
- handling heavily folded shapes
*/

#[derive(Clone, Serialize)]
pub struct Plushie {
    fixed_num: usize, // treat first N elements of `points` as fixed
    rounds: Rounds,
    pub points: Vec<Point>,
    pub centroids: Vec<Point>,
    pub centroid_force: f32,
    edges: Vec<Vec<usize>>,
    pub stuffing: Stuffing,
    desired_stitch_distance: f32,
    pub gravity: f32,
    acceptable_tension: f32,
    max_relaxing_iterations: usize,
}

impl Plushie {
    fn add_link_forces(&self, displacement: &mut Vec<V>) {
        for i in 0..self.points.len() {
            let this = self.points[i];
            for neibi in &self.edges[i] {
                let neib = self.points[*neibi];
                let diff: V = attract(this, neib, self.desired_stitch_distance);
                displacement[i] += diff;
                displacement[*neibi] -= diff;
            }
        }
    }

    fn add_stuffing_force(&mut self, displacement: &mut Vec<V>) {
        match &self.stuffing {
            Stuffing::None => (),
            Stuffing::PerRound => per_round_stuffing(
                &mut self.rounds,
                &self.points,
                self.desired_stitch_distance,
                displacement,
            ),
            Stuffing::Centroids => centroid_stuffing(
                &self.points,
                &mut self.centroids,
                self.centroid_force,
                displacement,
            ),
        }
    }

    fn add_gravity(&self, displacement: &mut Vec<V>) {
        for i in 0..self.points.len() {
            displacement[i].y -= self.gravity;
        }
    }

    fn apply_forces(&mut self, displacement: &Vec<V>, time: f32) {
        let mut total = V::zeros();
        for i in self.fixed_num..self.points.len() {
            total += displacement[i];
            self.points[i] += displacement[i] * time;
            self.points[i].y = self.points[i].y.max(0.0);
        }
        self.points[1].y += displacement[1].y.clamp(-1.0, 1.0) * time;
    }

    pub fn step(&mut self, time: f32) -> Vec<V> {
        let mut displacement: Vec<V> = vec![V::zeros(); self.points.len()];
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        self.add_link_forces(&mut displacement);
        self.add_stuffing_force(&mut displacement);
        self.add_gravity(&mut displacement);

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let elapsed = end - start;
        log::trace!("Elapsed: {}", elapsed.as_nanos());

        self.apply_forces(&displacement, time);

        displacement
    }

    fn is_relaxed(&self, displacement: &Vec<V>) -> bool {
        // TODO: elbow method
        let tension: f32 = displacement.iter().map(|v| v.magnitude()).sum();
        tension <= self.acceptable_tension
    }

    pub fn animate(&mut self) {
        for _ in 0..self.max_relaxing_iterations {
            let displacement = self.step(1.0);
            if self.is_relaxed(&displacement) {
                break;
            }
        }
    }

    pub fn set_centroid_num(&mut self, num: usize) {
        if self.centroids.len() == num {
            return;
        }

        while self.centroids.len() > num {
            self.centroids.pop();
        }

        while self.centroids.len() < num {
            self.centroids.push(Point::new(0.0, 1.0, 0.0));
            let centroid2points = self::centroid_stuffing::map(&self.points, &self.centroids);
            self::centroid_stuffing::recalculate_centroids(
                &self.points,
                &mut self.centroids,
                centroid2points,
            );
        }
        println!("cent {:?}", self.centroids);
    }
}

fn attract(this: Point, other: Point, desired_distance: f32) -> V {
    let diff = this - other;
    let x = diff.magnitude();
    let d = desired_distance;

    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    -diff.normalize() * fx
}
