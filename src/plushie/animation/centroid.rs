use na::distance;

use crate::common::{Point, V};

pub fn centroid_stuffing(
    points: &Vec<Point>,
    centroids: &mut Vec<Point>,
    centroid_force: f32,
    displacement: &mut Vec<V>,
) {
    let centroid2points = push_and_map(points, centroids, centroid_force, displacement);
    recalculate_centroids(points, centroids, centroid2points);
    // println!("{centroids:?}");
}

fn push_and_map(
    points: &Vec<Point>,
    centroids: &Vec<Point>,
    centroid_force: f32,
    displacement: &mut Vec<V>,
) -> Vec<Vec<usize>> {
    assert!(centroids.len() > 0);
    let mut centroid2points = vec![vec![]; centroids.len()];
    for (i_p, point) in points.iter().enumerate() {
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

pub fn map(points: &Vec<Point>, centroids: &Vec<Point>) -> Vec<Vec<usize>> {
    assert!(centroids.len() > 0);
    let mut centroid2points = vec![vec![]; centroids.len()];
    for (i_p, point) in points.iter().enumerate() {
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

pub fn recalculate_centroids(
    points: &Vec<Point>,
    centroids: &mut Vec<Point>,
    centroid2points: Vec<Vec<usize>>,
) {
    centroids.iter_mut().enumerate().for_each(|(i, centroid)| {
        let mut new_pos: V = V::zeros();
        let mut weight_sum = 0.0;
        for point_index in &centroid2points[i] {
            let point = points[*point_index];
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
