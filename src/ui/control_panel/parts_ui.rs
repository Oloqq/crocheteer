use bevy_egui::egui;

use crate::ui::{SimulationState, action_item::UiActionItem};

pub fn parts_ui(ui: &mut egui::Ui, state: &mut SimulationState) {
    let any_part_active = state.active_part.is_some();
    let mut placeholder: String = "(no parts)".into();
    let active_part = if let Some(active_part) = &mut state.active_part {
        active_part
    } else {
        &mut placeholder
    };

    ui.add_enabled_ui(any_part_active, |ui| {
        egui::ComboBox::new("part_selection", "Part")
            .selected_text(active_part.as_str())
            .show_ui(ui, |ui| {
                for part_option in &state.parts {
                    ui.selectable_value(active_part, part_option.into(), part_option);
                }
            });

        if ui.button("Select all nodes").clicked() {
            state
                .action_items
                .push(UiActionItem::SelectNodesOfPart(active_part.clone()));
        }
    });
}
