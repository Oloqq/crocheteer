/// Flow is an iterator over stitches.
/// Specific implemetations are designed to handle various pattern formats,
/// or creating patterns in Rust code directly
pub mod actions;
pub mod ergoflow;
pub mod genetic;
pub mod pest_parser;
pub mod simple_flow;

use self::actions::Action;

pub trait Flow {
    fn next(&mut self) -> Option<Action>;
    fn peek(&self) -> Option<Action>;
}

#[cfg(test)]
mod tests {
    use super::ergoflow::ErgoFlow;
    use super::pest_parser::{ErrorCode, Pattern};
    use super::simple_flow::SimpleFlow;
    use super::*;
    use pretty_assertions::assert_eq;
    use Action::*;

    fn assert_equal_flows(mut left: impl Flow, mut right: impl Flow) {
        let left_actions = {
            let mut res = vec![];
            while let Some(x) = left.next() {
                res.push(x);
            }
            res
        };
        let right_actions = {
            let mut res = vec![];
            while let Some(x) = right.next() {
                res.push(x);
            }
            res
        };
        assert_eq!(left_actions, right_actions);
        // let mut i = 0;
        // let mut left_action = left.next();
        // let mut right_action = right.next();
        // while left_action.is_some() {
        //     assert!(right_action.is_some(), "[{i}] Righthand flow is shorter.");
        //     assert!(
        //         left_action == right_action,
        //         "[{i}] Action mismatch: {left_action:?} vs {right_action:?}"
        //     );
        //     left_action = left.next();
        //     right_action = right.next();
        //     i += 1;
        // }
        // assert!(right_action.is_none(), "[{i}] Lefthand flow is shorter.");
    }

    #[test]
    fn test_assertion() {
        let ergo = {
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
            flow
        };
        assert_equal_flows(ergo.clone(), ergo);
    }

    #[test]
    fn test_simple_vs_ergo() {
        let simple = {
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
            SimpleFlow::new(actions)
        };

        let ergo = {
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
            flow
        };

        assert_equal_flows(simple, ergo);
    }

    #[test]
    fn test_ergo_vs_pest_pattern() {
        let ergo = {
            let mut flow = ErgoFlow::new();
            flow += MR(6);
            flow += 6 * Inc;
            flow += 12 * 3 * Sc;
            flow += Mark(0) + BLO;
            flow += 6 * Dec + FO;
            flow += Goto(0) + Color((255, 255, 0));
            flow += BL; // this is to account for automatic returns to BothLoop at the start of a round
            flow += FLO;
            flow += 12 * Inc;
            flow += BL + 24 * 2 * Sc;
            flow += 12 * Dec + 6 * Dec + FO;
            flow
        };

        let pattern = {
            let src = "MR(6)
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
            FO";
            match Pattern::parse(src) {
                Ok(x) => x,
                Err(e) => match e.code {
                    ErrorCode::Lexer(lexerr) => panic!("{lexerr}"),
                    _ => panic!("{e}"),
                },
            }
        };

        assert_equal_flows(ergo, pattern);
    }
}
