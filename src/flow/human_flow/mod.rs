mod pattern;

// use super::{actions::Action, Flow};

// pub struct HumanFlow {
//     actions: Vec<Action>,
//     i: usize,
// }

// impl HumanFlow {
//     pub fn new(actions: Vec<Action>) -> Self {
//         Self { actions, i: 0 }
//     }
// }

// impl Flow for HumanFlow {
//     fn next(&mut self) -> Option<Action> {
//         if self.i < self.actions.len() {
//             let got = self.actions[self.i];
//             self.i += 1;
//             Some(got)
//         } else {
//             None
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::flow::actions::Action::*;

//     #[test]
//     fn test_next() {
//         let mut f = HumanFlow::new(vec![MR(4), Sc, Sc, Sc, Sc]);
//         assert_eq!(f.next().unwrap(), MR(4));
//         assert_eq!(f.next().unwrap(), Sc);
//         assert_eq!(f.next().unwrap(), Sc);
//         assert_eq!(f.next().unwrap(), Sc);
//         assert_eq!(f.next().unwrap(), Sc);
//         assert!(f.next().is_none());
//         assert!(f.next().is_none());
//     }
// }
