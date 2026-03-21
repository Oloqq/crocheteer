use std::ops::{Add, AddAssign, Mul};

use super::{actions::Action, Flow};

#[derive(Clone)]
pub struct ErgoFlow {
    actions: Vec<Action>,
    i: usize,
}

impl ErgoFlow {
    pub fn new() -> Self {
        Self {
            actions: vec![],
            i: 0,
        }
    }

    pub fn from(actions: Vec<Action>) -> Self {
        Self {
            actions,
            ..Self::new()
        }
    }
}

impl Flow for ErgoFlow {
    fn next(&mut self) -> Option<Action> {
        if self.i < self.actions.len() {
            let got = self.actions[self.i].clone();
            self.i += 1;
            Some(got)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Action> {
        if self.i < self.actions.len() {
            let got = self.actions[self.i].clone();
            Some(got)
        } else {
            None
        }
    }
}

impl Mul<usize> for Action {
    type Output = ErgoFlow;

    fn mul(self, rhs: usize) -> Self::Output {
        ErgoFlow::from(vec![self; rhs])
    }
}

impl Mul<Action> for usize {
    type Output = ErgoFlow;

    fn mul(self, rhs: Action) -> Self::Output {
        ErgoFlow::from(vec![rhs; self])
    }
}

impl Mul<usize> for ErgoFlow {
    type Output = ErgoFlow;

    fn mul(mut self, rhs: usize) -> Self::Output {
        for _ in 2..=rhs {
            self.actions.append(&mut self.actions.clone());
        }
        self
    }
}

impl Add<Action> for Action {
    type Output = ErgoFlow;

    fn add(self, rhs: Action) -> Self::Output {
        ErgoFlow::from(vec![self, rhs])
    }
}

impl Add for ErgoFlow {
    type Output = ErgoFlow;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.actions.append(&mut rhs.actions);
        self
    }
}

impl Add<Action> for ErgoFlow {
    type Output = ErgoFlow;

    fn add(mut self, rhs: Action) -> Self::Output {
        self.actions.push(rhs);
        self
    }
}

impl Add<ErgoFlow> for Action {
    type Output = ErgoFlow;

    fn add(self, mut rhs: ErgoFlow) -> Self::Output {
        rhs.actions.insert(0, self);
        rhs
    }
}

impl AddAssign<ErgoFlow> for ErgoFlow {
    fn add_assign(&mut self, mut rhs: ErgoFlow) {
        self.actions.append(&mut rhs.actions);
    }
}

impl AddAssign<Action> for ErgoFlow {
    fn add_assign(&mut self, rhs: Action) {
        self.actions.push(rhs);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use Action::*;

    use super::*;

    #[test]
    fn test_ergoflow_produces_expected_actions() {
        let mut flow = ErgoFlow::new();
        flow += MR(6);
        flow += 6 * Inc;
        flow += 12 * 3 * Sc;
        flow += Mark("0".into()) + BLO;
        flow += 6 * Dec + FO;
        flow += Goto("0".into()) + FLO + Color((255, 255, 0));
        flow += 12 * Inc;
        flow += BL + (24 * Sc * 2);
        flow += 12 * Dec + 6 * Dec + FO;

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
        actions.push(Color((255, 255, 0)));
        actions.append(&mut vec![Inc; 12]);
        actions.push(BL);
        actions.append(&mut vec![Sc; 24]);
        actions.append(&mut vec![Sc; 24]);
        actions.append(&mut vec![Dec; 12]);
        actions.append(&mut vec![Dec; 6]);
        actions.push(FO);

        assert_eq!(flow.actions, actions);
    }
}
