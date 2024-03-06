use std::time::{SystemTime, UNIX_EPOCH};

use serde_derive::Serialize;

use self::centroid_stuffing::centroid_stuffing;
use self::per_round_stuffing::{per_round_stuffing, RoundsInfo};
use self::points::Points;

use super::common::*;

mod centroid_stuffing;
mod construction;
mod conversions;
pub mod examples;
mod per_round_stuffing;
mod points;

#[derive(Clone, Serialize)]
pub enum Stuffing {
    None,
    PerRound,
    Centroids,
}

#[derive(Clone, Serialize)]
pub struct Plushie {
    points: Points,
    rounds: RoundsInfo,
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
        for (i, point) in self.points.all() {
            for neibi in &self.edges[i] {
                let neib = &self.points[*neibi];
                let diff: V = attract(point, neib, self.desired_stitch_distance);
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
                &self.points.as_vec(),
                self.desired_stitch_distance,
                displacement,
            ),
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
            displacement[i].y -= self.gravity;
        }
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

        let _total = self.points.apply_forces(&displacement, time);

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
        // FIXME adding to many centroids at once glitches the plushie irrecoverably
        if self.centroids.len() == num {
            return;
        }

        while self.centroids.len() > num {
            self.centroids.pop();
        }

        while self.centroids.len() < num {
            self.centroids.push(Point::new(0.0, 1.0, 0.0));
            let centroid2points =
                self::centroid_stuffing::map(&self.points.as_vec(), &self.centroids);
            self::centroid_stuffing::recalculate_centroids(
                &self.points.as_vec(),
                &mut self.centroids,
                centroid2points,
            );
        }
    }

    pub fn get_points_vec(&self) -> &Vec<Point> {
        self.points.as_vec()
    }

    pub fn set_point_position(&mut self, i: usize, pos: Point) {
        if i >= self.points.len() {
            // using websockets, this could theoretically happen with reloading and some network delays
            panic!("Point index greater than vector size");
        }
        self.points[i] = pos;
    }
}

fn attract(this: &Point, other: &Point, desired_distance: f32) -> V {
    let diff = this - other;
    let x = diff.magnitude();
    let d = desired_distance;

    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    -diff.normalize() * fx
}
