use super::{
    common::{Output, Program},
    execution::Runtime,
};

pub type FitnessFunc = fn(expected: &Output, actual: &Output, runtime: &Runtime) -> f32;

// pub fn diff_first(expected: &Vec<Number>, actual: &Vec<Number>, _runtime: &Runtime) -> f32 {
//     let output = actual.get(0).unwrap_or(&0);
//     let expected = expected[0];
//     let error = (*output as f32 - expected as f32).abs();
//     -error
// }

pub fn normalize_fitness(fitness: &Vec<f32>, _programs: &Vec<Program>) -> Vec<f64> {
    // assert_eq!(fitness.len(), programs.len());
    // let fitness_max = fitness.iter().max_by(|a, b| a.total_cmp(b)).unwrap().clone() as f64;
    // let prog_len_max = programs.iter().map(|p| p.len()).max().unwrap().clone() as f64;
    // fitness.iter().zip(programs).map(|(fit, prog)| {
    //     (1.3 - (prog.len() as f64 / prog_len_max)) * (*fit as f64 / fitness_max)
    // }).collect()
    fitness.iter().map(|f| *f as f64).collect()
}

#[cfg(test)]
mod tests {
    // use super::*;
}
