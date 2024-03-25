mod hook;
mod hook_result;

use self::hook::Hook;
pub use self::hook_result::Peculiarity;
use super::animation::centroid::Centroids;
use super::nodes::Nodes;
use super::Plushie;
use crate::flow::Flow;

impl Plushie {
    pub fn from_flow(flow: impl Flow) -> Result<Self, String> {
        let hook_result = Hook::parse(flow)?;
        assert!(hook_result
            .peculiarities
            .get(&0)
            .is_some_and(|x| *x == Peculiarity::Root));

        Ok(Plushie {
            nodes: Nodes::new(hook_result.nodes, hook_result.peculiarities),
            edges: hook_result.edges,
            params: Default::default(),
            centroids: Centroids::new(2, hook_result.approximate_height),
        })
    }

    pub fn parse(_pattern: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn _position_based_on(&mut self, _other: &Self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::flow::simple_flow::SimpleFlow;

    use super::*;

    #[test]
    fn test_closed_shape() {
        use crate::flow::actions::Action;
        use Action::*;
        let mut actions: Vec<Action> = vec![MR(6)];
        actions.append(&mut vec![Sc; 6]);

        let flow = SimpleFlow::new(actions);
        let plushie = Plushie::from_flow(flow).unwrap();

        assert_eq!(plushie.nodes.len(), 13)
    }

    #[test]
    fn test_open_shape() {
        use crate::flow::actions::Action;
        use Action::*;
        let mut actions: Vec<Action> = vec![MR(6)];
        actions.append(&mut vec![Sc; 6]);
        actions.append(&mut vec![FO]);

        let flow = SimpleFlow::new(actions);
        let plushie = Plushie::from_flow(flow).unwrap();

        assert_eq!(plushie.nodes.len(), 14)
    }

    mod for_refactor {
        use super::*;
        use crate::flow::actions::Action::*;
        use crate::flow::simple_flow::SimpleFlow;

        use pretty_assertions::assert_eq;
        #[test]
        fn test_from_pattern_1() {
            let f = SimpleFlow::new(vec![MR(4), Sc, Sc, Sc, Sc, FO]);
            let plushie_flow = Plushie::from_flow(f).unwrap();
            assert_eq!(
                plushie_flow.edges,
                vec![
                    // 0 ->
                    vec![1, 2, 3, 4],
                    // 1 ->
                    vec![2, 5],
                    // 2 ->
                    vec![3, 6],
                    // 3 ->
                    vec![4, 7],
                    // 4 ->
                    vec![5, 8],
                    // new round 5 ->
                    vec![6, 9],
                    // 6 ->
                    vec![7, 9],
                    // 7 ->
                    vec![8, 9],
                    // 8 ->
                    vec![9],
                    // tip 9 ->
                    vec![],
                ]
            );
            assert_eq!(plushie_flow.nodes.len(), 10);
        }

        // #[test]
        // #[ignore = "need to fill"]
        // fn test_from_pattern_no_fasten_off() {
        //     let p = {
        //         use Stitch::Sc;
        //         Pattern {
        //             starting_circle: 4,
        //             fasten_off: false,
        //             rounds: vec![vec![Sc, Sc, Sc, Sc]],
        //             simulation_config: Params::default(),
        //         }
        //     };
        //     let f = SimpleFlow::new(vec![]);

        //     let plushie_pattern = Plushie::from_pattern(&p);
        //     assert_eq!(plushie_pattern.nodes.len(), 9);
        //     assert_eq!(
        //         plushie_pattern.edges,
        //         vec![
        //             // 0 ->
        //             vec![1, 2, 3, 4],
        //             // 1 ->
        //             vec![2, 5],
        //             // 2 ->
        //             vec![3, 6],
        //             // 3 ->
        //             vec![4, 7],
        //             // 4 ->
        //             vec![5, 8],
        //             // 5 ->
        //             vec![6],
        //             // 6 ->
        //             vec![7],
        //             // 7 ->
        //             vec![8],
        //             // 8 ->
        //             vec![],
        //         ]
        //     );
        //     let plushie_flow = Plushie::from_flow(f).unwrap();
        //     assert_eq!(plushie_flow.nodes.len(), plushie_pattern.nodes.len());
        //     assert_eq!(plushie_flow.edges, plushie_pattern.edges);
        // }

        // #[test]
        // #[ignore = "need to fill"]
        // fn test_from_pattern_increase_decrese() {
        //     let p = {
        //         use Stitch::*;
        //         Pattern {
        //             starting_circle: 4,
        //             fasten_off: true,
        //             rounds: vec![vec![Sc, Inc, Sc, Sc], vec![Sc, Dec, Sc, Sc]],
        //             simulation_config: Params::default(),
        //         }
        //     };
        //     let f = SimpleFlow::new(vec![]);

        //     let plushie_pattern = Plushie::from_pattern(&p);
        //     assert_eq!(plushie_pattern.nodes.len(), 15);
        //     assert_eq!(
        //         plushie_pattern.edges,
        //         vec![
        //             /* 0 -> */ vec![2, 3, 4, 5],
        //             /* 1 -> */ vec![11, 12, 13, 14],
        //             /* 2 -> */ vec![3, 6],
        //             /* 3 -> */ vec![4, 7, 8],
        //             /* 4 -> */ vec![5, 9],
        //             /* 5 -> */ vec![6, 10],
        //             /* 6 -> */ vec![7, 11],
        //             /* 7 -> */ vec![8, 12],
        //             /* 8 -> */ vec![9, 12],
        //             /* 9 -> */ vec![10, 13],
        //             /* 10 -> */ vec![11, 14],
        //             /* 11 -> */ vec![12],
        //             /* 12 -> */ vec![13],
        //             /* 13 -> */ vec![14],
        //             /* 14 -> */ vec![],
        //         ]
        //     );
        //     let plushie_flow = Plushie::from_flow(f).unwrap();
        //     assert_eq!(plushie_flow.nodes.len(), plushie_pattern.nodes.len());
        //     assert_eq!(plushie_flow.edges, plushie_pattern.edges);
        // }

        // #[test]
        // #[ignore = "need to fill"]
        // fn from_genetic_mutant_1() {
        //     let p = {
        //         use Stitch::*;
        //         Pattern {
        //             starting_circle: 6,
        //             fasten_off: true,
        //             rounds: vec![
        //                 vec![Dec, Dec, Dec],
        //                 vec![Sc, Dec],
        //                 vec![Dec],
        //                 vec![Sc],
        //                 vec![Sc],
        //                 vec![Sc],
        //                 vec![Inc],
        //                 vec![Sc, Inc],
        //             ],
        //             simulation_config: Params::default(),
        //         }
        //     };
        //     let f = SimpleFlow::new(vec![]);

        //     let pl = Plushie::from_pattern(&p);
        //     assert_eq!(pl.nodes.len(), 22);
        //     // pl.animate();
        //     let plushie_flow = Plushie::from_flow(f).unwrap();
        //     assert_eq!(plushie_flow.nodes.len(), pl.nodes.len());
        //     assert_eq!(plushie_flow.edges, pl.edges);
        // }

        // #[test]
        // #[ignore = "need to fill"]
        // fn from_genetic_mutant_2() {
        //     let p = {
        //         use Stitch::*;
        //         Pattern {
        //             starting_circle: 6,
        //             fasten_off: true,
        //             rounds: vec![vec![Dec, Dec, Dec]],
        //             simulation_config: Params::default(),
        //         }
        //     };
        //     let f = SimpleFlow::new(vec![]);

        //     let pl = Plushie::from_pattern(&p);
        //     assert_eq!(pl.nodes.len(), 11);
        //     // pl.animate();
        //     let plushie_flow = Plushie::from_flow(f).unwrap();
        //     assert_eq!(plushie_flow.nodes.len(), pl.nodes.len());
        //     assert_eq!(plushie_flow.edges, pl.edges);
        // }

        // #[test]
        // #[ignore = "need to fill"]
        // fn from_genetic_mutant_3() {
        //     let p = {
        //         use Stitch::*;
        //         Pattern {
        //             starting_circle: 6,
        //             fasten_off: true,
        //             rounds: vec![vec![Dec, Dec, Dec], vec![Sc, Sc, Inc]],
        //             simulation_config: Params::default(),
        //         }
        //     };
        //     let f = SimpleFlow::new(vec![]);

        //     let pl = Plushie::from_pattern(&p);
        //     assert_eq!(pl.nodes.len(), 15);
        //     // pl.animate();
        //     let plushie_flow = Plushie::from_flow(f).unwrap();
        //     assert_eq!(plushie_flow.nodes.len(), pl.nodes.len());
        //     assert_eq!(plushie_flow.edges, pl.edges);
        // }
    }
}
