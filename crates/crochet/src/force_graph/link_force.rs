use glam::Vec3;

use crate::{data::Edges, force_graph::simulated_plushie::Node};

/// O(N) assuming close-to-constant edge count in each node
pub(crate) fn link_forces(
    nodes: &[Node],
    edges: &Edges,
    hook_size: f32,
    displacement: &mut Vec<Vec3>,
) {
    for (i, node) in nodes.iter().enumerate() {
        for neibi in &edges.data()[i] {
            if *neibi >= nodes.len() {
                continue; // assert that it doesn't happen?
            }
            let neighbor = &nodes[*neibi];
            let diff = node.position - neighbor.position;
            let force: Vec3 = -diff.normalize() * link_force_magnitude(diff.length(), hook_size);
            displacement[i] += force;
            displacement[*neibi] -= force;
        }
    }
    // sanity!(self.displacement.assert_no_nan("link forces"));
}

/// Attract nodes far away, repel nodes close to each other
/// Returns value in [-1, 1]
///
/// \min\left(\frac{\left(x-d\right)^{3}}{\left(\frac{x}{2}+d\right)^{3}},\ 1\right)\left\{x\ge\ 0\right\}
pub fn link_force_magnitude(distance: f32, desired_distance: f32) -> f32 {
    debug_assert!(desired_distance > 0.0);
    let x = distance;
    let d = desired_distance;
    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    fx.min(1.0)
}
