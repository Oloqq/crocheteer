use na::distance;
use serde_derive::Serialize;

use crate::{
    common::{CheckNan, Point, V},
    plushie::params::CentroidParams,
    sanity,
};

#[derive(Clone, Serialize)]
pub struct Centroids {
    pub points: Vec<Point>,
}

impl Centroids {
    pub fn new(number: usize, approximate_height: f32) -> Self {
        let margin = 1.0;
        let ceiling = approximate_height - margin;
        let floor = margin;
        let spacing = (ceiling - floor) / number as f32;
        let centroids = (0..number)
            .map(|i| Point::new(0.0, floor + spacing * i as f32, 0.0))
            .collect();
        Self { points: centroids }
    }

    fn adjust_centroid_number(&mut self, params: &CentroidParams, skin: &[Point]) {
        let has_too_little = self.points.len() < params.number;
        let is_ready_to_add = skin.len() >= params.min_nodes_per_centroid * (self.points.len());

        if has_too_little {
            if is_ready_to_add {
                let new = if self.points.len() >= 2 {
                    let c0 = self.points[0];
                    let c1 = self.points[1];
                    Point::from((c0.coords + c1.coords) / 2.0)
                } else {
                    skin.last()
                        .expect("Some nodes to exist before centroid logic is running")
                        .clone()
                };
                self.points.push(new);
            }
        } else {
            while self.points.len() > params.number {
                self.points.pop();
            }
        }
        sanity!(self.points.assert_no_nan("after adjusting number"));
    }

    pub fn stuff(&mut self, params: &CentroidParams, skin: &[Point], displacement: &mut [V]) {
        if skin.len() < 5 {
            return;
        }

        self.adjust_centroid_number(params, skin);

        if !self.points.is_empty() {
            let centroid2points = push_and_map(skin, &self.points, params.force, displacement);
            recalculate_centroids(skin, &mut self.points, centroid2points);
        }
    }
}

fn push_and_map(
    nodes: &[Point],
    centroids: &Vec<Point>,
    centroid_force: f32,
    displacement: &mut [V],
) -> Vec<Vec<usize>> {
    assert_eq!(nodes.len(), displacement.len());
    let mut centroid2points = vec![vec![]; centroids.len()];
    for (i_p, point) in nodes.iter().enumerate() {
        let mut closest_i = 0;
        let mut closest = distance(point, &centroids[closest_i]);
        for (i_c, centroid) in centroids.iter().enumerate() {
            if distance(point, centroid) < closest {
                closest = distance(point, centroid);
                closest_i = i_c;
            }
            displacement[i_p] += push_away(point, centroid) * centroid_force;
        }
        centroid2points[closest_i].push(i_p);
    }
    centroid2points
}

fn recalculate_centroids(
    nodes: &[Point],
    centroids: &mut Vec<Point>,
    centroid2points: Vec<Vec<usize>>,
) {
    centroids.iter_mut().enumerate().for_each(|(i, centroid)| {
        let mut new_pos: V = V::zeros();
        let mut weight_sum = 0.0;
        if centroid2points[i].len() == 0 {
            log::warn!("No points assigned to centroid");
            return;
        }
        for point_index in &centroid2points[i] {
            let point = nodes[*point_index];
            let w = weight(distance(&centroid, &point));
            new_pos += point.coords * w;
            weight_sum += w;
        }
        assert!(weight_sum != 0.0, "About to divide by 0");
        let new_pos: Point = Point::from(new_pos / weight_sum);
        *centroid = new_pos
    });
    sanity!(centroids.assert_no_nan("after recalculating centroids"));
}

fn weight(dist: f32) -> f32 {
    // https://www.desmos.com/calculator: e^{\frac{-\left(\ln\left(x\right)-b\right)^{2}}{c^{2}}}
    let b: f32 = 1.0;
    let c: f32 = 1.4;
    let x = dist;

    let numerator = -(x.ln() - b).powi(2);
    let denominator = c.powi(2);

    f32::exp(numerator / denominator) * 1.0
}

fn push_away(point: &Point, repelant: &Point) -> V {
    let diff = point - repelant;
    if diff.magnitude() != 0.0 {
        let factor = 1.0;
        let res = diff.normalize() * (factor / diff.magnitude_squared()).min(1.0);
        sanity!(res.assert_no_nan("NaN while pushing"));
        res
    } else {
        V::zeros()
    }
}
