use crate::common::*;
use crate::comparison::Comparator;
use crate::flow::actions::Action;
use crate::flow::genetic;
use crate::flow::simple_flow::SimpleFlow;
use crate::plushie::params::{self, Leniency};
use crate::plushie::{Plushie, PlushieTrait};

pub fn rank(specimen: &Vec<u8>, judge: &impl Comparator) -> f32 {
    let actions: Vec<Action> = genetic::v1::express_genes(specimen);
    let mut params = params::handpicked::pillar();
    params.hook_leniency = Leniency::SkipIncorrect;
    let mut plushie = match Plushie::from_flow(SimpleFlow::new(actions), params) {
        Ok(plushie) => plushie,
        Err(err) => {
            println!(
                "Encountered error \"{}\" when parsing genome: {:?}",
                err, specimen
            );
            return -f32::INFINITY;
        }
    };
    plushie.animate();
    let nodes: Vec<Point> = serde_json::from_value(plushie.nodes_to_json()).unwrap();
    judge.judge(&nodes)
}
