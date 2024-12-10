use crate::common::*;
use std::f32::consts::PI;

/// Nodes in relaxed plushie have max distance of ~1.4
const CLUSTER_DISTANCE_THRESHOLD: f32 = 1.4;
// const GLOBAL_THRESHOLD: f32 = 1.5 * CLUSTER_DISTANCE_THRESHOLD;

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
        .filter_map(|(i, p)| ((normal_offset.dot(&p.coords) - d).abs() <= threshold).then_some(i))
        .collect();
    close_to_plane
    // let connected = close_to_plane
    //     .into_iter()
    //     .filter(|i| cloud[*i].coords.metric_distance(&cloud[seed].coords) <= GLOBAL_THRESHOLD)
    //     .collect();
    // connected
}

fn orient_cost(normals: &Vec<V>, inliers: &Vec<usize>, normal_offset: V) -> f32 {
    inliers
        .iter()
        .map(|i| normal_offset.dot(&normals[*i]).abs())
        .sum::<f32>()
        / inliers.len() as f32
}

#[derive(Debug, Clone)]
pub struct Orientation(pub f32, pub f32);

fn orient_plane(
    cloud: &Vec<Point>,
    normals: &Vec<V>,
    connectivity: (),
    seed: usize,
) -> (Orientation, Vec<usize>) {
    const ANGULAR_INTERVAL: f32 = PI / 6.0;
    const THETA_STEPS: usize = 12;
    const PHI_STEPS: usize = 4;

    let mut candidates: Vec<(Orientation, f32)> = Vec::with_capacity(THETA_STEPS * PHI_STEPS);
    let mut debug_inliers: Vec<Vec<usize>> = Vec::with_capacity(candidates.capacity());
    for theta in (0..THETA_STEPS).map(|t| t as f32 * ANGULAR_INTERVAL) {
        for phi in (0..PHI_STEPS).map(|p| p as f32 * ANGULAR_INTERVAL) {
            let normal_orient = V::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos());
            let inliers = get_inliers(
                cloud,
                connectivity,
                CLUSTER_DISTANCE_THRESHOLD,
                seed,
                normal_orient,
            );
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
