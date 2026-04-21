use indoc::indoc;

use crate::force_graph::simulated_plushie::step::SimulationParams;

const HOOK_SIZE: f32 = 5e-4;
const PARAMS: SimulationParams = SimulationParams {
    force_multiplier: 1.0,
    single_loop_force: 0.02,
};

mod regular_cylinder {
    use crate::acl::Action;

    use super::*;

    #[test]
    fn test_reflecting_node_stays_in_place() {
        let pat = indoc! {"
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
    "};
        let (_, mut plushie) = crate::parse(
            pat,
            HOOK_SIZE,
            &crate::force_graph::Initializer::RegularCylinder(12),
        )
        .unwrap();
        let initial_pos = plushie.nodes[0].position;
        plushie.step(&PARAMS);
        assert_eq!(plushie.nodes[0].position, initial_pos);
        plushie.step(&PARAMS);
        assert_eq!(plushie.nodes[0].position, initial_pos);
    }

    #[test]
    fn test_only_one_reflecting_node_on_connected_parts() {
        let pat = indoc::indoc! {"
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
        : sc, mark(s1), sc, mark(s2), sc, mark(s3), sc, mark(s4), sc, mark(s5), sc, mark(s6)
        : 6 inc (12)
        : [sc, inc] x 6 (18)
        : [2 sc, inc] x 6 (24)
        : 12 dec (12)
        : 6 dec (6)
        FO

        sew(d1, s1)
    "};
        let (_, mut plushie) = crate::parse(
            pat,
            HOOK_SIZE,
            &crate::force_graph::Initializer::RegularCylinder(12),
        )
        .unwrap();
        assert_eq!(
            plushie.parts[0].reflecting_node,
            plushie.parts[1].reflecting_node
        );
        let initial_pos_0 = plushie.nodes[0].position;
        let next_mr = plushie
            .nodes
            .iter()
            .enumerate()
            .skip(7)
            .find(|(_, n)| matches!(n.definition.origin.action, Action::MR(6)))
            .unwrap()
            .0;
        let initial_pos_next = plushie.nodes[next_mr].position;
        plushie.step(&PARAMS);
        assert_eq!(plushie.nodes[0].position, initial_pos_0);
        assert_ne!(plushie.nodes[next_mr].position, initial_pos_next);
        plushie.step(&PARAMS);
        assert_eq!(plushie.nodes[0].position, initial_pos_0);
        assert_ne!(plushie.nodes[next_mr].position, initial_pos_next);
    }
}

mod one_by_one {
    use super::*;

    #[test]
    fn test_reflecting_node_stays_in_place() {
        let pat = indoc! {"
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
    "};
        let (_, mut plushie) =
            crate::parse(pat, HOOK_SIZE, &crate::force_graph::Initializer::OneByOne).unwrap();
        plushie.advance_one_by_one();
        let initial_pos = plushie.nodes[0].position;
        plushie.step(&PARAMS);
        assert_eq!(plushie.nodes[0].position, initial_pos);
        plushie.step(&PARAMS);
        assert_eq!(plushie.nodes[0].position, initial_pos);
    }

    #[test]
    #[ignore = "developing, plushie in simulation needs to know that some edges are added later, another vec produced in hook?"]
    fn test_only_one_reflecting_node_on_connected_parts_with_sew() {
        let pat = indoc::indoc! {"
        == Stem ==
        @centroids = 1

        : MR(6)
        : sc, mark(d1), sc, mark(d2), sc, mark(d3), sc, mark(d4), sc, mark(d5), sc, mark(d6)  (6)
        FO

        == Cap ==
        @centroids = 1

        color(255, 255, 0)
        : MR(6)
        : sc, mark(s1), sc, mark(s2), sc, mark(s3), sc, mark(s4), sc, mark(s5), sc, mark(s6)
        FO

        sew(d1, s1)
    "};
        let (_, mut _plushie) = crate::parse(
            pat,
            HOOK_SIZE,
            &crate::force_graph::Initializer::RegularCylinder(12),
        )
        .unwrap();
    }
}
