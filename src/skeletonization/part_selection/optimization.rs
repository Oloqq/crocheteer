use std::error::Error;

// microlp was chosen purely because it installed without demanding any external libraries
use good_lp::{
    constraint, microlp, variables, Constraint, Expression, Solution, SolverModel, Variable,
};

// Solve the optimization problem
pub fn solve_optimization(
    c: &Vec<f32>,
    a: &Vec<usize>,
    q: &Vec<Vec<usize>>,
    min_nodes: usize,
    max_overlap: usize,
) -> Result<Vec<bool>, Box<dyn Error>> {
    assert_eq!(c.len(), a.len());
    assert_eq!(c.len(), q.len());
    let size = c.len();
    variables! {vars: x[size] (binary); }

    let solution = vars
        .minimise(total_cost(&x, &c))
        .using(microlp) // multiple solvers available
        .with(must_select(&x, &a, min_nodes))
        .with(allowed_overlap(&x, &q, max_overlap))
        .solve()?;

    Ok((0..c.len()).map(|i| solution.value(x[i]) > 0.0).collect())
}

fn total_cost(x: &Vec<Variable>, c: &Vec<f32>) -> Expression {
    assert_eq!(x.len(), c.len());
    x.iter().zip(c).map(|(x, c)| *x * *c).sum::<Expression>()
}

fn must_select(x: &Vec<Variable>, a: &Vec<usize>, min_nodes: usize) -> Constraint {
    assert_eq!(x.len(), a.len());
    let sum_selected = x
        .iter()
        .zip(a)
        .map(|(x, a)| *x * *a as f32)
        .sum::<Expression>();
    let min_nodes = min_nodes as f64;
    constraint!(sum_selected >= Expression::from(min_nodes))
}

fn allowed_overlap(x: &Vec<Variable>, q: &Vec<Vec<usize>>, max_allowed: usize) -> Constraint {
    let mut overlap = Expression::with_capacity(1);
    for i in 0..x.len() {
        for j in i + 1..x.len() {
            let overlap_here = q[i][j] as f64;
            let mut e = Expression::from(x[i]);
            e.add_mul(overlap_here, x[j]);
            overlap += e;
        }
    }
    let max_allowed = max_allowed as f64;
    constraint!(overlap <= Expression::from(max_allowed))
}
