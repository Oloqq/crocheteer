use serde_derive::{Deserialize, Serialize};

#[allow(unused)]
use super::common::*;
use std::fmt::Display;

type Probability = f64;

#[derive(Clone, Serialize, Deserialize)]
pub struct GrowingParams {}

#[derive(Clone, Serialize, Deserialize)]
pub struct Params {
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

    pub levels: Option<usize>,
    pub max_height: Option<f32>,
}

impl Default for GrowingParams {
    fn default() -> Self {
        Self {
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
            memsize: 5,
            popsize: 10,
            max_size: 1000,
            p_crossover: 0.0,
            p_mut_per_node: 0.05,
            tournament_size: 2,
            acceptable_error: -1e-3,
            random_initial_memory: false,
            growing: Default::default(),
            prefix: vec![],
            suffix: vec![],
            levels: None,
            max_height: None,
        }
    }
}

impl Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "
POPSIZE={}
CROSSOVER_PROB={}
PMUT_PER_NODE={}
TSIZE={}
----------------------------------\n",
                self.popsize, self.p_crossover, self.p_mut_per_node, self.tournament_size
            )
            .as_str(),
        )
    }
}
