use serde_derive::Serialize;

use self::{per_round_stuffing::RoundsInfo, points::Points};

use super::common::*;

mod animation;
mod centroid_stuffing;
pub mod config;
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
    floor: bool,
}

impl Plushie {
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
