use core::f32;
use std::collections::HashSet;

use super::initial_cross_sections::CrossSection;
use super::utils::Orientation;
use crate::common::*;
use crate::skeletonization::utils::find_best_plane;

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
    // let backwards = grow_in_direction(-1.0, &initial_section, cloud, surface_normals);
    // let sections = backwards
    //     .into_iter()
    //     .rev()
    //     .chain(std::iter::once(initial_section))
    //     .chain(forwards.into_iter())
    //     .collect();

    let sections = forwards;

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
    part_members: &mut std::collections::HashSet<usize>,
) -> Option<CrossSection> {
    let (theta, phi) = (source.normal.0, source.normal.1);
    let normal_orient = V::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos());
    let direction = direction * normal_orient;
    let delta_step = 1.0; // arbitrary
    let new_center = source.center + direction * delta_step;

    const DELTA_ANG_DEG: f32 = 12.5;
    let delta_ang = DELTA_ANG_DEG.to_radians();

    println!(
        "new center: {}, theta {}, phi {}, dang {}",
        new_center, theta, phi, delta_ang
    );

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
    let (best_plane_orientation, inliers) =
        find_best_plane(cloud, surface_normals, (), seed, &considered_normals);

    if inliers.len() == 0 {
        unreachable!();
    }
    let mut added = 0;
    for point in &inliers {
        if part_members.insert(*point) {
            added += 1;
        }
    }

    if added == 0 {
        return None;
    }

    Some(CrossSection::new(
        cloud,
        seed,
        best_plane_orientation,
        inliers,
    ))
}
