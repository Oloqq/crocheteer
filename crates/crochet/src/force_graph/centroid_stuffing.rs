use glam::Vec3;

pub fn stuff(
    node_positions: &[Vec3],
    centroid_positions: &[Vec3],
    desired_node_distance: f32,
) -> (Vec<Vec3>, Vec<Vec3>) {
    let mut node_movement: Vec<Vec3> = vec![Vec3::ZERO; node_positions.len()];
    let mut centroid_new_positions: Vec<Vec3> = vec![Vec3::ZERO; centroid_positions.len()];
    let centroid_force = 0.05;

    let centroid_to_points = push_and_map(
        &node_positions,
        &centroid_positions,
        centroid_force,
        &mut node_movement,
        desired_node_distance,
    );
    recalculate_centroids(
        &node_positions,
        &mut centroid_new_positions,
        centroid_to_points,
        desired_node_distance,
    );

    (node_movement, centroid_new_positions)
}

fn push_and_map(
    nodes: &[Vec3],
    centroids: &[Vec3],
    centroid_force: f32,
    displacement: &mut [Vec3],
    desired_node_distance: f32,
) -> Vec<Vec<usize>> {
    assert_eq!(nodes.len(), displacement.len());
    let mut centroid_to_points = vec![vec![]; centroids.len()];
    for (i_p, point) in nodes.iter().enumerate() {
        let mut closest_i = 0;
        let mut closest = distance(*point, centroids[closest_i]);
        for (i_c, centroid) in centroids.iter().enumerate() {
            if distance(*point, *centroid) < closest {
                closest = distance(*point, *centroid);
                closest_i = i_c;
            }
            displacement[i_p] += push_away(point, centroid, desired_node_distance) * centroid_force;
        }
        centroid_to_points[closest_i].push(i_p);
    }
    centroid_to_points
}

fn recalculate_centroids(
    nodes: &[Vec3],
    centroids: &mut [Vec3],
    centroid2points: Vec<Vec<usize>>,
    desired_node_distance: f32,
) {
    centroids.iter_mut().enumerate().for_each(|(i, centroid)| {
        let mut new_pos: Vec3 = Vec3::ZERO;
        let mut weight_sum = 0.0;
        if centroid2points[i].len() == 0 {
            // log::warn!("No points assigned to centroid");
            return;
        }
        for point_index in &centroid2points[i] {
            let point = nodes[*point_index];
            let w = weight(distance(*centroid, point), desired_node_distance);
            new_pos += point * w;
            weight_sum += w;
        }

        // assert!(weight_sum != 0.0, "About to divide by 0"); // can happen if centroids are added too quickly
        if weight_sum != 0.0 {
            let new_pos: Vec3 = Vec3::from(new_pos / weight_sum);
            *centroid = new_pos
        }
    });
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
