use crate::{
    project::{DisplayMode, Project},
    ui::SimulationState,
};

pub fn shroom_no_slf() -> Project {
    let project = Project {
        pattern: indoc::indoc! {"
            @centroids = 3
            : MR(6)
            : 6 inc (12)
            3: 12 sc (12)
            mark(cap_start)
            : BLO, 6 dec (6)
            FO

            goto(cap_start), color(255, 255, 0)
            : FLO, 12 inc (24)
            2: 24 sc (24)
            : 12 dec (12)
            : 6 dec (6)
            FO
        "}
        .into(),
        simulation_config: SimulationState {
            sim_speed: 1.0,
            display_mode: DisplayMode::Forces,
            single_loop_force: 0.0,
            ..Default::default()
        },
    };
    project
}

pub fn frog() -> Project {
    let project = Project {
        // https://toristorycreations.com/froggy-friend-pattern/
        // TODO color before an actual part still produces an anonymous part, it shouldn't (make an exception, where if color is alone in anonymous part, when named parts starts, color is merged into the actual part)
        // TODO configurable build orientation and position
        // frog body need to be built down, not up
        // eyes need to be built at an angle, and with some more sensible initial position
        // same applies to limbs
        // TODO stuffing between parts
        // eyes phase into the body as the stuffing does not work between parts
        // make repulsion act between parts (only when they are joined), keep centroid recalculation part-local
        // TODO ergonomic sews
        // place one mark (call it center) use one sew-like action, and let simulation decide which specific nodes to link
        // plushie needs to be relaxed for this to make sense,
        // UI should support a queue of joins
        // so user can wait for plushie to relax,
        // then drag the parts to appropriate positions, (TODO support part rotation in addition to translation)
        // then execute the joins from a queue (one click = one part join)
        // UI should display the calculated part position in a way the user can copy and paste it into the pattern
        // so on next simulation the part is already where it is needed without user input
        // making a join wait for the queue input must be configurable then
        // BUT, what should persist in pattern? the center of attachment, or each specific link?
        // center => joining is undeterministic (the user squishes plushie before join triggers, can't work at all with cylinder initializer)
        // per-link => how is it possible to be saved? human redable patterns do not specify precise stitches where things are sewn
        // doing marks like in the prototype below is unreadable and useless anyway
        // this should be saved as node indexes relative to part starts, doesn't have to be human-readable
        // the join would become invalid when the part is changed, this should be recognizable (e.g. by including a pattern hash in the generated data)
        // I don't like human-unreadable data in pattern.
        // this is a good pretext to create actual project files, file save, file load etc.
        pattern: indoc::indoc! {"
            color(0, 255, 0)

            == Body ==
            @centroids = 1
            R1: MR(6)
            R2: 6 inc (12)
            R3: sc, inc, sc, inc, sc, inc, mark(eye1_1b), sc, inc, sc, inc, sc, inc, mark(eye2_1b) (12)
            R4: 2 sc, color(0, 0, 0), inc, color(0, 255, 0), 2 sc, inc, 2 sc, inc, 2 sc, inc, 2 sc, inc, 2 sc, inc (18)
            R5: [3 sc, inc] x 6 (24)
            R6: 15 sc, mark(eye1_2b), 15 sc, mark(eye2_2b)
            R7-R8: 30 sc
            R9: 12 sc, mark(eye1_3b), 3 sc, mark(eye1_4b), 15 sc
            R10-R11: 30 sc
            # now create the mouth with some black thread between rows 4 and 5
            R12: [3 sc, dec] x 6 (24)
            R13: [2 sc, dec] x 6 (18)
            R14: [sc, dec] x 6 (12)
            # stuff firmly
            R15: 6 dec (6)
            FO

            == Eye1 ==
            @centroids = 1
            color(0, 255, 0)
            R1: MR(6)
            R2: 6 inc (12)
            R3: 3 sc, mark(eye1_1e), 3 sc, mark(eye1_2e), 3 sc, mark(eye1_3e), 3 sc, mark(eye1_4e) (12)
            # fasten off with a slip stitch, leave some yarn for sewing
            # attach safety eye between rows 2 and 3

            == Eye2 ==
            @centroids = 1
            color(0, 255, 0)
            R1: MR(6)
            R2: 6 inc (12)
            R3: 6 sc, mark(eye2_1e), 6 sc, mark(eye2_2e) (12)
            # fasten off with a slip stitch, leave some yarn for sewing
            # attach safety eye between rows 2 and 3

            == Arm1 ==
            @centroids = 1
            R1: MR(5)
            R2: 5 sc
            R3: 5 sc
            # fasten off with a slip stitch, leave some yarn for sewing

            == Arm2 ==
            @centroids = 1
            R1: MR(5)
            R2: 5 sc
            R3: 5 sc
            # fasten off with a slip stitch, leave some yarn for sewing

            == Leg1 ==
            @centroids = 1
            R1: MR(5)
            R2: 5 sc
            R3: 5 sc
            # fasten off with a slip stitch, leave some yarn for sewing

            == Leg2 ==
            @centroids = 1
            R1: MR(5)
            R2: 5 sc
            R3: 5 sc
            # fasten off with a slip stitch, leave some yarn for sewing

            sew(eye1_1b, eye1_1e)
            sew(eye1_2b, eye1_2e)
            sew(eye1_3b, eye1_3e)
            sew(eye1_4b, eye1_4e)
            sew(eye2_1b, eye2_1e)
            sew(eye2_2b, eye2_2e)
        "}
        .into(),
        simulation_config: SimulationState {
            sim_speed: 1.0,
            single_loop_force: 0.0,
            display_mode: DisplayMode::Pattern,
            initializer: crochet::force_graph::Initializer::OneByOne,
            ..Default::default()
        },
    };
    project
}

pub fn code_highlights() -> Project {
    let project = Project {
        pattern: indoc::indoc! {"
            @centroids = 3,
            @param = yes # comment

            : MR(6) # trailing comment, no round
            : 6 inc (12) # trailing after anchor count
            # expect an error with cap_start in a repetition
            : 12 sc # trailing without anchor count
            # 2: 12 sc, mark(🐸ąęó編みぐるみ) (12) # unicode unfortunately does not work in labels, but is fine in comments
            2: 12 sc, mark(cap_start) (12)
            : BLO, 6 dec (6)
            FO

            goto(cap_start), color(255, 255, 0)
            : FLO, 12 inc (24)
            2: 24 sc (24)
            : 12 dec (12)
            : dec, dec, dec, dec, dec, dec (6)
            FO
        "}
        .into(),

        // TODO initialize in a way that some nodes are already selected
        simulation_config: SimulationState {
            sim_speed: 1.0,
            display_mode: DisplayMode::Forces,
            ..Default::default()
        },
    };
    project
}

pub fn one_by_one() -> Project {
    let project = Project {
        pattern: indoc::indoc! {"
            @centroids = 3
            : MR(6)
            : 6 inc (12)
            3: 12 sc (12)
            mark(cap_start)
            : BLO, 6 dec (6)
            FO

            goto(cap_start), color(255, 255, 0)
            : FLO, 12 inc (24)
            2: 24 sc (24)
            : 12 dec (12)
            : 6 dec (6)
            FO
        "}
        .into(),
        simulation_config: SimulationState {
            sim_speed: 1.0,
            single_loop_force: 0.0,
            display_mode: DisplayMode::Forces,
            initializer: crochet::force_graph::Initializer::OneByOne,
            ..Default::default()
        },
    };
    project
}

pub fn sewn_parts_one_by_one() -> Project {
    let project = Project {
        pattern: indoc::indoc! {"
            == Stem ==
            @centroids = 1

            : MR(6)
            : 6 inc (12)
            2: 12 sc (12)
            : BLO, dec, mark(d1), dec, mark(d2), dec, mark(d3), dec, mark(d4), dec, mark(d5), dec, mark(d6)  (6)
            FO

            == Cap ==
            @centroids = 1

            color(255, 255, 0)
            : MR(6)
            : sc, mark(s1), sc, mark(s2), sc, mark(s3), sc, mark(s4), sc, mark(s5), sc, mark(s6)
            : 6 inc (12)
            : [sc, inc] x 6 (18)
            : [2 sc, inc] x 6 (24)
            : 12 dec (12)
            : dec, mark(g1), dec, mark(g2), dec, mark(g3), dec, mark(g4), dec, mark(g5), dec, mark(g6)  (6)
            FO

            sew(d1, s1)
            sew(d2, s2)
            sew(d3, s3)
            sew(d4, s4)
            sew(d5, s5)
            sew(d6, s6)

            == Parasite ==
            @centroids = 2

            color(255, 0, 255)
            : MR(6)
            4: 6 sc (6)
            : sc, mark(h1), sc, mark(h2), sc, mark(h3), sc, mark(h4), sc, mark(h5), sc, mark(h6)
            FO

            sew(h1, g1)
            sew(h2, g2)
            sew(h3, g3)
            sew(h4, g4)
            sew(h5, g5)
            sew(h6, g6)
        "}
        .into(),
        simulation_config: SimulationState {
            sim_speed: 1.0,
            single_loop_force: 0.0,
            display_mode: DisplayMode::Forces,
            initializer: crochet::force_graph::Initializer::OneByOne,
            ..Default::default()
        },
    };
    project
}

pub fn sewn_parts() -> Project {
    let project = Project {
        // TODO acl: place these 6 marks with single action e.g mark_next_n(6, d)
        // TODO acl: marks can't be placed on ring of MR right now, add e.g. mark_previous_n(6, m)
        pattern: indoc::indoc! {"
            == Stem ==
            @centroids = 1

            : MR(6)
            : 6 inc (12)
            2: 12 sc (12)
            : BLO, dec, mark(d1), dec, mark(d2), dec, mark(d3), dec, mark(d4), dec, mark(d5), dec, mark(d6)  (6)
            FO

            == Cap ==
            @centroids = 1

            color(255, 255, 0)
            : MR(6)
            : sc, mark(s1), sc, mark(s2), sc, mark(s3), sc, mark(s4), sc, mark(s5), sc, mark(s6)
            : 6 inc (12)
            : [sc, inc] x 6 (18)
            : [2 sc, inc] x 6 (24)
            : 12 dec (12)
            : dec, mark(g1), dec, mark(g2), dec, mark(g3), dec, mark(g4), dec, mark(g5), dec, mark(g6)  (6)
            FO

            sew(d1, s1)
            sew(d2, s2)
            sew(d3, s3)
            sew(d4, s4)
            sew(d5, s5)
            sew(d6, s6)

            == Parasite ==
            @centroids = 2

            color(255, 0, 255)
            : MR(6)
            4: 6 sc (6)
            : sc, mark(h1), sc, mark(h2), sc, mark(h3), sc, mark(h4), sc, mark(h5), sc, mark(h6)
            FO

            sew(h1, g1)
            sew(h2, g2)
            sew(h3, g3)
            sew(h4, g4)
            sew(h5, g5)
            sew(h6, g6)
        "}
        .into(),
        simulation_config: SimulationState {
            sim_speed: 1.0,
            single_loop_force: 0.0,
            display_mode: DisplayMode::Forces,
            ..Default::default()
        },
    };
    project
}

pub fn unconnected_parts() -> Project {
    let project = Project {
        pattern: indoc::indoc! {"
            == Stem ==
            @centroids = 1

            : MR(6)
            : 6 inc (12)
            2: 12 sc (12)
            : BLO, 6 dec (6)
            FO

            == Cap ==
            @centroids = 2

            color(255, 255, 0)
            : MR(6)
            : 6 inc (12)
            : [sc, inc] x 6 (18)
            : [2 sc, inc] x 6 (24)
            : 12 dec (12)
            : 6 dec (6)
            FO
        "}
        .into(),
        simulation_config: SimulationState {
            sim_speed: 1.0,
            single_loop_force: 0.0,
            display_mode: DisplayMode::Forces,
            ..Default::default()
        },
    };
    project
}
