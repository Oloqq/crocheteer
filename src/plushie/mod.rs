use self::{params::Params, points::Points};
use super::common::*;

use serde_derive::Serialize;

mod animation;
mod construction;
mod conversions;
pub mod examples;
pub mod params;
mod points;

#[derive(Clone, Serialize)]
pub enum Stuffing {
    None,
    Centroids,
}

#[derive(Clone, Serialize)]
pub struct Plushie {
    points: Points,
    edges: Vec<Vec<usize>>,
    pub params: Params,

    pub centroids: Vec<Point>,
    pub stuffing: Stuffing,
}

impl Plushie {
    pub fn new(
        points: Points,
        edges: Vec<Vec<usize>>,
        params: Params,
        centroids: Vec<Point>,
    ) -> Self {
        Self {
            stuffing: Stuffing::Centroids,
            points,
            edges,
            params,
            centroids,
        }
    }

    fn is_relaxed(&self, displacement: &Vec<V>) -> bool {
        // TODO: elbow method
        let tension: f32 = displacement.iter().map(|v| v.magnitude()).sum();
        tension <= self.params.acceptable_tension
    }

    pub fn animate(&mut self) {
        for _ in 0..self.params.max_relaxing_iterations {
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
            let centroid2points = animation::centroid::map(&self.points.as_vec(), &self.centroids);
            animation::centroid::recalculate_centroids(
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
