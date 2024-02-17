use std::f32::consts::PI;

use crate::common::*;
use crate::pattern::stitches::count_anchors_produced;
use crate::pattern::{Pattern, Stitch};
use crate::plushie::Stuffing;

use super::Plushie;

impl Plushie {
    pub fn from_pattern(pattern: Pattern) -> Self {
        const FIXED_NUM: usize = 2;
        const START_EDGE: usize = 0;
        const END_EDGE: usize = 1;

        let start = Point::origin();
        let end = Point::new(0.0, pattern.rounds.len() as f32 + 1.0, 0.0);

        let mut points = vec![start, end];
        let mut edges: Vec<Vec<usize>> = vec![vec![], vec![]];
        let mut height: f32 = 0.0;

        points.append(&mut ring(pattern.starting_circle, height));

        for i in FIXED_NUM..pattern.starting_circle + FIXED_NUM {
            edges[START_EDGE].push(i);
            edges.push(vec![i + 1]);
        }

        let mut anchor = FIXED_NUM;
        let mut current = FIXED_NUM + pattern.starting_circle;
        let mut round_starts: Vec<usize> = vec![];
        let mut round_counts: Vec<usize> = vec![];
        for round in pattern.rounds {
            round_starts.push(points.len());
            round_counts.push(count_anchors_produced(&round));
            height += 1.0;
            let current_at_round_start = current;
            for stitch in round {
                match stitch {
                    Stitch::Sc => {
                        edges[anchor].push(current);
                        edges.push(vec![current + 1]);
                        anchor += 1;
                        current += 1;
                    }
                    Stitch::Inc => {
                        edges[anchor].push(current);
                        edges[anchor].push(current + 1);
                        edges.push(vec![current + 1]);
                        edges.push(vec![current + 2]);
                        current += 2;
                        anchor += 1;
                    }
                    Stitch::Dec => {
                        edges[anchor].push(current);
                        edges[anchor + 1].push(current);
                        edges.push(vec![current + 1]);
                        current += 1;
                        anchor += 2;
                    }
                }
            }
            points.append(&mut ring(current - current_at_round_start, height));
        }

        *edges.last_mut().unwrap() = vec![];
        edges[END_EDGE] = (points.len() - pattern.ending_circle..points.len()).collect();

        Plushie {
            fixed_num: FIXED_NUM,
            points,
            edges,
            desired_stitch_distance: 1.0,
            stuffing: Stuffing::PerRound,
            round_starts,
            round_counts,
            round_centers: vec![],
            // gravity: 0.0000001,
            _gravity: 0.0,
        }
    }
}

pub fn ring(nodes: usize, y: f32) -> Vec<Point> {
    assert!(nodes >= 3);
    const RADIUS: f32 = 1.0;

    let interval = 2.0 * PI / nodes as f32;
    let mut result: Vec<Point> = vec![];

    for i in 0..nodes {
        let rads = interval * i as f32;
        let x = rads.cos() * RADIUS;
        let z = rads.sin() * RADIUS;
        let point = Point::new(x, y, z);
        result.push(point);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_pattern() {
        use Stitch::Sc;
        let p = Pattern {
            starting_circle: 4,
            ending_circle: 4,
            rounds: vec![vec![Sc, Sc, Sc, Sc]],
        };
        let plushie = Plushie::from_pattern(p);
        assert_eq!(plushie.fixed_num, 2);
        assert_eq!(plushie.points.len(), 10);
        assert_eq!(
            plushie.edges,
            vec![
                // 0 ->
                vec![2, 3, 4, 5],
                // 1 ->
                vec![6, 7, 8, 9],
                // 2 ->
                vec![3, 6],
                // 3 ->
                vec![4, 7],
                // 4 ->
                vec![5, 8],
                // 5 ->
                vec![6, 9],
                // 6 ->
                vec![7],
                // 7 ->
                vec![8],
                // 8 ->
                vec![9],
                // 9 ->
                vec![],
            ]
        );
    }

    #[test]
    fn test_from_pattern_increase_decrese() {
        use Stitch::*;
        let p = Pattern {
            starting_circle: 4,
            ending_circle: 4,
            rounds: vec![vec![Sc, Inc, Sc, Sc], vec![Sc, Dec, Sc, Sc]],
        };
        let plushie = Plushie::from_pattern(p);
        assert_eq!(plushie.fixed_num, 2);
        assert_eq!(plushie.points.len(), 15);
        assert_eq!(
            plushie.edges,
            vec![
                /* 0 -> */ vec![2, 3, 4, 5],
                /* 1 -> */ vec![11, 12, 13, 14],
                /* 2 -> */ vec![3, 6],
                /* 3 -> */ vec![4, 7, 8],
                /* 4 -> */ vec![5, 9],
                /* 5 -> */ vec![6, 10],
                /* 6 -> */ vec![7, 11],
                /* 7 -> */ vec![8, 12],
                /* 8 -> */ vec![9, 12],
                /* 9 -> */ vec![10, 13],
                /* 10 -> */ vec![11, 14],
                /* 11 -> */ vec![12],
                /* 12 -> */ vec![13],
                /* 13 -> */ vec![14],
                /* 14 -> */ vec![],
            ]
        );
    }
}
