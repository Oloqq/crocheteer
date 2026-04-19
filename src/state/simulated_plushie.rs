use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource)]
pub struct PlushieInSimulation {
    pub definition: crochet::PlushieDef,
    pub plushie: crochet::force_graph::simulated_plushie::SimulatedPlushie,
    pub node_lookup: NodeLookup,
}

pub struct NodeLookup {
    pub index_to_entity: HashMap<usize, Entity>,
    pub entity_to_index: HashMap<Entity, usize>,
}

impl NodeLookup {
    pub fn new() -> Self {
        Self {
            index_to_entity: HashMap::new(),
            entity_to_index: HashMap::new(),
        }
    }
}

impl PlushieInSimulation {
    pub fn root_node_at(&mut self, entity: Entity, pos: Vec3) {
        let Some(index) = self.node_lookup.entity_to_index.get(&entity) else {
            warn!("no index for node");
            return;
        };
        self.plushie.root_node_at(*index, pos);
    }

    pub fn unroot_node(&mut self, entity: Entity) {
        let Some(index) = self.node_lookup.entity_to_index.get(&entity) else {
            warn!("no index for node");
            return;
        };
        self.plushie.unroot_node(*index);
    }
}
