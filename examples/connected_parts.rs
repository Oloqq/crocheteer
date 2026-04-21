use bevy::utils::default;
use crocheteer::project::{DisplayMode, Project, SimulationState};

fn main() {
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
            @centroids = 2

            color(255, 255, 0)
            : MR(6)
            # this line is wrong and crashes the application
            : sc, mark(s1), dec, mark(s2), dec, mark(s3), dec, mark(s4), dec, mark(s5), dec, mark(s6)
            #: sc, mark(s1), sc, mark(s2), sc, mark(s3), sc, mark(s4), sc, mark(s5), sc, mark(s6)
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
            ..default()
        },
    };
    crocheteer::app(project).run();
}
