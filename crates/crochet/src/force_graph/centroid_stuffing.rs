use std::ops::Not;

use glam::Vec3;

use crate::force_graph::simulated_plushie::Node;

pub fn centroid_stuffing(
    nodes: &[Node],
    centroids: &mut Vec<Vec3>,
    hook_size: f32,
    displacement: &mut [Vec3],
) {
    assert_eq!(nodes.len(), displacement.len());
    let centroid_force = 0.05;
    let centroid_to_points =
        push_and_map(&nodes, &centroids, centroid_force, displacement, hook_size);
    recalculate_centroids(&nodes, centroids, centroid_to_points, hook_size);
}

fn push_and_map(
    nodes: &[Node],
    centroids: &[Vec3],
    centroid_force: f32,
    displacement: &mut [Vec3],
    hook_size: f32,
) -> Vec<Vec<usize>> {
    let mut centroid_to_points = vec![vec![]; centroids.len()];
    if centroids.is_empty() {
        return centroid_to_points;
    }

    for (node_index, node) in nodes.iter().enumerate() {
        let mut closest_centroid_index = 0;
        let mut closest = distance(node.position, centroids[closest_centroid_index]);
        for (centroid_index, centroid) in centroids.iter().enumerate() {
            if distance(node.position, *centroid) < closest {
                closest = distance(node.position, *centroid);
                closest_centroid_index = centroid_index;
            }
            displacement[node_index] +=
                push_away(&node.position, centroid, hook_size) * centroid_force;
        }
        centroid_to_points[closest_centroid_index].push(node_index);
    }
    centroid_to_points
}

fn recalculate_centroids(
    nodes: &[Node],
    centroids: &mut [Vec3],
    centroid_to_nodes: Vec<Vec<usize>>,
    desired_node_distance: f32,
) {
    let mut orphans: Vec<usize> = vec![];

    for (i, centroid) in centroids.iter_mut().enumerate() {
        let mut new_pos: Vec3 = Vec3::ZERO;
        let mut weight_sum = 0.0;
        if centroid_to_nodes[i].len() == 0 {
            orphans.push(i);
            continue;
        }
        for node_index in &centroid_to_nodes[i] {
            let node = &nodes[*node_index];
            let w = weight(distance(*centroid, node.position), desired_node_distance);
            new_pos += node.position * w;
            weight_sum += w;
        }

        // assert!(weight_sum != 0.0, "About to divide by 0"); // can happen if centroids are added too quickly
        if weight_sum != 0.0 {
            let new_pos: Vec3 = Vec3::from(new_pos / weight_sum);
            *centroid = new_pos
        }
    }

    if orphans.len() > 0 {
        let arbitrary_non_orphan = centroids
            .iter()
            .enumerate()
            .find_map(|(i, pos)| orphans.contains(&i).not().then_some(*pos));

        if let Some(non_orphan_pos) = arbitrary_non_orphan {
            for orphan in orphans {
                centroids[orphan] = non_orphan_pos + Vec3::new(desired_node_distance, 0.0, 0.0);
            }
        } else {
            log::warn!("centroids are orphaned and can't be reunited");
        }
    }

    // sanity!(centroids.assert_no_nan("after recalculating centroids"));
}

fn distance(a: Vec3, b: Vec3) -> f32 {
    a.distance(b)
}

pub fn weight(dist: f32, desired_node_distance: f32) -> f32 {
    // \left(e^{\frac{-\left(\ln\left(\frac{x}{20d}\right)-b\right)^{2}}{c^{2}}}\right)
    let b: f32 = -3.0;
    let c: f32 = 2.0;
    let x = dist;
    let logarg = x / (20.0 * desired_node_distance);

    let numerator = -(logarg.ln() - b).powi(2);
    let denominator = c.powi(2);

    f32::exp(numerator / denominator) * 1.0
}

fn push_away(point: &Vec3, repelant: &Vec3, desired_node_distance: f32) -> Vec3 {
    let diff = point - repelant;
    if diff.length() != 0.0 {
        let magnitude = centroid_push_magnitude(diff.length(), desired_node_distance);
        let res = diff.normalize() * magnitude;
        // sanity!(res.assert_no_nan("NaN while pushing"));
        res
    } else {
        Vec3::ZERO
    }
}

// \min\left(1,\ \frac{d}{\left(x^{2}\right)+d}\right)\left\{x>0\right\}
pub fn centroid_push_magnitude(distance: f32, desired_node_distance: f32) -> f32 {
    // cutoff distance?
    (desired_node_distance / (distance.powi(2) + desired_node_distance)).min(1.0)
}
