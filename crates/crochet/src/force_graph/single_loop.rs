use glam::Vec3;

use crate::{data::Peculiarity, force_graph::simulated_plushie::Node};

pub fn single_loop_forces(nodes: &[Node], multiplier: f32, displacement: &mut [Vec3]) {
    for (i, node) in nodes.iter().enumerate() {
        let (push_plane_spec, direction) = match node.definition.peculiarity {
            Some(Peculiarity::BLO(x)) => (x, 1.0),
            Some(Peculiarity::FLO(x)) => (x, -1.0),
            _ => continue,
        };
        let a: Vec3 = nodes[push_plane_spec.0].position;
        let b: Vec3 = nodes[push_plane_spec.1].position;
        let c: Vec3 = nodes[push_plane_spec.2].position;
        let normal = based_on_push_plane(a, b, c, direction);
        displacement[i] += normal * multiplier;
    }
}

fn based_on_push_plane(a: Vec3, b: Vec3, c: Vec3, direction: f32) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    let cross = ab.cross(ac);

    if cross.length() != 0.0 {
        let normal = cross.normalize() * direction;
        normal
    } else {
        log::warn!("Colinear points prevent applying single loop force");
        Vec3::ZERO
    }
}
