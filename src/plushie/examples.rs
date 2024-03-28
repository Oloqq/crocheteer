#![allow(unused)]

use crate::pattern::stitches::Stitch;
use crate::pattern::Pattern;
use crate::{flow::simple_flow::SimpleFlow, pattern::builder::PatternBuilder};
use Stitch::*;

use super::{LegacyPlushie, Params, Plushie};

pub fn get(name: &str) -> Option<(Pattern, LegacyPlushie)> {
    Some(match name {
        "pillar" => pillar(),
        "bigball" => bigball(),
        "ball" => ball(),
        "bigpillar" => bigpillar(),
        "vase" => vase(),
        "bowl" => bowl(),
        _ => return None,
    })
}

pub fn pillar() -> (Pattern, LegacyPlushie) {
    let pattern = PatternBuilder::new(6).full_rounds(4).fasten_off().unwrap();
    let plushie = LegacyPlushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn bigball() -> (Pattern, LegacyPlushie) {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .round_like(&vec![Sc, Inc])
        .full_rounds(1)
        .round_like(&vec![Sc, Dec])
        .round_like(&vec![Dec])
        .fasten_off()
        .unwrap();
    let plushie = LegacyPlushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn ball() -> (Pattern, LegacyPlushie) {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .full_rounds(1)
        .round_like(&vec![Dec])
        .fasten_off()
        .unwrap();
    let plushie = LegacyPlushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn bigpillar() -> (Pattern, LegacyPlushie) {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .full_rounds(6)
        .round_like(&vec![Dec])
        .fasten_off()
        .unwrap();
    let plushie = LegacyPlushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn vase() -> (Pattern, LegacyPlushie) {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .full_rounds(6)
        .loose_end()
        .unwrap();
    let plushie = LegacyPlushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn vase_simple_flow() -> Plushie {
    use crate::flow::actions::Action;
    use Action::*;
    let mut actions: Vec<Action> = vec![MR(6)];
    actions.append(&mut vec![Inc; 6]);
    let full_round = vec![Sc; 12];
    for _ in 0..6 {
        actions.append(&mut full_round.clone());
    }

    let flow = SimpleFlow::new(actions);
    let plushie = Plushie::from_flow(flow).unwrap();
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
    let plushie = Plushie::from_flow(flow).unwrap();
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
    let plushie = Plushie::from_flow(flow).unwrap();
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
    let plushie = Plushie::from_flow(flow).unwrap();
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
    let plushie = Plushie::from_flow(flow).unwrap();
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
    let plushie = Plushie::from_flow(flow).unwrap();
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
    let plushie = Plushie::from_flow(flow).unwrap();
    plushie
}

pub fn bowl() -> (Pattern, LegacyPlushie) {
    let pattern = Pattern::from_human_readable(
        "@centroids = 6
    @floor = true
    : MR 6 (6)
    : 6 inc (12)
    : [inc, sc] x 6 (18)
    : [inc, 2 sc] x 6 (24)
    : [inc, 3 sc] x 6 (30)
    : [inc, 4 sc] x 6 (36)
    : 36 sc (36) # BLO
    : 36 sc (36)
    : [inc, 5 sc] x 6 (42)
    2: 42 sc (42)
    : [inc, 6 sc] x 6 (48)",
    )
    .unwrap();
    let plushie = LegacyPlushie::from_pattern(&pattern);
    (pattern, plushie)
}
