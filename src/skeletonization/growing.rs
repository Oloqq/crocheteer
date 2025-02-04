use core::f32;
use std::collections::HashSet;

use super::initial_cross_sections::CrossSection;
use super::utils::Orientation;
use super::Connectivity;
use crate::common::*;
use crate::skeletonization::utils::find_best_plane;

pub struct Part {
    pub sections: Vec<CrossSection>,
}

pub fn grow(
    cloud: &Vec<Point>,
    edges: &Connectivity,
    cross_sections: Vec<CrossSection>,
    surface_normals: &Vec<V>,
) -> Vec<Part> {
    cross_sections
        .into_iter()
        .map(|cs| grow_single_part(cloud, edges, cs, surface_normals))
        .collect()
}

fn grow_single_part(
    cloud: &Vec<Point>,
    edges: &Connectivity,
    initial_section: CrossSection,
    surface_normals: &Vec<V>,
) -> Part {
    let forwards = grow_in_direction(1.0, &initial_section, cloud, edges, surface_normals);
    let backwards = grow_in_direction(-1.0, &initial_section, cloud, edges, surface_normals);
    let sections = backwards
        .into_iter()
        .rev()
        .chain(std::iter::once(initial_section))
        .chain(forwards.into_iter())
        .collect();

    Part { sections }
}

fn grow_in_direction(
    direction: f32,
    section: &CrossSection,
    cloud: &Vec<Point>,
    edges: &Connectivity,
    surface_normals: &Vec<V>,
) -> Vec<CrossSection> {
    let mut result = Vec::new();
    let mut current: Option<&CrossSection> = Some(section);
    let mut part_members: HashSet<usize> = HashSet::from_iter(section.inliers.iter().cloned());

    for i in 0..1_000 {
        if i >= 100 {
            panic!("deadlock");
        }

        let source = current.as_ref().expect("some cross section");
        if let Some(new) = sprout(
            direction,
            &source,
            cloud,
            surface_normals,
            edges,
            &mut part_members,
        ) {
            result.push(new);
            current = result.last();
        } else {
            break;
        }
    }

    result
}

fn sprout(
    direction: f32,
    source: &CrossSection,
    cloud: &Vec<Point>,
    surface_normals: &Vec<V>,
    edges: &Connectivity,
    part_members: &mut std::collections::HashSet<usize>,
) -> Option<CrossSection> {
    let (theta, phi) = (source.normal.0, source.normal.1);
    let normal_orient = V::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos());
    let direction = direction * normal_orient;
    let delta_step = 1.5; // arbitrary
    let new_center = source.center + direction * delta_step;

    const DELTA_ANG_DEG: f32 = 12.5;
    let delta_ang = DELTA_ANG_DEG.to_radians();

    // println!(
    //     "new center: {}, theta {}, phi {}, dang {}",
    //     new_center, theta, phi, delta_ang
    // );

    let mut considered_normals: Vec<(V, Orientation)> = Vec::with_capacity(9);
    for theta in [theta - delta_ang, theta, theta + delta_ang] {
        for phi in [phi - delta_ang, phi, phi + delta_ang] {
            considered_normals.push((
                V::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos()),
                Orientation(theta, phi),
            ));
        }
    }

    // TODO kd tree
    let (seed, _) = cloud
        .iter()
        .enumerate()
        .min_by(|(_, point), (_, b)| {
            point
                .coords
                .metric_distance(&new_center)
                .total_cmp(&b.coords.metric_distance(&new_center))
        })
        .unwrap();
    let (best_plane_orientation, inliers, orient_cost) =
        find_best_plane(cloud, surface_normals, edges, seed, &considered_normals);

    // why did I ever say it was unreachable? investigate
    // if inliers.len() == 0 {
    //     unreachable!();
    // }
    let mut added = 0;
    for point in &inliers {
        if part_members.insert(*point) {
            added += 1;
        }
    }

    if added == 0 {
        return None;
    }

    let new_section = CrossSection::new(cloud, seed, best_plane_orientation, inliers, orient_cost);

    if scale_changed_too_much(source.scale, new_section.scale, 0.9) {
        None
    } else {
        Some(new_section)
    }
}

fn scale_changed_too_much(ei: na::Vector2<f32>, ej: na::Vector2<f32>, acceptable: f32) -> bool {
    let numer = ei.metric_distance(&ej);
    let denom = ei.norm();

    f32::abs(1.0 - (numer / denom)) > acceptable
}

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
}
