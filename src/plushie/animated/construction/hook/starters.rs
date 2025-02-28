use std::collections::HashMap;

use colors::Color;

use super::{leniency::Leniency, utils::*, Edges, Hook, Moment, Queue};
use crate::acl::Flow;

const DEFAULT_COLOR: colors::Color = (255, 0, 255);

impl Hook {
    pub fn with_leniency(action: &Action, leniency: &Leniency) -> Result<Self, HookError> {
        let mut res = Self::start_with(action, DEFAULT_COLOR)?;
        res.leniency = leniency.clone();
        Ok(res)
    }

    pub fn from_starting_sequence(flow: &mut impl Flow) -> Result<Self, HookError> {
        let mut action = flow.next().unwrap();
        let mut color = DEFAULT_COLOR;
        if let Color(c) = action {
            color = c;
            action = flow.next().unwrap();
        }
        Self::start_with(&action, color)
    }

    pub fn start_with(action: &Action, color: colors::Color) -> Result<Self, HookError> {
        match action {
            MR(x) => {
                let edges: Vec<Vec<usize>> = {
                    let edges_from_root: Vec<usize> = (1..=*x).collect();
                    let ring_edges = (2..=*x).map(|i| vec![i]);
                    let mut edges = vec![edges_from_root];
                    edges.extend(ring_edges);
                    edges.push(vec![]);
                    edges.push(vec![]);
                    edges
                };
                let parents: Vec<Option<usize>> = {
                    let mut tmp = vec![None];
                    tmp.append(&mut vec![Some(0); *x]);
                    tmp
                };
                let colors: Vec<Color> = (0..=*x).map(|_| color).collect();

                Ok(Self {
                    edges: Edges::from(edges),
                    peculiar: HashMap::from([(0, Peculiarity::Root)]),
                    now: Moment {
                        round_count: 0,
                        round_left: *x,
                        anchors: Queue::from_iter(1..=*x), // 1 because root takes index 0
                        cursor: x + 1,                     // + 1 because root takes index 0
                        working_on: WorkingLoops::Both,
                    },
                    round_spans: vec![(0, *x)],
                    parents,
                    labels: HashMap::new(),
                    override_previous_stitch: None,
                    color,
                    colors,
                    last_stitch: None,
                    last_mark: None,
                    leniency: Leniency::NoMercy,
                })
            }
            Ch(x) => {
                let edges: Vec<Vec<usize>> = {
                    let mut edges: Vec<Vec<usize>> = (1..*x).map(|i| vec![i]).collect();
                    edges.push(vec![]);
                    edges.push(vec![]);
                    edges
                };

                let mut peculiar = HashMap::new();
                for i in 0..*x {
                    peculiar.insert(i, Peculiarity::Constrained(V::new(1.0, 0.0, 1.0)));
                }
                let colors: Vec<Color> = (0..*x).map(|_| color).collect();

                Ok(Self {
                    edges: Edges::from(edges),
                    peculiar,
                    now: Moment {
                        round_count: 0,
                        round_left: *x,
                        anchors: Queue::from_iter(0..*x),
                        cursor: *x,
                        working_on: WorkingLoops::Both,
                    },
                    round_spans: vec![(0, *x - 1)],
                    parents: vec![None; *x],
                    labels: HashMap::new(),
                    override_previous_stitch: None,
                    color,
                    colors,
                    last_stitch: None,
                    last_mark: None,
                    leniency: Leniency::NoMercy,
                })
            }
            _ => Err(HookError::BadStarter),
        }
    }
}
