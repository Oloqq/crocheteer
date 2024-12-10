mod initial_cross_sections;
mod local_surface_normals;

#[rustfmt::skip]
mod in_execution_order {
    use super::*;

    pub use local_surface_normals::local_surface_normals_per_point;

    #[allow(unused)]
    pub use initial_cross_sections::max_dists;
    pub use initial_cross_sections::do_clustering;
    pub use initial_cross_sections::select_seeds;
    pub use initial_cross_sections::orient_planes;
    pub use initial_cross_sections::detect_initial_cross_sections;
}

pub use in_execution_order::*;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_bruh() {
        use nalgebra::{DMatrix, SymmetricEigen};
        // Suppose you have n 3D points in a Vec of arrays:
        let points = vec![
            [-1.0, 2.0, 3.0],
            [1.0, 2.0, 3.0],
            [1.0, 2.0, 3.0],
            [1.0, 2.0, 3.0],
        ];

        // Convert this into a DMatrix<f64> of shape (n, 3)
        // Each row is a data point (x, y, z)
        let n = points.len();
        let mut data = DMatrix::from_iterator(3, n, points.iter().flat_map(|&p| p.clone()));
        println!("data {}", &data);

        let mean = data.column_mean();
        println!("mean {}", mean);

        // 2. Center data: For each row, subtract the mean
        for mut col in data.column_iter_mut() {
            col -= &mean;
        }

        // 3. Compute covariance matrix (3x3)
        // Covariance = (X^T X) / (n - 1)
        // shape(X) = (n,3), shape(X^T X) = (3,3)
        let cov = (&data.transpose() * &data) / (n as f64 - 1.0);

        // 4. Perform eigen decomposition on the symmetric covariance matrix
        let eig = SymmetricEigen::new(cov);

        // eig.eigenvalues and eig.eigenvectors are now available
        // Sort eigenvalues (and vectors) by descending order of eigenvalue
        let mut eigen_pairs: Vec<(f64, Vec<f64>)> = eig
            .eigenvalues
            .iter()
            .zip(eig.eigenvectors.column_iter())
            .map(|(val, vec)| (*val, vec.iter().copied().collect()))
            .collect();

        eigen_pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // The two most significant eigenvalues:
        let top_two_eigenvalues = [eigen_pairs[0].0, eigen_pairs[1].0];
        println!("Most significant eigenvalues: {:?}", top_two_eigenvalues);

        // Optional: If you need the principal components (eigenvectors):
        // let top_two_eigenvectors = [eigen_pairs[0].1.clone(), eigen_pairs[1].1.clone()];
        // println!("Corresponding eigenvectors: {:?}", top_two_eigenvectors);
        // assert!(false);
    }

    #[test]
    fn test_bruh2() {
        // let data = ndarray::array![[1.0, 1.0, 0.0], [0.0, 2.0, 0.0], [0.0, 3.0, 0.0]];
        // Assuming data is an Array2<f64> of shape (n_points, 3)
        // let n = data.shape()[0];
        // // Compute the mean along axis 0, resulting in a 1D array [3]
        // let mean = data.mean_axis(Axis(0)).unwrap(); // shape is (3,)

        // // Method 1: Use broadcasting directly
        // // Broadcast mean from shape (3,) to (n,3)
        // let mean_broadcasted = mean.broadcast((data.nrows(), data.ncols())).unwrap();
        // let centered = &data - &mean_broadcasted;

        // // Compute covariance matrix (3x3)
        // let cov = centered.t().dot(&centered) / (n as f64 - 1.0);

        // // Eigen decomposition
        // let (eigenvalues, _eigenvectors) = cov.eig().unwrap();

        // println!("eigen {}", eigenvalues);
        // assert!(false);
    }
}
