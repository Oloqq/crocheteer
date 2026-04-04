use crate::acl::actions::Action;
pub mod ergoflow;
#[cfg(test)]
pub mod simple_flow;

/// Iterator over ACL actions
pub trait Flow {
    fn next(&mut self) -> Option<Action>;
    fn peek(&self) -> Option<Action>;
}

#[cfg(test)]
mod tests {
    use Action::*;
    use pretty_assertions::assert_eq;

    use crate::acl::pest_parser::Pattern;

    use super::{ergoflow::ErgoFlow, simple_flow::SimpleFlow, *};

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
    }

    #[test]
    fn test_assertion_helper() {
        let ergo = {
            let mut flow = ErgoFlow::new();
            flow += MR(6);
            flow += 6 * Inc;
            flow += 12 * 3 * Sc;
            flow += Mark("0".into()) + BLO;
            flow += 6 * Dec + FO;
            flow += Goto("0".into()) + FLO + Color((255, 255, 0).into());
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
            actions.push(Mark("0".into()));
            actions.push(BLO);
            actions.append(&mut vec![Dec; 6]);
            actions.push(FO);

            actions.push(Goto("0".into()));
            actions.push(FLO);
            actions.push(Color((255, 255, 0).into()));
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
            flow += Mark("0".into()) + BLO;
            flow += 6 * Dec + FO;
            flow += Goto("0".into()) + FLO + Color((255, 255, 0).into());
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
            flow += Mark("cap_start".into()) + BLO;
            flow += 6 * Dec + FO;
            flow += Goto("cap_start".into()) + Color((255, 255, 0).into());
            flow += BL; // this is to account for automatic returns to BothLoop at the start of a round
            flow += FLO;
            flow += 12 * Inc;
            flow += BL + 24 * 2 * Sc;
            flow += 12 * Dec + 6 * Dec + FO;
            flow
        };

        let pattern = {
            let src = "MR(6)
            : 6 inc
            3: 12 sc
            mark(cap_start)
            : BLO, 6 dec
            FO

            goto(cap_start), color(255, 255, 0)
            : FLO, 12 inc
            2: 24 sc
            : 12 dec
            : 6 dec
            FO";
            Pattern::parse(src).unwrap()
        };

        assert_equal_flows(ergo, pattern);
    }
}
