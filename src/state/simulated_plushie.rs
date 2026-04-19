use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource)]
pub struct PlushieInSimulation {
    pub definition: crochet::PlushieDef,
    pub plushie: crochet::simulated_plushie::SimulatedPlushie,
    pub index_to_entity: HashMap<usize, Entity>,
}
