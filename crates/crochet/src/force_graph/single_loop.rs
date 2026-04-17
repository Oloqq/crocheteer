use glam::Vec3;

use crate::data::Peculiarity;

pub fn find_normals(nodes: &[(Vec3, Option<Peculiarity>)]) -> Vec<Vec3> {
    nodes
        .iter()
        .map(|(_, peculiarity)| {
            let (push_plane_spec, direction) = match peculiarity {
                Some(Peculiarity::BLO(x)) => (x, 1.0),
                Some(Peculiarity::FLO(x)) => (x, -1.0),
                _ => return Vec3::ZERO,
            };
            let a: Vec3 = nodes[push_plane_spec.0].0;
            let b: Vec3 = nodes[push_plane_spec.1].0;
            let c: Vec3 = nodes[push_plane_spec.2].0;
            based_on_push_plane(a, b, c, direction)
        })
        .collect()
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
