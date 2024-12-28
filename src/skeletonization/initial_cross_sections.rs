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
    pub scale: na::Vector2<f32>,
}

impl CrossSection {
    pub fn new(cloud: &Vec<Point>, seed: usize, normal: Orientation, inliers: Vec<usize>) -> Self {
        let center = center(cloud, &inliers);
        let scale = get_e1_e2(cloud, &inliers);
        CrossSection {
            seed,
            normal,
            inliers,
            center,
            scale,
        }
    }
}

pub fn detect_initial_cross_sections(
    cloud: &Vec<Point>,
    edges: &Vec<Vec<usize>>,
    clusters: usize,
    surface_normals: &Vec<V>,
) -> Vec<CrossSection> {
    let (cluster_membership, centroids) = do_clustering(clusters, cloud);
    let seeds = select_seeds(cloud, &cluster_membership, &centroids);

    orient_planes(cloud, surface_normals, edges, &seeds)
        .into_iter()
        .zip(seeds)
        .map(|((orient, inliers), seed)| CrossSection::new(cloud, seed, orient, inliers))
        .collect()
}

fn center(cloud: &Vec<Point>, inliers: &Vec<usize>) -> V {
    let points: Vec<&Point> = inliers.iter().map(|i| &cloud[*i]).collect();
    let mut sum = V::zeros();
    for point in &points {
        sum += point.coords;
    }
    sum / points.len() as f32
}

fn get_e1_e2(cloud: &Vec<Point>, inliers: &Vec<usize>) -> na::Vector2<f32> {
    use nalgebra::{DMatrix, SymmetricEigen};
    // Suppose you have n 3D points in a Vec of arrays:
    let points: Vec<[f32; 3]> = inliers
        .iter()
        .map(|x| [cloud[*x].coords.x, cloud[*x].coords.y, cloud[*x].coords.z])
        .collect();

    // Convert this into a DMatrix<f64> of shape (n, 3)
    // Each row is a data point (x, y, z)
    let n = inliers.len();
    let mut data = DMatrix::from_iterator(3, n, points.iter().flat_map(|&p| p.clone()));
    // println!("data {}", &data);

    let mean = data.column_mean();
    // println!("mean {}", mean);

    // 2. Center data: For each row, subtract the mean
    for mut col in data.column_iter_mut() {
        col -= &mean;
    }

    // 3. Compute covariance matrix (3x3)
    // Covariance = (X^T X) / (n - 1)
    // shape(X) = (n,3), shape(X^T X) = (3,3)
    let cov = (&data.transpose() * &data) / (n as f32 - 1.0);

    // 4. Perform eigen decomposition on the symmetric covariance matrix
    let eig = SymmetricEigen::new(cov);

    // eig.eigenvalues and eig.eigenvectors are now available
    // Sort eigenvalues (and vectors) by descending order of eigenvalue
    let mut eigen_pairs: Vec<(f32, Vec<f32>)> = eig
        .eigenvalues
        .iter()
        .zip(eig.eigenvectors.column_iter())
        .map(|(val, vec)| (*val, vec.iter().copied().collect()))
        .collect();

    eigen_pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // The two most significant eigenvalues:
    let top_two_eigenvalues = [eigen_pairs[0].0, eigen_pairs[1].0];
    // println!("Most significant eigenvalues: {:?}", top_two_eigenvalues);

    na::Vector2::<f32>::new(top_two_eigenvalues[0] as f32, top_two_eigenvalues[1] as f32)
}
