#![allow(unused)]

use Action::*;

use super::{Params, Plushie};
use crate::{
    acl::{actions::Action, ergoflow::ErgoFlow, simple_flow::SimpleFlow},
    common::*,
};

pub fn vase_simple_flow() -> Plushie {
    let mut actions: Vec<Action> = vec![MR(6)];
    actions.append(&mut vec![Inc; 6]);
    let full_round = vec![Sc; 12];
    for _ in 0..6 {
        actions.append(&mut full_round.clone());
    }

    let flow = SimpleFlow::new(actions);
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn pillar_simple_flow() -> Plushie {
    use Action::*;

    use crate::acl::actions::Action;
    let mut actions: Vec<Action> = vec![MR(6)];
    actions.append(&mut vec![Inc; 6]);
    let full_round = vec![Sc; 12];
    for _ in 0..6 {
        actions.append(&mut full_round.clone());
    }
    actions.push(FO);

    let flow = SimpleFlow::new(actions);
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn pillar_blo() -> Plushie {
    use Action::*;

    use crate::acl::actions::Action;
    let mut actions: Vec<Action> = vec![MR(6)];
    actions.append(&mut vec![Inc; 6]);
    let full_round = vec![Sc; 12];
    actions.append(&mut full_round.clone());
    actions.append(&mut full_round.clone());

    actions.push(BLO);
    actions.append(&mut full_round.clone());
    actions.push(BL);

    actions.append(&mut full_round.clone());
    actions.append(&mut full_round.clone());
    actions.append(&mut full_round.clone());

    actions.push(FO);

    let flow = SimpleFlow::new(actions);
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn grzib() -> Plushie {
    use Action::*;

    use crate::acl::actions::Action;

    let mut actions: Vec<Action> = vec![MR(6)];
    actions.append(&mut vec![Inc; 6]);
    let full_round = vec![Sc; 12];
    for _ in 0..3 {
        actions.append(&mut full_round.clone());
    }
    actions.push(Mark("0".into()));
    actions.push(BLO);
    actions.append(&mut vec![Dec; 6]);
    actions.push(FO);

    actions.push(Goto("0".into()));
    actions.push(FLO);
    actions.push(Color((255, 255, 0)));
    actions.append(&mut vec![Inc; 12]);
    actions.push(BL);
    actions.append(&mut vec![Sc; 24]);
    actions.append(&mut vec![Sc; 24]);
    actions.append(&mut vec![Dec; 12]);
    actions.append(&mut vec![Dec; 6]);
    actions.push(FO);

    let flow = SimpleFlow::new(actions);
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn ergogrzib() -> Plushie {
    use Action::*;

    use crate::acl::actions::Action;

    let mut flow = ErgoFlow::new();
    flow += MR(6);
    flow += 6 * Inc;
    flow += 12 * 3 * Sc;
    flow += Mark("0".into()) + BLO;
    flow += 6 * Dec + FO;
    flow += Goto("0".into()) + FLO + Color((255, 255, 0));
    flow += 12 * Inc;
    flow += BL + 24 * 2 * Sc;
    flow += 12 * Dec + 6 * Dec + FO;
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn ergogrzob() -> Plushie {
    use Action::*;

    use crate::acl::actions::Action;

    let mut flow = ErgoFlow::new();
    flow += MR(6);
    flow += 6 * Inc;
    flow += 12 * 3 * Sc;
    flow += Mark("0".into());
    flow += 6 * Dec + FO;
    flow += Goto("0".into()) + Color((255, 255, 0));
    flow += 12 * Inc;
    flow += BL + 24 * 2 * Sc;
    flow += 12 * Dec + 6 * Dec + FO;
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn pillar() -> Plushie {
    use Action::*;

    use crate::acl::actions::Action;

    let mut flow = ErgoFlow::new();
    flow += MR(6);
    flow += Sc * 42;
    flow += FO;
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn disk() -> Plushie {
    use Action::*;

    use crate::acl::actions::Action;

    let mut flow = ErgoFlow::new();
    flow += MR(6);
    flow += Inc * 6; //12
    flow += Inc * 12; //24
    flow += Inc * 24; //48

    flow += Dec * 24;
    flow += Dec * 12;
    flow += Dec * 6;
    flow += FO;
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

macro_rules! generate_get_example {
    ($($name:ident),*) => {
        pub fn get_example(name: &str) -> Option<Plushie> {
            match name {
                $(stringify!($name) => Some($name()),)*
                _ => None,
            }
        }
    };
}
generate_get_example!(pillar, disk);
