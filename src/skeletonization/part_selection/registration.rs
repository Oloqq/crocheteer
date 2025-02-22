// This is AI generated and I do not claim to understand it

#![allow(unused)]

use kiddo::KdTree;
use kiddo::SquaredEuclidean;
use kmeans::EuclideanDistance;
use na::Matrix3;
use na::Rotation3;
use na::UnitQuaternion;
use na::Vector3;
type Num = f32;
type CrossSection = Vec<na::Vector3<Num>>;

#[derive(Debug, Clone)]
pub struct Similarity3 {
    pub scale: Num,
    pub rotation: UnitQuaternion<Num>,
    pub translation: Vector3<Num>,
}

impl Similarity3 {
    /// Apply this similarity transform to a 3D point.
    pub fn transform_point(&self, p: &Vector3<Num>) -> Vector3<Num> {
        // p' = scale * (rotation * p) + translation
        self.scale * (self.rotation * p) + self.translation
    }
}

pub fn icp_similarity_transform(
    source: &mut CrossSection,
    target: &CrossSection,
    max_iterations: usize,
    convergence_threshold: Num,
) -> Similarity3 {
    let mut accumulated = Similarity3 {
        scale: 1.0,
        rotation: UnitQuaternion::identity(),
        translation: Vector3::zeros(),
    };

    for _ in 0..max_iterations {
        let sim_step = register_similarity_transform(source, target);
        // println!("iteration sim step {:?}", sim_step);

        // Apply this step to the source
        transform_cross_section(source, &sim_step);

        // Compose step with our accumulated transform
        // accumulated = compose_similarity(&sim_step, &accumulated);
        accumulated = compose_similarity(&accumulated, &sim_step);

        // Check for convergence:
        //   We look at how much the new step changed scale, rotation, translation.
        //   For example, if translation is small, scale is close to 1.0,
        //   and rotation is a small angle, we can say “we’re done.”
        if is_small_transform(&sim_step, convergence_threshold) {
            break;
        }
    }

    accumulated
}

pub fn register_similarity_transform(source: &CrossSection, target: &CrossSection) -> Similarity3 {
    // 1) Build KD-tree for 'target'
    let mut kd_tree = KdTree::<Num, 3>::new();
    for (i, &pt) in target.iter().enumerate() {
        kd_tree.add(&[pt.x, pt.y, pt.z], i.try_into().unwrap());
    }

    // 2) For each point in 'source', find NN in 'target' => collect correspondences
    let mut source_matched = Vec::new();
    let mut target_matched = Vec::new();

    for &s in source.iter() {
        let nearest = kd_tree.nearest_one::<SquaredEuclidean>(&[s.x, s.y, s.z]);
        let t_idx = nearest.item;
        let t_pt = target[t_idx as usize];
        source_matched.push(s);
        target_matched.push(t_pt);
    }

    // 3) Solve for the similarity transform that best aligns source_matched to target_matched.
    //    We'll follow this approach:
    //
    //    - Compute centroids of both sets.
    //    - "Demean" each set: (p_i - centroid).
    //    - We want to find scale 'alpha' and rotation 'R' (3x3) that minimize
    //         sum_i || (alpha R (s_i - s_mean) + t_mean) - (t_i) ||^2
    //      in a least squares sense. The translation is then:
    //         trans = t_mean - alpha * R * s_mean
    //
    //    - The rotation R can be found via SVD on the covariance of the demeaned points,
    //      just like a classic rigid alignment. Once we get R, we can solve for alpha
    //      by ratio of variances. Then translation is derived.
    //
    // Reference: https://igl.ethz.ch/projects/ARAP/svd_rot.pdf
    // or “Least-Squares Fitting of Two 3D Point Sets” by Arun et al. (1987).

    let source_mean = mean_point(&source_matched);
    let target_mean = mean_point(&target_matched);

    // Demean both sets
    let source_demeaned: Vec<Vector3<Num>> =
        source_matched.iter().map(|p| p - source_mean).collect();
    let target_demeaned: Vec<Vector3<Num>> =
        target_matched.iter().map(|p| p - target_mean).collect();

    // Build cross-covariance matrix: sum over i of (source_demeaned_i * target_demeaned_i^T).
    let mut cov = Matrix3::<Num>::zeros();
    for (s, t) in source_demeaned.iter().zip(target_demeaned.iter()) {
        cov += s * t.transpose();
    }

    // SVD of covariance
    let svd = cov.svd(true, true);

    let u = svd.u.unwrap();

    let v_t = svd.v_t.unwrap();

    // Compute rotation
    let mut r = v_t.transpose() * u.transpose();

    // Fix possible reflection if determinant is negative
    if r.determinant() < 0.0 {
        let mut fix = Matrix3::identity();
        fix[(2, 2)] = -1.0; // flip the sign on the z-axis of v
        r = v_t.transpose() * fix * u.transpose();
    }

    // for (i, (&s, &t)) in source_matched.iter().zip(target_matched.iter()).enumerate() {
    //     println!("  match #{i} => source: {s:?}, target: {t:?}");
    // If you see the same
    // 𝑡
    // t for multiple
    // 𝑠
    // s, or only 1–2 unique target points for 4 source points, that is likely the problem.
    // }
    let rotation = UnitQuaternion::from_rotation_matrix(&Rotation3::from_matrix(&r));

    // Compute scale (alpha).
    //
    //  alpha = (sum_i ||t_i'||) / (sum_i ||s_i'||)
    //  where s_i' = R * s_i_demeaned, t_i' = t_i_demeaned.
    //  In practice, we can compute the ratio of Frobenius norms:
    //
    //    alpha = sqrt( (sum_i ||t_i_demeaned||^2) / (sum_i ||R*s_i_demeaned||^2 ) )
    //
    // But since R is orthonormal, ||R*s_i|| = ||s_i||, so it’s simpler:
    let numerator = target_demeaned
        .iter()
        .map(|t| t.norm_squared())
        .sum::<Num>();
    let denominator = source_demeaned
        .iter()
        .map(|s| s.norm_squared())
        .sum::<Num>();

    let scale = if denominator.abs() < 1e-12 {
        1.0
    } else {
        (numerator / denominator).sqrt()
    };

    // Now compute translation
    //   trans = t_mean - alpha * R * s_mean
    let translation = target_mean - scale * (rotation * source_mean);

    Similarity3 {
        scale,
        rotation,
        translation,
    }
}

fn mean_point(points: &[Vector3<Num>]) -> Vector3<Num> {
    if points.is_empty() {
        return Vector3::zeros();
    }
    let sum = points.iter().fold(Vector3::zeros(), |acc, &p| acc + p);
    sum / (points.len() as Num)
}

fn is_small_transform(sim: &Similarity3, eps: Num) -> bool {
    // 1) scale close to 1.0
    let ds = (sim.scale - 1.0).abs();
    // 2) translation is small
    let dt = sim.translation.norm();
    // 3) rotation angle is small
    let angle = sim.rotation.angle();

    ds < eps && dt < eps && angle < eps
}

/// Compose two Similarity3 transforms: sim1 followed by sim2.
fn compose_similarity(sim1: &Similarity3, sim2: &Similarity3) -> Similarity3 {
    // If we denote:
    //   x' = sim1(x) = scale1 * (rot1 * x) + trans1
    //   x''= sim2(x')= scale2 * (rot2 * x') + trans2
    // Then compose(sim2, sim1)(x) = x''.
    //
    // The result is effectively:
    //   scale = scale2 * scale1
    //   rot   = rot2 * rot1
    //   trans = rot2 * (scale2 * trans1) + trans2
    //   (plus the scale2 factor on trans1 if we want uniform scaling of the translation).
    //
    // However, be aware that scaling the translation is somewhat model-dependent.
    // The typical approach is to apply scale2 to the translation from the first transform
    // if the scale is uniform. That is:
    //   trans = trans2 + scale2 * (rot2 * trans1)
    // because the second transform's scale2 and rotation2 also affect the translation of the first.

    Similarity3 {
        scale: sim2.scale * sim1.scale,
        rotation: sim2.rotation * sim1.rotation,
        translation: sim2.rotation * (sim2.scale * sim1.translation) + sim2.translation,
    }
}

/// Apply the Similarity3 to an entire cross-section (vector of 3D points).
fn transform_cross_section(cs: &mut CrossSection, sim: &Similarity3) {
    for p in cs.iter_mut() {
        *p = sim.transform_point(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    // #[test]
    // fn test_register_similarity_transform() {
    //     // Let's build a small synthetic example:
    //     // Source points in a cross-section
    //     let mut source = vec![
    //         Vector3::new(1.0, 1.0, 1.0),
    //         Vector3::new(2.0, 1.0, 1.0),
    //         Vector3::new(1.0, 2.0, 1.0),
    //         Vector3::new(2.0, 2.0, 1.0),
    //         Vector3::new(3.0, 2.0, 1.0),
    //         Vector3::new(4.0, 2.0, 1.0),
    //         Vector3::new(5.0, 2.0, 1.0),
    //         Vector3::new(6.0, 2.0, 1.0),
    //         Vector3::new(7.0, 2.0, 1.0),
    //     ];

    //     let scale = 1.0;
    //     // let angle_rad = std::f64::consts::FRAC_PI_8;
    //     // let angle_rad = 0.0;
    //     // let rot = UnitQuaternion::from_euler_angles(0.0, 0.0, angle_rad);
    //     let rot = UnitQuaternion::identity();
    //     let tx = Vector3::new(0.0, 1.0, 0.0);

    //     let target: Vec<Vector3<Num>> = source.iter().map(|p| scale * (rot * p) + tx).collect();

    //     println!("pre icp");
    //     let sim = icp_similarity_transform(&mut source, &target, 100, 1e-10);
    //     println!("\nresult {:?}", sim);

    //     // Check that the estimated transform is close to what we applied
    //     assert!(
    //         (sim.scale - scale).abs() < 1e-7,
    //         "Scale mismatch {}",
    //         (sim.scale - scale)
    //     );
    //     assert!(
    //         (sim.rotation.angle_to(&rot)).abs() < 1e-7,
    //         "Rotation mismatch"
    //     );
    //     assert!((sim.translation - tx).norm() < 1e-7, "Translation mismatch");
    // }
}
