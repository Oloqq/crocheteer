use std::collections::HashMap;

use super::{utils::*, Color, Edges, Hook, Moment, Queue};

impl Hook {
    pub fn start_with(action: &Action) -> Result<Self, HookError> {
        let color = (255, 0, 255);
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
                    part_start: 0,
                    parts: vec![],
                    labels: HashMap::new(),
                    at_junction: false,
                    override_previous_stitch: None,
                    color,
                    colors,
                    last: None,
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
                    part_start: 0,
                    parts: vec![],
                    labels: HashMap::new(),
                    at_junction: false,
                    override_previous_stitch: None,
                    color,
                    colors,
                    last: None,
                })
            }
            _ => Err(HookError::BadStarter),
        }
    }
}