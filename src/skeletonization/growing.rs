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

    const DELTA_ANG_DEG: f32 = 12.5;
    let delta_ang = DELTA_ANG_DEG.to_radians();
    const NUM_STEPS: usize = 3;
    let thetas = vec![theta - delta_ang, theta, theta + delta_ang];
    let phis = vec![phi - delta_ang, phi, phi + delta_ang];
    assert_eq!(thetas.len(), NUM_STEPS);
    assert_eq!(phis.len(), NUM_STEPS);

    todo!()
}
