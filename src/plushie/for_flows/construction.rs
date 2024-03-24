mod from_flow;
mod graph;
mod hook;

use std::f32::consts::PI;

use crate::common::*;
use crate::flow::Flow;

use super::Plushie;

#[allow(unused)]
use from_flow::from_flow;

impl Plushie {
    #[allow(unused)]
    pub fn from_flow(flow: impl Flow) -> Result<Self, String> {
        from_flow(flow)
    }

    pub fn position_based_on(&mut self, _other: &Self) {
        println!("TODO: Repositioning");
    }
}
