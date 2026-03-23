use glam::Vec3;

/// O(N) assuming close-to-constant edge count in each node
pub fn link_forces(nodes: &[Vec3], edges: &Vec<Vec<usize>>) -> Vec<Vec3> {
    let mut forces = vec![Vec3::ZERO; nodes.len()];
    let desired_stitch_distance = 1.0;
    for (i, point) in nodes.iter().enumerate() {
        for neibi in &edges[i] {
            if *neibi >= nodes.len() {
                continue; // assert that it doesn't happen?
            }
            let neib = &nodes[*neibi];
            let diff = point - neib;
            let force: Vec3 =
                -diff.normalize() * link_force_magnitude(diff.length(), desired_stitch_distance);
            forces[i] += force;
            forces[*neibi] -= force;
        }
    }
    // sanity!(self.displacement.assert_no_nan("link forces"));
    forces
}

/// Attract nodes far away, repel nodes close to each other
///
/// \min\left(\frac{\left(x-d\right)^{3}}{\left(\frac{x}{2}+d\right)^{3}},\ 1\right)\left\{x\ge\ 0\right\}
pub fn link_force_magnitude(distance: f32, desired_distance: f32) -> f32 {
    debug_assert!(desired_distance > 0.0);
    let x = distance;
    let d = desired_distance;
    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    fx.min(1.0)
}
