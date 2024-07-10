#![allow(unused)]

use super::{Params, Plushie};
use crate::common::*;
use crate::flow::actions::Action;
use crate::flow::ergoflow::ErgoFlow;
use crate::flow::simple_flow::SimpleFlow;
use Action::*;

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
    use crate::flow::actions::Action;
    use Action::*;
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
    use crate::flow::actions::Action;
    use Action::*;
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

pub fn hat() -> Plushie {
    use crate::flow::actions::Action;
    use Action::*;

    let mut actions: Vec<Action> = vec![Ch(12)];
    let full_round = vec![Sc; 12];
    for _ in 0..6 {
        actions.append(&mut full_round.clone());
    }
    actions.push(FO);

    let flow = SimpleFlow::new(actions);
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn flailer() -> Plushie {
    use crate::flow::actions::Action;
    use Action::*;

    let mut actions: Vec<Action> = vec![MR(6)];
    actions.append(&mut vec![Inc; 6]);
    for _ in 0..3 {
        actions.append(&mut vec![Sc; 12]);
    }
    actions.push(Ch(6));

    let flow = SimpleFlow::new(actions);
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn fatflailer() -> Plushie {
    use crate::flow::actions::Action;
    use Action::*;

    let mut flow = ErgoFlow::new();
    flow += MR(6);
    flow += 6 * Inc;
    flow += 12 * 3 * Sc;
    flow += Color(RED) + Ch(6) + Color(GREEN) + Sc * 6;
    flow += Sc * 8;
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn grzib() -> Plushie {
    use crate::flow::actions::Action;
    use Action::*;

    let mut actions: Vec<Action> = vec![MR(6)];
    actions.append(&mut vec![Inc; 6]);
    let full_round = vec![Sc; 12];
    for _ in 0..3 {
        actions.append(&mut full_round.clone());
    }
    actions.push(Mark(0));
    actions.push(BLO);
    actions.append(&mut vec![Dec; 6]);
    actions.push(FO);

    actions.push(Goto(0));
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
    use crate::flow::actions::Action;
    use Action::*;

    let mut flow = ErgoFlow::new();
    flow += MR(6);
    flow += 6 * Inc;
    flow += 12 * 3 * Sc;
    flow += Mark(0) + BLO;
    flow += 6 * Dec + FO;
    flow += Goto(0) + FLO + Color((255, 255, 0));
    flow += 12 * Inc;
    flow += BL + 24 * 2 * Sc;
    flow += 12 * Dec + 6 * Dec + FO;
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}

pub fn lollipop() -> Plushie {
    use crate::flow::actions::Action;
    use Action::*;

    let mut actions: Vec<Action> = vec![MR(6)];
    actions.append(&mut vec![Inc; 6]);
    let full_round = vec![Sc; 12];
    for _ in 0..6 {
        actions.append(&mut full_round.clone());
    }
    actions.push(Mark(0));
    actions.push(BLO);
    actions.append(&mut vec![Dec; 6]);
    actions.push(Mark(1));
    actions.push(FO);

    actions.push(Goto(0));
    actions.push(FLO);
    actions.append(&mut vec![Inc; 12]);
    actions.push(BL);
    actions.append(&mut vec![Sc; 24]);
    actions.append(&mut vec![Sc; 24]);
    actions.append(&mut vec![Dec; 12]);

    actions.push(Ch(2));
    actions.push(Attach(1));
    actions.append(&mut vec![Sc; 2]);

    actions.append(&mut vec![Dec; 6]);
    actions.push(FO);

    let flow = SimpleFlow::new(actions);
    let plushie = Plushie::from_flow(flow, Params::default()).unwrap();
    plushie
}
