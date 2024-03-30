pub mod actions;
pub mod ergoflow;
pub mod human_flow;
pub mod simple_flow;

use self::actions::Action;

pub trait Flow {
    fn next(&mut self) -> Option<Action>;
    fn peek(&self) -> Option<Action>;
}
