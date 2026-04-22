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
            ..default()
        },
    };
    crocheteer::app(project).run();
}
