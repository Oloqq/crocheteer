use crate::common::{Point, V};

pub fn centroid_stuffing(
    points: &Vec<Point>,
    centroids: &mut Vec<Point>,
    centroid_force: f32,
    displacement: &mut Vec<V>,
) {
    push(points, centroids, centroid_force, displacement);
    recalculate_centroids(points, centroids, displacement);
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

fn recalculate_centroids(
    points: &Vec<Point>,
    centroids: &mut Vec<Point>,
    displacement: &mut Vec<V>,
) {
}

fn weight(dist: f32) -> f32 {
    // https://www.desmos.com/calculator: e^{\frac{-\left(\ln\left(x\right)-b\right)^{2}}{c^{2}}}
    let c: f32 = 1.4;
    let b = 0.4;
    let x = dist;

    let numerator = -(x.ln() - b).powi(2);
    let denominator = c.powi(2);

    f32::exp(numerator / denominator)
}

fn push_away(point: &Point, repelant: &Point) -> V {
    let diff = point - repelant;
    diff.normalize() * (1.0 / (diff.magnitude() + 0.5))
}
