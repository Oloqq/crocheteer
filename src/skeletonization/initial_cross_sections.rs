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

// fn get_inliers()

pub fn orient_planes(seeds: &Vec<usize>) -> Vec<(f32, f32)> {
    const ANGULAR_INTERVAL: f32 = PI / 6.0;
    // assert_eq!(result.len(), seeds.len());
    // result

    seeds.iter().map(|_| (0.0, 0.0)).collect()
}

// pub fn detect_initial_cross_sections() {}
