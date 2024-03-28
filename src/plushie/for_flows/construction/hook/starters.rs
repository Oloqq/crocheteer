use std::collections::HashMap;

use super::{utils::*, Color, Hook, Moment};

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
                    edges
                };
                let parents: Vec<Option<usize>> = {
                    let mut tmp = vec![None];
                    tmp.append(&mut vec![Some(0); *x]);
                    tmp
                };
                let colors: Vec<Color> = (0..=*x).map(|_| color).collect();

                Ok(Self {
                    edges,
                    peculiar: HashMap::from([(0, Peculiarity::Root)]),
                    now: Moment {
                        round_count: 0,
                        round_left: *x,
                        anchor: 1,     // 1 because root takes index 0
                        cursor: x + 1, // + 1 because root takes index 0
                        working_on: WorkingLoops::Both,
                    },
                    round_spans: vec![(0, *x)],
                    fastened_off: false,
                    parents,
                    part_start: 0,
                    parts: vec![],
                    labels: HashMap::new(),
                    at_junction: false,
                    override_previous_stitch: None,
                    color,
                    colors,
                })
            }
            Ch(x) => {
                let edges: Vec<Vec<usize>> = {
                    let mut edges: Vec<Vec<usize>> = (1..*x).map(|i| vec![i]).collect();
                    edges.push(vec![]);
                    edges
                };

                let mut peculiar = HashMap::new();
                for i in 0..*x {
                    peculiar.insert(i, Peculiarity::Constrained(V::new(1.0, 0.0, 1.0)));
                }
                let colors: Vec<Color> = (0..*x).map(|_| color).collect();

                Ok(Self {
                    edges,
                    peculiar,
                    now: Moment {
                        round_count: 0,
                        round_left: *x,
                        anchor: 0,
                        cursor: *x,
                        working_on: WorkingLoops::Both,
                    },
                    round_spans: vec![(0, *x - 1)],
                    fastened_off: false,
                    parents: vec![None; *x],
                    part_start: 0,
                    parts: vec![],
                    labels: HashMap::new(),
                    at_junction: false,
                    override_previous_stitch: None,
                    color,
                    colors,
                })
            }
            _ => Err(HookError::BadStarter),
        }
    }
}
