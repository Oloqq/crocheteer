use super::{
    common::{Output, Program},
    params::{self, Params},
};

#[allow(unused)]
#[derive(Debug)]
pub enum EvalError {
    Finished,
    Syntax(usize, String),
    Semantic(String),
    MaxIteration,
}

pub struct Runtime {}

impl Runtime {
    pub fn new(params: &Params) -> Self {
        Self {}
    }

    pub fn execute(&mut self, program: &Program) -> Output {
        todo!()
    }

    pub fn output(&self) {
        todo!()
    }
}
