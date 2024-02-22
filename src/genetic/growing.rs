use crate::pattern::Stitch;

use super::common::*;
use super::params::Params;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

fn grow(length: usize, rand: &mut StdRng) -> Program {
    let mut p = Vec::with_capacity(length);
    for _ in 0..length {
        let s: Stitch = rand.gen();
        p.push(s);
    }
    p
}

pub fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    let mut program: Program = Vec::with_capacity(50);
    program.append(&mut params.prefix.clone());
    program.append(&mut grow(12, rand));
    program.append(&mut params.suffix.clone());
    program
}
