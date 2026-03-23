use glam::Vec3;

pub fn centroid_stuffing(
    node_positions: &[Vec3],
    centroid_positions: &[Vec3],
) -> (Vec<Vec3>, Vec<Vec3>) {
    let mut node_movement: Vec<Vec3> = vec![Vec3::ZERO; node_positions.len()];
    let mut centroid_new_positions: Vec<Vec3> = vec![Vec3::ZERO; centroid_positions.len()];

    let centroid_to_points = push_and_map(
        &node_positions,
        &centroid_positions,
        0.3,
        &mut node_movement,
    );
    recalculate_centroids(
        &node_positions,
        &mut centroid_new_positions,
        centroid_to_points,
    );

    (node_movement, centroid_new_positions)
}

fn push_and_map(
    nodes: &[Vec3],
    centroids: &[Vec3],
    centroid_force: f32,
    displacement: &mut [Vec3],
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
            displacement[i_p] += push_away(point, centroid) * centroid_force;
        }
        centroid_to_points[closest_i].push(i_p);
    }
    centroid_to_points
}

fn recalculate_centroids(nodes: &[Vec3], centroids: &mut [Vec3], centroid2points: Vec<Vec<usize>>) {
    centroids.iter_mut().enumerate().for_each(|(i, centroid)| {
        let mut new_pos: Vec3 = Vec3::ZERO;
        let mut weight_sum = 0.0;
        if centroid2points[i].len() == 0 {
            // log::warn!("No points assigned to centroid");
            return;
        }
        for point_index in &centroid2points[i] {
            let point = nodes[*point_index];
            let w = weight(distance(*centroid, point));
            new_pos += point * w;
            weight_sum += w;
        }
        assert!(weight_sum != 0.0, "About to divide by 0");
        let new_pos: Vec3 = Vec3::from(new_pos / weight_sum);
        *centroid = new_pos
    });
    // sanity!(centroids.assert_no_nan("after recalculating centroids"));
}

fn distance(a: Vec3, b: Vec3) -> f32 {
    a.distance(b)
}

fn weight(dist: f32) -> f32 {
    // https://www.desmos.com/calculator: e^{\frac{-\left(\ln\left(x\right)-b\right)^{2}}{c^{2}}}
    let b: f32 = 1.0;
    let c: f32 = 1.4;
    let x = dist;

    let numerator = -(x.ln() - b).powi(2);
    let denominator = c.powi(2);

    f32::exp(numerator / denominator) * 1.0
}

fn push_away(point: &Vec3, repelant: &Vec3) -> Vec3 {
    let diff = point - repelant;
    if diff.length() != 0.0 {
        let factor = 1.0;
        let res = diff.normalize() * (factor / diff.length_squared()).min(1.0);
        // sanity!(res.assert_no_nan("NaN while pushing"));
        res
    } else {
        Vec3::ZERO
    }
}
