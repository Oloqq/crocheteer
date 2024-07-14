use crate::common::*;
use crate::comparison::Comparator;
use crate::flow::actions::Action;
use crate::flow::ergoflow::ErgoFlow;
use crate::plushie::{Params, Plushie, PlushieTrait};

fn genome_to_actions(genome: &Vec<u8>) -> Vec<Action> {
    use Action::*;
    let mut result = vec![MR(6)];

    for gene in genome {
        result.push(match *gene {
            0 => Sc,
            1 => Inc,
            2 => Dec,
            3 => FLO,
            4 => BLO,
            _ => panic!("Unrecognized gene: {}", gene),
        });
    }

    result.push(FO);
    result
}

pub fn rank(specimen: &Vec<u8>, judge: &impl Comparator) -> f32 {
    let actions: Vec<Action> = genome_to_actions(specimen);
    let mut plushie =
        match Plushie::from_flow(ErgoFlow::from(actions), Params::handpicked_for_grzib()) {
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
