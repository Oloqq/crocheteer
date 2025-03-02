use std::collections::HashMap;

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
            MRConfigurable(x, label) => {
                let mut hook = Self::start_with(&MR(*x), color)?;
                assert_eq!(hook.peculiar.get(&0), Some(&Peculiarity::Locked));
                hook.mark_to_node.insert(label.clone(), 0);

                // TEMP
                // hook.peculiar.insert(40, Peculiarity::Locked);

                Ok(hook)
            }
            MR(x) => {
                let edges = {
                    let mut tmp = Edges::new();
                    tmp.grow();
                    tmp
                };
                let this_will_be_overwritten_how_do_i_design_it_readably_bruh_please_tell_me_via_pr_thanks =
                    Moment {
                        round_count: 0,
                        round_left: 0,
                        anchors: Queue::new(),
                        cursor: 0,
                        working_on: WorkingLoops::Both,
                        limb_ownerhip: 0,
                    };

                let mut result = Self {
                    edges,
                    peculiar: HashMap::new(),
                    now: this_will_be_overwritten_how_do_i_design_it_readably_bruh_please_tell_me_via_pr_thanks,
                    parents: vec![],
                    labels: HashMap::new(),
                    override_previous_stitch: None,
                    color,
                    colors: vec![],
                    last_stitch: None,
                    last_mark: None,
                    leniency: Leniency::NoMercy,
                    mark_to_node: HashMap::new(),
                    part_limits: vec![],
                    mr_count: 0,
                };
                result.magic_ring(*x);
                Ok(result)
            }
            Ch(_x) => {
                todo!("Chain starter requires locking a single coordinate");
                // let edges: Vec<Vec<usize>> = {
                //     let mut edges: Vec<Vec<usize>> = (1..*x).map(|i| vec![i]).collect();
                //     edges.push(vec![]);
                //     edges.push(vec![]);
                //     edges
                // };

                // let mut peculiar = HashMap::new();
                // for i in 0..*x {
                //     peculiar.insert(i, Peculiarity::Locked(V::new(1.0, 0.0, 1.0)));
                // }
                // let colors: Vec<Color> = (0..*x).map(|_| color).collect();

                // Ok(Self {
                //     edges: Edges::from(edges),
                //     peculiar,
                //     now: Moment {
                //         round_count: 0,
                //         round_left: *x,
                //         anchors: Queue::from_iter(0..*x),
                //         cursor: *x,
                //         working_on: WorkingLoops::Both,
                //     },
                //     round_spans: vec![(0, *x - 1)],
                //     parents: vec![None; *x],
                //     labels: HashMap::new(),
                //     override_previous_stitch: None,
                //     color,
                //     colors,
                //     last_stitch: None,
                //     last_mark: None,
                //     leniency: Leniency::NoMercy,
                // })
            }
            _ => Err(HookError::BadStarter),
        }
    }
}
