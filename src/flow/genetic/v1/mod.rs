use super::super::actions::Action;

pub fn express_genes(genome: &Vec<u8>) -> Vec<Action> {
    use Action::*;
    let mut result = vec![MR(6)];

    for gene in genome {
        result.push(match *gene {
            0 => Sc,
            1 => Inc,
            2 => Dec,
            _ => panic!("Unrecognized gene: {}", gene),
        });
    }

    result.push(FO);
    result
}
