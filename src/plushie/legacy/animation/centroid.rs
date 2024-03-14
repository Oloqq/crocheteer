use crate::{
    common::{Point, V},
    plushie::legacy::nodes::Nodes,
};

use na::distance;
use serde_derive::Serialize;

#[derive(Clone, Serialize)]
pub struct Centroids {
    // keep in mind that this field name is important in the frontend in current communication
    centroids: Vec<Point>,
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
        Self { centroids }
    }

    pub fn set_centroid_num(&mut self, num: usize, nodes: &Nodes) {
        // FIXME adding to many centroids at once glitches the plushie irrecoverably
        if self.centroids.len() == num {
            return;
        }

        while self.centroids.len() > num {
            self.centroids.pop();
        }

        while self.centroids.len() < num {
            self.centroids.push(Point::new(0.0, 1.0, 0.0));
            let centroid2points = map(&nodes.as_vec(), &self.centroids);
            recalculate_centroids(&nodes.as_vec(), &mut self.centroids, centroid2points);
        }
    }

    pub fn stuff(&mut self, force: f32, nodes: &Nodes, displacement: &mut Vec<V>) {
        // TODO handle changing number here
        if self.centroids.is_empty() {
            return;
        }

        let centroid2points = push_and_map(nodes.as_vec(), &self.centroids, force, displacement);
        recalculate_centroids(nodes.as_vec(), &mut self.centroids, centroid2points);
    }
}

fn push_and_map(
    nodes: &Vec<Point>,
    centroids: &Vec<Point>,
    centroid_force: f32,
    displacement: &mut Vec<V>,
) -> Vec<Vec<usize>> {
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

fn map(nodes: &Vec<Point>, centroids: &Vec<Point>) -> Vec<Vec<usize>> {
    assert!(centroids.len() > 0);
    let mut centroid2points = vec![vec![]; centroids.len()];
    for (i_p, point) in nodes.iter().enumerate() {
        let mut closest_i = 0;
        let mut closest = distance(point, &centroids[closest_i]);
        for (i_c, centroid) in centroids.iter().enumerate() {
            if distance(point, centroid) < closest {
                closest = distance(point, centroid);
                closest_i = i_c;
            }
        }
        centroid2points[closest_i].push(i_p);
    }
    centroid2points
}

fn recalculate_centroids(
    nodes: &Vec<Point>,
    centroids: &mut Vec<Point>,
    centroid2points: Vec<Vec<usize>>,
) {
    centroids.iter_mut().enumerate().for_each(|(i, centroid)| {
        let mut new_pos: V = V::zeros();
        let mut weight_sum = 0.0;
        for point_index in &centroid2points[i] {
            let point = nodes[*point_index];
            let w = weight(distance(&centroid, &point));
            new_pos += point.coords * w;
            weight_sum += w;
        }
        let new_pos: Point = Point::from(new_pos / weight_sum);
        *centroid = new_pos
    })
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
    diff.normalize() * (1.0 / (diff.magnitude() + 0.5))
}
