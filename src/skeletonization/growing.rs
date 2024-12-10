use super::initial_cross_sections::CrossSection;
use crate::common::*;

pub struct Part {
    pub sections: Vec<CrossSection>,
}

pub fn grow(
    cloud: &Vec<Point>,
    cross_sections: Vec<CrossSection>,
    surface_normals: &Vec<V>,
) -> Vec<Part> {
    cross_sections
        .into_iter()
        .map(|cs| grow_single_part(cloud, cs, surface_normals))
        .collect()
}

fn grow_single_part(
    cloud: &Vec<Point>,
    initial_section: CrossSection,
    surface_normals: &Vec<V>,
) -> Part {
    let forwards = grow_in_direction(1.0, &initial_section, cloud, surface_normals);
    let backwards = grow_in_direction(-1.0, &initial_section, cloud, surface_normals);
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
    surface_normals: &Vec<V>,
) -> Vec<CrossSection> {
    let mut result = Vec::new();
    let mut current: Option<&CrossSection> = Some(section);

    for i in 0..100_000 {
        if i >= 1000 {
            panic!("deadlock");
        }

        let source = current.as_ref().expect("some cross section");
        if let Some(new) = sprout(direction, &source, cloud, surface_normals) {
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
) -> Option<CrossSection> {
    let (theta, phi) = (source.normal.0, source.normal.1);
    let normal_orient = V::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos());
    let direction = direction * normal_orient;
    let delta_step = 0.5; // arbitrary
    let new_center = source.center + direction * delta_step;

    todo!()
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

// fn orient_plane(
//     cloud: &Vec<Point>,
//     normals: &Vec<V>,
//     connectivity: (),
//     seed: usize,
// ) -> (Orientation, Vec<usize>) {
//     use std::f32::consts::PI;
//     const ANGULAR_INTERVAL: f32 = PI / 6.0;
//     const THETA_STEPS: usize = 12;
//     const PHI_STEPS: usize = 4;

//     let mut candidates: Vec<(Orientation, f32)> = Vec::with_capacity(THETA_STEPS * PHI_STEPS);
//     let mut debug_inliers: Vec<Vec<usize>> = Vec::with_capacity(candidates.capacity());
//     for theta in (0..THETA_STEPS).map(|t| t as f32 * ANGULAR_INTERVAL) {
//         for phi in (0..PHI_STEPS).map(|p| p as f32 * ANGULAR_INTERVAL) {
//             let normal_orient = V::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos());
//             let inliers = get_inliers(
//                 cloud,
//                 connectivity,
//                 CLUSTER_DISTANCE_THRESHOLD,
//                 seed,
//                 normal_orient,
//             );
//             let cost = orient_cost(normals, &inliers, normal_orient);
//             candidates.push((Orientation(theta, phi), cost));
//             debug_inliers.push(inliers);
//         }
//     }

//     let (index, best_orientation) = candidates
//         .into_iter()
//         .enumerate()
//         .min_by(|(_, a), (_, b)| a.1.total_cmp(&b.1))
//         .and_then(|(i, candidate)| Some((i, candidate.0)))
//         .unwrap();

//     (best_orientation, debug_inliers.swap_remove(index))
// }

// pub fn orient_planes(
//     cloud: &Vec<Point>,
//     normals: &Vec<V>,
//     connectivity: (),
//     seeds: &Vec<usize>,
// ) -> Vec<(Orientation, Vec<usize>)> {
//     seeds
//         .iter()
//         .map(|seed| orient_plane(cloud, normals, connectivity, *seed))
//         .collect()
// }
