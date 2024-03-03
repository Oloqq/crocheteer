use na::distance;

use crate::common::{Point, V};

pub fn centroid_stuffing(
    points: &Vec<Point>,
    centroids: &mut Vec<Point>,
    centroid_force: f32,
    displacement: &mut Vec<V>,
) {
    recalculate_centroids(points, centroids);
    // println!("{centroids:?}");
    push(points, centroids, centroid_force, displacement);
}

fn push(
    points: &Vec<Point>,
    centroids: &Vec<Point>,
    centroid_force: f32,
    displacement: &mut Vec<V>,
) {
    for (i, point) in points.iter().enumerate() {
        for centroid in centroids {
            displacement[i] += push_away(point, centroid) * centroid_force;
        }
    }
}

fn recalculate_centroids(points: &Vec<Point>, centroids: &mut Vec<Point>) {
    centroids.iter_mut().for_each(|centroid| {
        let mut new_pos: V = V::zeros();
        for point in points {
            new_pos += point.coords * weight(distance(&centroid, point));
        }
        let new_pos: Point = Point::from(new_pos / points.len() as f32);
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
