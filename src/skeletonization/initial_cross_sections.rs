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

    println!("Centroids: {:?}", result.centroids);
    println!("Cluster-Assignments: {:?}", result.assignments);
    println!("Error: {}", result.distsum);

    // let centroids: Vec<Point> = result
    //     .centroids
    //     .iter()
    //     .array_chunks::<3>()
    //     .map(|[x, y, z]| Point::new((*x)[0], (*y)[0], (*z)[0]))
    //     .collect();

    let centroids = vec![Point::new(0.0, 0.0, 0.0); num_clusters];

    // maybe these could be included in the type system?
    assert!(result.assignments.len() == points.len());
    assert_eq!(centroids.len(), num_clusters);
    (result.assignments, centroids)
}

// pub fn detect_initial_cross_sections() {}
