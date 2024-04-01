pub mod actions;
pub mod ergoflow;
pub mod pest_parser;
pub mod simple_flow;

use self::actions::Action;

pub trait Flow {
    fn next(&mut self) -> Option<Action>;
    fn peek(&self) -> Option<Action>;
}
