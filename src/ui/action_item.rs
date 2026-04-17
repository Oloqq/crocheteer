use bevy::prelude::*;

use crate::ui::SimulationState;

pub enum UiActionItem {
    SelectNodesOfPart(String),
}

pub fn complete_action_items(mut state: ResMut<SimulationState>) {
    let items = std::mem::take(&mut state.action_items);
    use UiActionItem::*;
    for item in items {
        match item {
            SelectNodesOfPart(_part_name) => todo!(),
        }
    }
}
