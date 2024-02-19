use super::{
    common::{Output, Program},
    execution::Runtime,
};

pub type FitnessFunc = fn(expected: &Output, actual: &Output, runtime: &Runtime) -> f32;

pub fn compare_shapes(expected: &Output, actual: &Output, _runtime: &Runtime) -> f32 {
    0.0
}

pub fn normalize_fitness(fitness: &Vec<f32>, _programs: &Vec<Program>) -> Vec<f64> {
    fitness.iter().map(|f| *f as f64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_cylinders() {}
}
