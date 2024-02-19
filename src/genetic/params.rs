#[allow(unused)]
use super::common::*;
use std::fmt::Display;

type Number = i32;

pub type Case = (Input, Output);
type Probability = f64;

#[derive(Clone)]
pub struct GrowingParams {
    /// inclusive
    pub min_const: Number,
    /// exclusive
    pub max_const: Number,
    /// Expressions can easily grow large due to their recursive structure. This is the probability of terminating the expression with a non-recursive term (e.g. Reg or Num). Programs may fail to finish generating if set too low. In the future should probably be replaced with a function.
    pub p_expression_plug: Probability,
    /// When plugging, affinity towards Reg vs Num
    pub p_prefer_reg_over_num: Probability,
    /// Weights used when growing expressions
    // pub d_expr: Vec<(Expr, i32)>,
    // /// Weights used when growing statements
    // pub d_stat: Vec<(Stat, i32)>,
    /// Probability of inserting a brand new statement when mutating one
    pub p_insertion: Probability,
}

#[derive(Clone)]
pub struct Params {
    pub seed: u64,
    pub memsize: usize,
    pub popsize: usize,
    /// Max size of Vec<Token> representing a program. Ignored during initial generation.
    pub max_size: usize,
    pub p_crossover: Probability,
    pub p_mut_per_node: Probability,
    pub tournament_size: usize,
    /// Minimum fitness required to consider the program fitted. Must be negative.
    pub acceptable_error: f32,
    pub growing: GrowingParams,
    pub random_initial_memory: bool,
    pub prefix: Vec<Token>,
    pub suffix: Vec<Token>,
}

impl Params {
    // pub fn from_string(data: String) -> Result<(Params, Vec<Case>), Box<dyn Error>> {
    //     let lines: Vec<&str> = data.split('\n').collect();
    //     let header: Vec<&str> = lines[0].trim().split([' ', '\t']).collect();
    //     let memsize: usize = header[0].parse()?;
    //     let separator: &str = header[1];
    //     let num_cases: usize = header[2].parse()?;

    //     let mut cases: Vec<Case> = Vec::with_capacity(num_cases);
    //     for i in 0..num_cases {
    //         let tokens: Vec<&str> = lines[i + 1]
    //             .trim()
    //             .split([' ', '\t'])
    //             .filter(|t| !t.is_empty())
    //             .collect();
    //         let split_pos = tokens
    //             .iter()
    //             .position(|&t| t == separator)
    //             .expect("No pipe '|' found in the input");
    //         let (inputs, pipe_and_outputs) = tokens.split_at(split_pos);
    //         let outputs = &pipe_and_outputs[1..];
    //         let inputs = inputs
    //             .iter()
    //             .map(|t| t.parse().unwrap())
    //             .collect::<Vec<Number>>();

    //         let outputs = outputs
    //             .iter()
    //             .map(|t| t.parse().unwrap())
    //             .collect::<Vec<Number>>();

    //         cases.push((Vec::from(inputs), Vec::from(outputs)));
    //     }

    //     Ok((
    //         Params {
    //             seed: 5,
    //             memsize,
    //             ..Default::default()
    //         },
    //         cases,
    //     ))
    // }
}

impl Default for GrowingParams {
    fn default() -> Self {
        const PLACEHOLDER: usize = 0;

        Self {
            min_const: -100,
            max_const: 100,
            p_expression_plug: 0.8, // TODO this should really be replaced by a function that increases in value as expression get longer
            p_prefer_reg_over_num: 0.5,
            p_insertion: 0.1,
            // d_expr: vec![
            //     (Expr::ADD, 1),
            //     (Expr::SUB, 1),
            //     (Expr::MUL, 1),
            //     (Expr::DIV, 1),
            //     (Expr::EQ, 1),
            //     (Expr::LT, 1),
            //     (Expr::GT, 1),
            //     (Expr::OR, 1),
            //     (Expr::AND, 1),
            //     (Expr::NOT, 1),
            //     (Expr::Num(PLACEHOLDER as i32), 1),
            //     (Expr::Reg(PLACEHOLDER), 1),
            // ],
            // d_stat: vec![
            //     (Stat::LOAD, 1),
            //     (Stat::IF, 1),
            //     (Stat::WHILE, 0),
            //     (Stat::INPUT, 1),
            //     (Stat::OUTPUT, 1),
            // ],
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            seed: Default::default(),
            memsize: 5,
            popsize: 10,
            max_size: 1000,
            p_crossover: 0.9,
            p_mut_per_node: 0.05,
            tournament_size: 2,
            acceptable_error: -1e-3,
            random_initial_memory: false,
            growing: Default::default(),
            prefix: vec![],
            suffix: vec![],
        }
    }
}

impl Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "SEED={}
POPSIZE={}
CROSSOVER_PROB={}
PMUT_PER_NODE={}
TSIZE={}
----------------------------------\n",
                self.seed,
                self.popsize,
                self.p_crossover,
                self.p_mut_per_node,
                self.tournament_size
            )
            .as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::genetic::params::Params;

    //     #[test]
    //     fn test_read_params() {
    //         let (_param, cases) = match Params::from_string(
    //             "2 | 3
    // 2 2 | 4
    // 1 3 | 4
    // 10 20 | 30
    // "
    //             .to_owned(),
    //         ) {
    //             Ok(p) => p,
    //             Err(_) => panic!("Read problem failed"),
    //         };

    //         assert_eq!(cases.len(), 3);
    //         cases.iter().for_each(|(inputs, targets)| {
    //             assert_eq!(inputs.len(), 2);
    //             assert_eq!(targets.len(), 1);
    //         });
    //     }
}
