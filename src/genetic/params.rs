use serde_derive::{Deserialize, Serialize};

use super::common::*;

type Probability = f64;

#[derive(Clone, Serialize, Deserialize)]
pub struct Params {
    /// Population size
    pub popsize: usize,
    /// Max size of Vec<Token> representing a program. Ignored during initial generation.
    pub max_size: usize,
    /// Probability of performing crossover over mutation.
    pub p_crossover: Probability,
    /// If mutation was chosen over crossover, probability for mutating each token.
    pub p_mutation_per_node: Probability,
    /// Number of programs to choose from, when looking for best/worst fitness
    pub tournament_size: usize,
    /// Minimum fitness required to consider the program fitted. Keep in mind fitness is negative and reaches 0 on perfect program.
    pub acceptable_error: f32,

    pub prefix: Vec<Token>,
    pub suffix: Vec<Token>,
    pub levels: Option<usize>,
    pub max_height: Option<f32>,
}

// impl Default for GrowingParams {
//     fn default() -> Self {
//         Self {
//             // d_expr: vec![
//             //     (Expr::ADD, 1),
//             //     (Expr::SUB, 1),
//             //     (Expr::MUL, 1),
//             //     (Expr::DIV, 1),
//             //     (Expr::EQ, 1),
//             //     (Expr::LT, 1),
//             //     (Expr::GT, 1),
//             //     (Expr::OR, 1),
//             //     (Expr::AND, 1),
//             //     (Expr::NOT, 1),
//             //     (Expr::Num(PLACEHOLDER as i32), 1),
//             //     (Expr::Reg(PLACEHOLDER), 1),
//             // ],
//             // d_stat: vec![
//             //     (Stat::LOAD, 1),
//             //     (Stat::IF, 1),
//             //     (Stat::WHILE, 0),
//             //     (Stat::INPUT, 1),
//             //     (Stat::OUTPUT, 1),
//             // ],
//         }
//     }
// }

impl Default for Params {
    fn default() -> Self {
        Self {
            popsize: 10,
            max_size: 1000,
            p_crossover: 0.0,
            p_mutation_per_node: 0.05,
            tournament_size: 2,
            acceptable_error: -1e-3,
            prefix: vec![],
            suffix: vec![],
            levels: None,
            max_height: None,
        }
    }
}
