pub mod hook;
mod hook_result;

use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
};

use colors::Color;

use self::hook::Hook;
pub use self::hook_result::{Peculiarity, PointsOnPushPlane};
use super::{centroid::Centroids, nodes::Nodes, Initializer, Params, Plushie};
use crate::{
    acl::{pest_parser::Pattern, Flow},
    common::*,
    sanity,
};

impl Plushie {
    fn for_one_by_one(
        params: Params,
        peculiarities: HashMap<usize, Peculiarity>,
        colors: Vec<Color>,
        edges: Vec<Vec<usize>>,
    ) -> Self {
        let mut displacement = Vec::with_capacity(edges.len());
        displacement.push(V::zeros());
        Self {
            nodes: Nodes::new(vec![Point::new(0.0, 0.0, 0.0)], peculiarities, colors),
            edges: vec![vec![]],
            edges_goal: edges,
            params,
            centroids: Centroids::new(0, 0.0),
            displacement,
            force_node_construction_timer: 0.0,
            // initializing with INF so it won't come as relaxed before first step by accident
            last_total_displacement: V::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            perf: vec![],
        }
    }

    fn with_initial_positions(
        params: Params,
        peculiarities: HashMap<usize, Peculiarity>,
        colors: Vec<Color>,
        edges: Vec<Vec<usize>>,
        nodes: Vec<Point>,
        height: f32,
    ) -> Self {
        let edges: Vec<Vec<usize>> = edges.into();
        let nodes = Nodes::new(nodes, peculiarities, colors);
        Self {
            displacement: vec![V::zeros(); edges.len()],
            edges_goal: edges.clone(),
            edges,
            centroids: Centroids::new(params.centroids.number, height),
            params,
            nodes,
            force_node_construction_timer: 0.0,
            // initializing with INF so it won't come as relaxed before first step by accident
            last_total_displacement: V::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            perf: vec![],
        }
    }

    pub fn from_flow(flow: impl Flow, params: Params) -> Result<Self, String> {
        let hook_result = Hook::parse(flow, &params.hook_leniency)?;

        Ok(match params.initializer {
            Initializer::OneByOne(_) => Plushie::for_one_by_one(
                params,
                hook_result.peculiarities,
                hook_result.colors,
                hook_result.edges.into(),
            ),
            Initializer::Cylinder => {
                let (nodes, highest) = arrange_cylinder(hook_result.round_spans);
                assert_eq!(nodes.len(), hook_result.colors.len());

                Plushie::with_initial_positions(
                    params,
                    hook_result.peculiarities,
                    hook_result.colors,
                    hook_result.edges.into(),
                    nodes,
                    highest,
                )
            }
        })
    }

    pub fn parse(src: &str) -> Result<Self, String> {
        let pattern = Pattern::parse(src)?;
        let mut params = Params::default();
        let update_errors = params.update(&pattern.parameters);
        if update_errors.len() > 0 {
            return Err(update_errors[0].clone());
        }

        if !params.reflect_locked {
            // TODO ensure at least one point is locked
        }

        Ok(Self::from_flow(pattern, params)?)
    }

    pub fn _position_based_on(&mut self, _other: &Self) {
        todo!()
    }
}

fn arrange_cylinder(round_spans: Vec<(usize, usize)>) -> (Vec<Point>, f32) {
    fn is_uniq(vec: &Vec<Point>) -> bool {
        let uniq = vec
            .into_iter()
            .map(|v| format!("{:?}", v.coords))
            .collect::<HashSet<_>>();
        uniq.len() == vec.len()
    }

    let mut y = 0.0;
    let mut nodes = vec![];

    let mut round_spans = round_spans.into_iter();
    let mr_round = round_spans.next().unwrap();
    assert_eq!(mr_round.0, 0);
    nodes.append(&mut vec![Point::new(0.0, 0.0, 0.0)]);
    nodes.append(&mut ring(mr_round.1 - mr_round.0, y, 1.0));
    y += 0.7;

    for (from, to) in round_spans {
        let count = to - from + 1;
        nodes.append(&mut ring(count, y, 1.0));
        y += 0.7;
    }

    sanity!(assert!(is_uniq(&nodes), "hook created duplicate positions"));

    (nodes, y)
}

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
    use super::*;

    #[test]
    fn test_arrange_cylinder() {
        let rounds_spans = vec![(0, 4)];
        let (res, _) = arrange_cylinder(rounds_spans);
        assert_eq!(res.len(), 5);

        let rounds_spans = vec![(0, 4), (5, 8)];
        let (res, _) = arrange_cylinder(rounds_spans);
        assert_eq!(res.len(), 9);
    }
}
