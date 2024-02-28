use crate::pattern::Stitch;

use super::common::*;
use super::params::Params;
use rand::prelude::*;

fn grow(length: usize, rand: &mut StdRng) -> Program {
    let mut p = Vec::with_capacity(length);
    for _ in 0..length {
        let s: Stitch = rand.gen();
        p.push(s);
    }
    Program { tokens: p }
}

pub fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    let mut program = Program {
        tokens: Vec::with_capacity(50),
    };
    program.tokens.append(&mut params.prefix.clone());
    program.tokens.append(&mut grow(12, rand).tokens);
    program.tokens.append(&mut params.suffix.clone());
    program
}
