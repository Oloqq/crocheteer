use super::utils::{orient_planes, Orientation};
use crate::common::*;
use kmeans::*;

#[allow(unused)]
pub fn max_dists(points: &Vec<Point>, edges: &Vec<Vec<usize>>) -> Vec<f32> {
    edges
        .iter()
        .enumerate()
        .map(|(from, targets)| {
            let source = points[from];
            let targets = targets.iter().map(|ind| points[*ind]);
            targets
                .map(|t| source.coords.metric_distance(&t.coords))
                .max_by(|a, b| a.total_cmp(&b))
                .unwrap_or(0.0)
        })
        .collect()
}

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

#[derive(Debug, Clone)]
pub struct CrossSection {
    pub seed: usize,
    pub normal: Orientation,
    pub inliers: Vec<usize>,
    pub center: V,
}

impl CrossSection {
    fn new(cloud: &Vec<Point>, seed: usize, normal: Orientation, inliers: Vec<usize>) -> Self {
        let center = Self::center(cloud, &inliers);
        CrossSection {
            seed,
            normal,
            inliers,
            center,
        }
    }

    fn center(cloud: &Vec<Point>, inliers: &Vec<usize>) -> V {
        let points: Vec<&Point> = inliers.iter().map(|i| &cloud[*i]).collect();
        let mut sum = V::zeros();
        for point in &points {
            sum += point.coords;
        }
        sum / points.len() as f32
    }
}

pub fn detect_initial_cross_sections(
    cloud: &Vec<Point>,
    clusters: usize,
    surface_normals: &Vec<V>,
) -> Vec<CrossSection> {
    let (cluster_membership, centroids) = do_clustering(clusters, cloud);
    let seeds = select_seeds(cloud, &cluster_membership, &centroids);

    orient_planes(cloud, surface_normals, (), &seeds)
        .into_iter()
        .zip(seeds)
        .map(|((orient, inliers), seed)| CrossSection::new(cloud, seed, orient, inliers))
        .collect()
}
