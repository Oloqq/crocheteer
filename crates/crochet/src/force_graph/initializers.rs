use std::f32::consts::PI;

use glam::Vec3;

#[derive(PartialEq)]
pub enum Initializer {
    /// Start with nodes arranged into a cylinder shape. Advance height every X nodes.
    /// Magic Ring is placed at origin.
    RegularCylinder(u32),
    /// Spawn the nodes one by one, waiting for the previous node to reach a relatively stable position before advancing.
    OneByOne,
}

impl Initializer {
    pub fn apply(&self, nodes_num: u32, hook_size: f32) -> Vec<Vec3> {
        match self {
            Initializer::RegularCylinder(nodes_in_cirumference) => {
                arrange_cylinder(nodes_num, *nodes_in_cirumference, hook_size)
            }
            Initializer::OneByOne => vec![
                Vec3::ZERO,
                Vec3::new(hook_size, 0.0, 0.0),
                Vec3::new(0.0, 0.0, hook_size),
            ],
        }
    }
}

fn arrange_cylinder(nodes_num: u32, nodes_in_cirumference: u32, hook_size: f32) -> Vec<Vec3> {
    if nodes_num == 0 {
        return vec![];
    }

    let mut y = 0.0;
    let mut nodes = vec![Vec3::ZERO]; // for the magic ring

    while nodes.len() as u32 + nodes_in_cirumference < nodes_num {
        nodes.append(&mut ring(nodes_in_cirumference, y, hook_size));
        y += hook_size;
    }
    nodes.append(&mut ring(nodes_num - nodes.len() as u32, y, hook_size));
    nodes
}

fn ring(members: u32, y: f32, hook_size: f32) -> Vec<Vec3> {
    let circumference = hook_size * members as f32;
    let radius = circumference / (2.0 * PI);

    let angular_interval = 2.0 * PI / members as f32;
    let mut result: Vec<Vec3> = vec![];

    for i in 0..members {
        let rads = angular_interval * i as f32;
        let x = rads.cos() * radius;
        let z = rads.sin() * radius;
        let point = Vec3::new(x, y, z);
        result.push(point);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_cylinder() {
        let res = arrange_cylinder(5, 12, 1.0);
        assert_eq!(res.len(), 5);

        let res = arrange_cylinder(9, 12, 1.0);
        assert_eq!(res.len(), 9);

        let res = arrange_cylinder(21, 12, 1.0);
        assert_eq!(res.len(), 21);

        let res = arrange_cylinder(12, 12, 1.0);
        assert_eq!(res.len(), 12);
    }
}
