use std::f32::consts::PI;

use crate::common::*;
use kmeans::*;

pub fn do_clustering(num_clusters: usize, points: &Vec<Point>) -> (Vec<usize>, Vec<Point>) {
    let sample_count = points.len();
    let sample_dims = 3;
    let k = num_clusters;
    let max_iter = 100;

    let samples: Vec<f32> = points
        .iter()
        .flat_map(|p| p.coords.iter().cloned())
        .collect();

    let kmean: KMeans<_, 8, _> = KMeans::new(samples, sample_count, sample_dims, EuclideanDistance);
    let result = kmean.kmeans_lloyd(
        k,
        max_iter,
        KMeans::init_kmeanplusplus,
        &KMeansConfig::default(),
    );

    let centroids: Vec<Point> = result
        .centroids
        .to_vec()
        .iter()
        .array_chunks::<3>()
        .map(|[x, y, z]| Point::new(*x, *y, *z))
        .collect();

    // maybe these could be included in the type system?
    assert_eq!(result.assignments.len(), points.len());
    assert_eq!(centroids.len(), num_clusters);
    (result.assignments, centroids)
}

pub fn select_seeds(
    points: &Vec<Point>,
    assignments: &Vec<usize>,
    centroids: &Vec<Point>,
) -> Vec<usize> {
    assert_eq!(points.len(), assignments.len());

    type BestId = usize;
    type Distance = f32;
    let mut closest_to_centroid: Vec<(BestId, Distance)> =
        vec![(0, Distance::MAX); centroids.len()];
    for (i, (point, cluster)) in points.iter().zip(assignments).enumerate() {
        let center = centroids[*cluster];
        let distance = point.coords.metric_distance(&center.coords);
        if distance < closest_to_centroid[*cluster].1 {
            closest_to_centroid[*cluster] = (i, distance);
        }
    }

    let seeds: Vec<usize> = closest_to_centroid.iter().map(|(i, _dist)| *i).collect();
    assert_eq!(seeds.len(), centroids.len());
    seeds
}

fn get_inliers(
    cloud: &Vec<Point>,
    _connectivity: (),
    threshold: f32,
    seed: usize,
    normal_offset: V,
) -> Vec<usize> {
    let d = normal_offset.dot(&cloud[seed].coords);
    let close_to_plane: Vec<usize> = cloud
        .iter()
        .enumerate()
        .filter_map(|(i, p)| (normal_offset.dot(&p.coords) - d <= threshold).then_some(i))
        .collect();
    let connected = close_to_plane;
    connected
}

fn orient_cost(normals: &Vec<V>, inliers: &Vec<usize>, normal_offset: V) -> f32 {
    inliers
        .iter()
        .map(|i| normal_offset.dot(&normals[*i]).abs())
        .sum::<f32>()
        / inliers.len() as f32
}

pub struct Orientation(pub f32, pub f32);

fn orient_plane(
    cloud: &Vec<Point>,
    normals: &Vec<V>,
    connectivity: (),
    seed: usize,
) -> (Orientation, Vec<usize>) {
    const ANGULAR_INTERVAL: f32 = PI / 6.0;
    const THETA_STEPS: usize = 11;
    const PHI_STEPS: usize = 4;
    const GLOBAL_THRESHOLD: f32 = 0.00001;

    let mut candidates: Vec<(Orientation, f32)> = Vec::with_capacity(THETA_STEPS * PHI_STEPS);
    let mut debug_inliers: Vec<Vec<usize>> = Vec::with_capacity(candidates.capacity());
    for theta in (0..=THETA_STEPS).map(|t| t as f32 * ANGULAR_INTERVAL) {
        for phi in (0..=PHI_STEPS).map(|p| p as f32 * ANGULAR_INTERVAL) {
            let normal_orient = V::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos());
            let inliers = get_inliers(cloud, connectivity, GLOBAL_THRESHOLD, seed, normal_orient);
            let cost = orient_cost(normals, &inliers, normal_orient);
            candidates.push((Orientation(theta, phi), cost));
            debug_inliers.push(inliers);
        }
    }

    let (index, best_orientation) = candidates
        .into_iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.1.total_cmp(&b.1))
        .and_then(|(i, candidate)| Some((i, candidate.0)))
        .unwrap();

    (best_orientation, debug_inliers.swap_remove(index))
}

pub fn orient_planes(
    cloud: &Vec<Point>,
    normals: &Vec<V>,
    connectivity: (),
    seeds: &Vec<usize>,
) -> Vec<(Orientation, Vec<usize>)> {
    seeds
        .iter()
        .map(|seed| orient_plane(cloud, normals, connectivity, *seed))
        .collect()
}

// pub fn detect_initial_cross_sections() {}
