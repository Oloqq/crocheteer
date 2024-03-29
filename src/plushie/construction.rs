use std::f32::consts::PI;

use crate::common::*;
use crate::pattern::genetic::Genom;
use crate::pattern::stitches::count_anchors_produced;
use crate::pattern::{Pattern, Stitch};
use crate::plushie::per_round_stuffing::RoundsInfo;
use crate::plushie::points::{Points, ROOT_INDEX};
use crate::plushie::Stuffing;

use super::Plushie;

impl Plushie {
    pub fn from_genetic(genom: &Genom) -> Self {
        let pattern = Pattern::from_genom(&genom);
        Self::from_pattern(&pattern)
    }

    pub fn from_pattern(pattern: &Pattern) -> Self {
        const ROOT_NODE: usize = ROOT_INDEX;
        let (fixed_points_num, tip_node): (usize, Option<usize>) = match pattern.fasten_off {
            true => (2, Some(1)),
            false => (1, None),
        };

        let height_per_round = 1.0;
        let desired_stitch_distance = 1.0;

        let start = Point::origin();
        let mut points = vec![start];
        let mut edges: Vec<Vec<usize>> = vec![vec![]];
        let mut height: f32 = 0.0;

        let approximate_height = pattern.rounds.len() as f32 + 1.0;
        if tip_node.is_some() {
            points.push(Point::new(0.0, approximate_height, 0.0));
            edges.push(vec![]);
        }

        points.append(&mut ring(
            pattern.starting_circle,
            height,
            desired_stitch_distance,
        ));

        // edges around root
        for i in fixed_points_num..pattern.starting_circle + fixed_points_num {
            edges[ROOT_NODE].push(i);
            edges.push(vec![i + 1]);
        }

        let mut anchor = fixed_points_num;
        let mut current = fixed_points_num + pattern.starting_circle;
        let mut round_starts: Vec<usize> = vec![];
        let mut round_counts: Vec<usize> = vec![];
        for round in &pattern.rounds {
            round_starts.push(points.len());
            round_counts.push(count_anchors_produced(&round));
            height += height_per_round;
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
            // place the points in 3d space
            points.append(&mut ring(
                current - current_at_round_start,
                height,
                desired_stitch_distance,
            ));
        }

        // delete the connection from last point (the tip) to the next
        *edges.last_mut().unwrap() = vec![];

        // connect the tip
        if let Some(tip) = tip_node {
            let last_round_count = *round_counts.last().unwrap();
            edges[tip] = (points.len() - last_round_count..points.len()).collect();
        }

        let constraints = match pattern.fasten_off {
            true => vec![V::zeros(), V::new(0.1, 0.1, 0.1)],
            false => vec![V::zeros()],
        };

        let centroids = {
            let cnum = pattern.simulation_config.centroids;
            let margin = 1.0;
            let ceiling = approximate_height - margin;
            let floor = margin;
            let spacing = (ceiling - floor) / cnum as f32;
            (0..cnum)
                .map(|i| Point::new(0.0, floor + spacing * i as f32, 0.0))
                .collect()
        };
        let floor = pattern.simulation_config.floor;

        Plushie {
            points: Points::new(points, constraints),
            edges,
            desired_stitch_distance,
            stuffing: Stuffing::Centroids,
            rounds: RoundsInfo::new(round_starts, round_counts),
            gravity: 5e-4,
            acceptable_tension: 0.02,
            max_relaxing_iterations: 100,
            centroids,
            centroid_force: 0.05,
            floor,
        }
    }
}

pub fn ring(nodes: usize, y: f32, desired_stitch_distance: f32) -> Vec<Point> {
    let circumference = (nodes + 1) as f32 * desired_stitch_distance;
    let radius = circumference / (2.0 * PI) / 4.0;

    let interval = 2.0 * PI / nodes as f32;
    let mut result: Vec<Point> = vec![];

    for i in 0..nodes {
        let rads = interval * i as f32;
        let x = rads.cos() * radius;
        let z = rads.sin() * radius;
        let point = Point::new(x, y, z);
        result.push(point);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::plushie::config::SimulationConfig;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_pattern() {
        use Stitch::Sc;
        let p = Pattern {
            starting_circle: 4,
            fasten_off: true,
            rounds: vec![vec![Sc, Sc, Sc, Sc]],
            simulation_config: SimulationConfig::default(),
        };
        let plushie = Plushie::from_pattern(&p);
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
    fn test_from_pattern_no_fasten_off() {
        use Stitch::Sc;
        let p = Pattern {
            starting_circle: 4,
            fasten_off: false,
            rounds: vec![vec![Sc, Sc, Sc, Sc]],
            simulation_config: SimulationConfig::default(),
        };
        let plushie = Plushie::from_pattern(&p);
        assert_eq!(plushie.points.len(), 9);
        assert_eq!(
            plushie.edges,
            vec![
                // 0 ->
                vec![1, 2, 3, 4],
                // 1 ->
                vec![2, 5],
                // 2 ->
                vec![3, 6],
                // 3 ->
                vec![4, 7],
                // 4 ->
                vec![5, 8],
                // 5 ->
                vec![6],
                // 6 ->
                vec![7],
                // 7 ->
                vec![8],
                // 8 ->
                vec![],
            ]
        );
    }

    #[test]
    fn test_from_pattern_increase_decrese() {
        use Stitch::*;
        let p = Pattern {
            starting_circle: 4,
            fasten_off: true,
            rounds: vec![vec![Sc, Inc, Sc, Sc], vec![Sc, Dec, Sc, Sc]],
            simulation_config: SimulationConfig::default(),
        };
        let plushie = Plushie::from_pattern(&p);
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

    #[test]
    fn from_genetic_mutant_1() {
        use Stitch::*;
        let p = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![
                vec![Dec, Dec, Dec],
                vec![Sc, Dec],
                vec![Dec],
                vec![Sc],
                vec![Sc],
                vec![Sc],
                vec![Inc],
                vec![Sc, Inc],
            ],
            simulation_config: SimulationConfig::default(),
        };
        let pl = Plushie::from_pattern(&p);
        assert_eq!(pl.points.len(), 22);
        // pl.animate();
    }

    #[test]
    fn from_genetic_mutant_2() {
        use Stitch::*;
        let p = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![vec![Dec, Dec, Dec]],
            simulation_config: SimulationConfig::default(),
        };
        let pl = Plushie::from_pattern(&p);
        assert_eq!(pl.points.len(), 11);
        // pl.animate();
    }

    #[test]
    fn from_genetic_mutant_3() {
        use Stitch::*;
        let p = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![vec![Dec, Dec, Dec], vec![Sc, Sc, Inc]],
            simulation_config: SimulationConfig::default(),
        };
        let pl = Plushie::from_pattern(&p);
        assert_eq!(pl.points.len(), 15);
        // pl.animate();
    }

    #[test]
    fn from_genetic_mutant_4() {
        // this tests assumes dec is not allowed to overflow
        use Stitch::*;
        let p = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![vec![Dec, Dec, Dec], vec![Sc, Dec], vec![Dec]],
            simulation_config: SimulationConfig::default(),
        };
        assert_eq!(p.rounds.len(), 3);
        let pl = Plushie::from_pattern(&p);
        println!("{:?}", pl.points.as_vec());
        assert_eq!(pl.points[1].y, 4.0);
        assert_eq!(pl.points.len(), 14);
        // pl.animate();
    }
}
