use bevy::prelude::*;

use crate::{
    plushie::data::{GraphNode, Selected},
    state::simulated_plushie::PlushieInSimulation,
    ui::SimulationState,
};

pub enum UiActionItem {
    SelectNodesOfPart(String),
}

pub fn complete_action_items(
    mut state: ResMut<SimulationState>,
    mut commands: Commands,
    plushie: Option<Res<PlushieInSimulation>>,
    nodes: Query<(Entity, &GraphNode)>,
) {
    let items = std::mem::take(&mut state.action_items);
    use UiActionItem::*;
    for item in items {
        match item {
            SelectNodesOfPart(part_name) => {
                let Some(plushie) = &plushie else {
                    return;
                };
                let Some(part_index) = plushie
                    .definition
                    .pattern
                    .parts
                    .iter()
                    .enumerate()
                    .find_map(|(i, p)| (p.name == part_name).then_some(i))
                else {
                    return;
                };

                for (entity, _) in nodes
                    .iter()
                    .filter(|(_, node)| node.part_index == part_index)
                {
                    commands.entity(entity).insert(Selected);
                }
            }
        }
    }
}
