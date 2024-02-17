use crate::common::*;
use serde_derive::Serialize;
use std::f32::consts::PI;

#[derive(Clone, Serialize)]
pub struct Rounds {
    start: Vec<usize>,
    anchors: Vec<usize>,
    center: Vec<V>,
}

impl Rounds {
    pub fn new(round_starts: Vec<usize>, round_counts: Vec<usize>) -> Self {
        Rounds {
            center: vec![V::new(0.0, 0.0, 0.0); round_counts.len()],
            start: round_starts,
            anchors: round_counts,
        }
    }

    pub fn ideal_radius(&self, i: usize, desired_stitch_distance: f32) -> f32 {
        let circumference = self.anchors[i] as f32 * desired_stitch_distance;
        circumference / (2.0 * PI)
    }

    pub fn update_round_centers(&mut self, points: &Vec<Point>) {
        let first_round_start = *self.start.first().unwrap();
        let mut center = V::zeros();
        let mut node_cnt = 0; // this can be calculated from rounds.start right?
        let mut round = 0;
        let mut next_round_start = *self.start.get(round + 1).unwrap_or(&points.len());

        for (i, point) in points.iter().enumerate().skip(first_round_start) {
            if i == next_round_start {
                assert!(node_cnt == self.start[round + 1] - self.start[round]); // TODO remove checks and node_cnt
                assert!(node_cnt == self.anchors[round]);
                self.center[round] = center / self.anchors[round] as f32;
                round += 1;
                center = V::zeros();
                node_cnt = 0;
                next_round_start = *self.start.get(round + 1).unwrap_or(&points.len());

                assert!(next_round_start > i);
            }
            center += point.coords;
            node_cnt += 1;
        }
        self.center[round] = center / node_cnt as f32;
    }
}

pub fn ideal_radius(stitch_count: usize, desired_stitch_distance: f32) -> f32 {
    let circumference = stitch_count as f32 * desired_stitch_distance;
    circumference / (2.0 * PI)
}

pub fn push_offcenter(point: &Point, center: &V, radius: f32) -> V {
    let diff = point.coords - center;
    let diff_len = diff.magnitude();
    // println!("radius: {radius} diff_len {diff_len}");

    if diff_len >= radius {
        V::zeros()
    } else {
        // println!("yeee");
        diff.normalize()
    }
}

pub fn push_rounds_offcenter(
    centers: &Vec<V>,
    round_starts: &Vec<usize>,
    round_counts: &Vec<usize>,
    points: &Vec<Point>,
    desired_stitch_distance: f32,
    displacement: &mut Vec<V>,
) {
    let mut centers = centers.iter().peekable();
    let first_round_start = round_starts.first().expect("Expected at least one round");
    let pointslen = &[points.len()];
    let mut rounds = round_starts.iter().chain(pointslen);
    let mut round_counts = round_counts.iter();
    let mut next_round_start: usize = 0;
    let mut current_round_count: usize = 0;
    let mut current = *centers.peek().unwrap();
    let last_round_start = *round_starts.last().unwrap();
    for (i, point) in points
        .iter()
        .enumerate()
        .take(last_round_start)
        .skip(*first_round_start)
    {
        // println!("i {i}");
        if i == next_round_start {
            next_round_start = *rounds.next().unwrap();
            current_round_count = *round_counts.next().unwrap_or(&points.len());
            current = centers.next().unwrap();
        }

        // if the round was not stressed by anything else, pushing offcenter
        // would create an approximation of a circle of circumference = N * link distance
        // where N is the number of stitches in a round,
        // and link distance means unstressed distance between stitches
        // points should be pushed out until they reach radius of that circle
        // does pulling in make sense?

        let radius = ideal_radius(current_round_count, desired_stitch_distance);
        let tmp = push_offcenter(point, current, radius);
        // println!("{tmp:?} {current_round_count:?} {point:?} {current:?} {radius:?}");
        displacement[i] += tmp;
    }
}

pub fn per_round_stuffing(
    rounds: &mut Rounds,
    points: &Vec<Point>,
    desired_stitch_distance: f32,
    displacement: &mut Vec<V>,
) {
    rounds.update_round_centers(points);
    // calculate_round_centers(rounds, points);
    // let centers = &rounds.center;
    // println!("{round_counts:?}");
    // println!("{centers:?}");
    push_rounds_offcenter(
        &rounds.center,
        &rounds.start,
        &rounds.anchors,
        points,
        desired_stitch_distance,
        displacement,
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        pattern::{construction::PatternBuilder, Pattern, Stitch},
        plushie::Plushie,
    };
    use Stitch::*;

    use super::*;

    #[test]
    fn test_push_offcenter() {
        let point = Point::new(10.0, 0.0, 10.0);
        let center = V::new(0.0, 0.0, 0.0);
        let res = push_offcenter(&point, &center, 3.0);
        assert_eq!(res.magnitude(), 0.0);

        let res = push_offcenter(&point, &center, 100.0);
        assert_ne!(res.magnitude(), 0.0);
        assert!(res.x > 0.0);
        assert_eq!(res.y, 0.0);
        assert!(res.z > 0.0);
    }

    #[test]
    fn test_calculate_round_centers_on_origin() {
        let points = vec![
            Point::new(-1.0, 0.0, -1.0),
            Point::new(-1.0, 0.0, 1.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(-1.0, 0.0, -1.0),
            Point::new(-1.0, 0.0, 1.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(1.0, 0.0, -1.0),
        ];
        let mut rounds = Rounds::new(vec![0, 4], vec![4, 4]);
        rounds.update_round_centers(&points);
        assert_eq!(
            rounds.center,
            vec![V::new(0.0, 0.0, 0.0), V::new(0.0, 0.0, 0.0)]
        );
    }

    #[test]
    fn test_calculate_round_centers() {
        let points = vec![
            Point::new(-2.0, 0.0, -1.0),
            Point::new(-2.0, 0.0, 2.0),
            Point::new(2.0, 0.0, 2.0),
            Point::new(2.0, 0.0, -1.0),
        ];

        let mut rounds = Rounds::new(vec![0], vec![4]);
        rounds.update_round_centers(&points);
        assert_eq!(rounds.center, vec![V::new(0.0, 0.0, 0.5)]);
    }

    #[test]
    fn test_rounds() {
        let pattern = PatternBuilder::new(6)
            .round_like(&vec![Inc])
            .full_rounds(1)
            .round_like(&vec![Dec])
            .build()
            .unwrap();
        let plushie = Plushie::from_pattern(pattern);
        let mut rounds = plushie.rounds;

        assert_eq!(rounds.center, vec![V::new(0.0, 0.0, 0.0); 3]);
        assert_eq!(rounds.start, vec![8, 20, 32]);
        assert_eq!(rounds.anchors, vec![12, 12, 6]);

        rounds.update_round_centers(&plushie.points);
    }

    #[test]
    fn test_ideal_radius() {
        assert_eq!(ideal_radius(4, 1.0), 4.0 / (2.0 * PI));
    }

    // #[test]
    // fn test_per_round_stuffing() {
    //     let points = vec![
    //         Point::new(-2.0, 0.0, -1.0),
    //         Point::new(-2.0, 0.0, 2.0),
    //         Point::new(2.0, 0.0, 2.0),
    //         Point::new(2.0, 0.0, -1.0),
    //     ];
    //     let centers = vec![V::new(0.0, 0.0, 0.5)];
    //     let mut displacement = vec![V::new(0.0, 0.0, 0.0); 4];
    //     let round_starts: Vec<usize> = vec![0, 5];
    //     let round_counts: Vec<usize> = vec![4];
    //     push_rounds_offcenter(
    //         centers,
    //         &round_starts,
    //         &round_counts,
    //         &points,
    //         4.0,
    //         &mut displacement,
    //     );
    //     for d in displacement {
    //         assert!(d.magnitude() > 0.0);
    //     }
    // }
}
