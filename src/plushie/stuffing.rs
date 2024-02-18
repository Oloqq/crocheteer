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

    fn ideal_radius(&self, i: usize, desired_stitch_distance: f32) -> f32 {
        let circumference = self.anchors[i] as f32 * desired_stitch_distance;
        circumference / (2.0 * PI)
    }

    fn next_start(&self, current: usize, points: &Vec<Point>) -> usize {
        *self.start.get(current + 1).unwrap_or(&points.len())
    }

    pub fn update_round_centers(&mut self, points: &Vec<Point>) {
        let mut center = V::zeros();
        let mut node_cnt = 0; // this can be calculated from rounds.start right?
        let mut round = 0;
        let mut next_round_start = self.next_start(round, points);

        for (i, point) in points.iter().enumerate().skip(*self.start.first().unwrap()) {
            if i == next_round_start {
                assert!(node_cnt == self.start[round + 1] - self.start[round]); // TODO remove checks and node_cnt
                assert!(node_cnt == self.anchors[round]);
                self.center[round] = center / self.anchors[round] as f32;
                round += 1;
                center = V::zeros();
                node_cnt = 0;
                next_round_start = self.next_start(round, points);

                assert!(next_round_start > i);
            }
            center += point.coords;
            node_cnt += 1;
        }
        self.center[round] = center / node_cnt as f32;
    }

    pub fn push_rounds_offcenter(
        &self,
        points: &Vec<Point>,
        desired_stitch_distance: f32,
        displacement: &mut Vec<V>,
    ) {
        let mut round = 0;
        let mut center = self.center[round];
        let mut radius = self.ideal_radius(round, desired_stitch_distance);
        let mut next_round_start = self.next_start(round, points);

        for (i, point) in points
            .iter()
            .enumerate()
            .take(*self.start.last().unwrap())
            .skip(*self.start.first().unwrap())
        {
            if i == next_round_start {
                round += 1;
                next_round_start = self.next_start(round, points);
                center = self.center[round];
                radius = self.ideal_radius(round, desired_stitch_distance);
            }

            // if the round was not stressed by anything else, pushing offcenter
            // would create an approximation of a circle of circumference = N * link distance
            // where N is the number of stitches in a round,
            // and link distance means unstressed distance between stitches
            // points should be pushed out until they reach radius of that circle
            // does pulling in make sense?

            let tmp = push_offcenter(point, &center, radius);
            displacement[i] += tmp;
        }
    }
}

pub fn push_offcenter(point: &Point, center: &V, radius: f32) -> V {
    let diff = point.coords - center;
    let too_close = radius - diff.magnitude();

    if too_close < 0.0 {
        V::zeros()
    } else {
        diff.normalize() * (too_close / 4.0).powi(2)
    }
}

pub fn per_round_stuffing(
    rounds: &mut Rounds,
    points: &Vec<Point>,
    desired_stitch_distance: f32,
    displacement: &mut Vec<V>,
) {
    rounds.update_round_centers(points);
    rounds.push_rounds_offcenter(points, desired_stitch_distance, displacement);
}

#[cfg(test)]
mod tests {
    use crate::{
        pattern::{builder::PatternBuilder, Stitch},
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
    fn test_per_round_stuffing() {
        let points = vec![
            Point::new(-2.0, 0.0, -1.0),
            Point::new(-2.0, 0.0, 2.0),
            Point::new(2.0, 0.0, 2.0),
            Point::new(2.0, 0.0, -1.0),
        ];
        let mut displacement = vec![V::new(0.0, 0.0, 0.0); 4];
        let rounds = Rounds {
            start: vec![0, 4],
            anchors: vec![4],
            center: vec![V::new(0.0, 0.0, 0.5)],
        };
        rounds.push_rounds_offcenter(&points, 1000000.0, &mut displacement);

        for d in displacement {
            assert!(d.magnitude() > 0.0);
        }
    }
}
