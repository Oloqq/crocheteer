use std::f32::consts::PI;

use crate::{
    common::*,
    flow::Flow,
    plushie::{animation::centroid::Centroids, nodes::Nodes, Plushie},
};

use super::hook::Hook;

type Error = String;

// TODO make it return just nodes and edges
pub fn from_flow(mut flow: impl Flow) -> Result<Plushie, Error> {
    let fasten_off = true;

    let first = flow.next().ok_or("Flow empty")?;
    let mut hook = Hook::start_with(&first)?;
    while let Some(action) = flow.next() {
        hook.perform(&action)?;
    }
    let result = hook.finish();

    let constraints = match fasten_off {
        true => vec![V::zeros(), V::new(0.1, 0.1, 0.1)],
        false => vec![V::zeros()],
    };

    Ok(Plushie {
        nodes: Nodes::new(result.nodes, constraints),
        edges: result.edges,
        params: Default::default(),
        centroids: Centroids::new(2, result.approximate_height),
        stuffing: crate::plushie::Stuffing::Centroids,
    })
}

#[allow(unused)]
fn ring(nodes: usize, y: f32, desired_stitch_distance: f32) -> Vec<Point> {
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

    mod for_refactor {
        use crate::flow::actions::Action::*;
        use crate::flow::simple_flow::SimpleFlow;
        use crate::{
            pattern::{Pattern, Stitch},
            plushie::{params::Params, Plushie},
        };

        use pretty_assertions::assert_eq;
        #[test]
        fn test_from_pattern_1() {
            let p = {
                use Stitch::Sc;
                Pattern {
                    starting_circle: 4,
                    fasten_off: true,
                    rounds: vec![vec![Sc, Sc, Sc, Sc]],
                    simulation_config: Params::default(),
                }
            };
            let f = SimpleFlow::new(vec![MR(4), Sc, Sc, Sc, Sc]);
            let plushie_pattern = Plushie::from_pattern(&p);
            assert_eq!(plushie_pattern.nodes.len(), 10);
            assert_eq!(
                plushie_pattern.edges,
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
            let plushie_flow = Plushie::from_flow(f).unwrap();
            assert_eq!(plushie_flow.nodes.len(), plushie_pattern.nodes.len());
            assert_eq!(plushie_flow.edges, plushie_pattern.edges);
        }

        #[test]
        #[ignore = "need to fill"]
        fn test_from_pattern_no_fasten_off() {
            let p = {
                use Stitch::Sc;
                Pattern {
                    starting_circle: 4,
                    fasten_off: false,
                    rounds: vec![vec![Sc, Sc, Sc, Sc]],
                    simulation_config: Params::default(),
                }
            };
            let f = SimpleFlow::new(vec![]);

            let plushie_pattern = Plushie::from_pattern(&p);
            assert_eq!(plushie_pattern.nodes.len(), 9);
            assert_eq!(
                plushie_pattern.edges,
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
            let plushie_flow = Plushie::from_flow(f).unwrap();
            assert_eq!(plushie_flow.nodes.len(), plushie_pattern.nodes.len());
            assert_eq!(plushie_flow.edges, plushie_pattern.edges);
        }

        #[test]
        #[ignore = "need to fill"]
        fn test_from_pattern_increase_decrese() {
            let p = {
                use Stitch::*;
                Pattern {
                    starting_circle: 4,
                    fasten_off: true,
                    rounds: vec![vec![Sc, Inc, Sc, Sc], vec![Sc, Dec, Sc, Sc]],
                    simulation_config: Params::default(),
                }
            };
            let f = SimpleFlow::new(vec![]);

            let plushie_pattern = Plushie::from_pattern(&p);
            assert_eq!(plushie_pattern.nodes.len(), 15);
            assert_eq!(
                plushie_pattern.edges,
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
            let plushie_flow = Plushie::from_flow(f).unwrap();
            assert_eq!(plushie_flow.nodes.len(), plushie_pattern.nodes.len());
            assert_eq!(plushie_flow.edges, plushie_pattern.edges);
        }

        #[test]
        #[ignore = "need to fill"]
        fn from_genetic_mutant_1() {
            let p = {
                use Stitch::*;
                Pattern {
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
                    simulation_config: Params::default(),
                }
            };
            let f = SimpleFlow::new(vec![]);

            let pl = Plushie::from_pattern(&p);
            assert_eq!(pl.nodes.len(), 22);
            // pl.animate();
            let plushie_flow = Plushie::from_flow(f).unwrap();
            assert_eq!(plushie_flow.nodes.len(), pl.nodes.len());
            assert_eq!(plushie_flow.edges, pl.edges);
        }

        #[test]
        #[ignore = "need to fill"]
        fn from_genetic_mutant_2() {
            let p = {
                use Stitch::*;
                Pattern {
                    starting_circle: 6,
                    fasten_off: true,
                    rounds: vec![vec![Dec, Dec, Dec]],
                    simulation_config: Params::default(),
                }
            };
            let f = SimpleFlow::new(vec![]);

            let pl = Plushie::from_pattern(&p);
            assert_eq!(pl.nodes.len(), 11);
            // pl.animate();
            let plushie_flow = Plushie::from_flow(f).unwrap();
            assert_eq!(plushie_flow.nodes.len(), pl.nodes.len());
            assert_eq!(plushie_flow.edges, pl.edges);
        }

        #[test]
        #[ignore = "need to fill"]
        fn from_genetic_mutant_3() {
            let p = {
                use Stitch::*;
                Pattern {
                    starting_circle: 6,
                    fasten_off: true,
                    rounds: vec![vec![Dec, Dec, Dec], vec![Sc, Sc, Inc]],
                    simulation_config: Params::default(),
                }
            };
            let f = SimpleFlow::new(vec![]);

            let pl = Plushie::from_pattern(&p);
            assert_eq!(pl.nodes.len(), 15);
            // pl.animate();
            let plushie_flow = Plushie::from_flow(f).unwrap();
            assert_eq!(plushie_flow.nodes.len(), pl.nodes.len());
            assert_eq!(plushie_flow.edges, pl.edges);
        }
    }
}
