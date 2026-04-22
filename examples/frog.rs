use bevy::utils::default;
use crocheteer::project::{DisplayMode, Project, SimulationState};

fn main() {
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
            ..default()
        },
    };
    crocheteer::app(project).run();
}
