use super::{
    common::{Output, Program},
    execution::Runtime,
    shapes::compare_shapes,
};

pub type FitnessFunc = fn(expected: &Output, actual: &Output, runtime: &Runtime) -> f32;

pub fn shape_fitness(expected: &Output, actual: &Output, _runtime: &Runtime) -> f32 {
    compare_shapes(expected, actual)
}

pub fn normalize_fitness(fitness: &Vec<f32>, _programs: &Vec<Program>) -> Vec<f64> {
    fitness.iter().map(|f| *f as f64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
}
