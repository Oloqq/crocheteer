use bevy::ecs::system::ResMut;
use bevy_egui::egui;

use crate::{
    state::simulated_plushie::PlushieInSimulation,
    ui::{SimulationState, action_item::UiActionItem},
};

pub fn parts_ui(
    ui: &mut egui::Ui,
    state: &mut SimulationState,
    current_plushie: &mut Option<ResMut<PlushieInSimulation>>,
) {
    let (mut placeholder_name, mut placeholder_part) = placeholders();
    let (using_placeholders, part_name, part, part_names) = get_part_to_configure(
        &mut placeholder_name,
        &mut placeholder_part,
        &mut state.active_part,
        current_plushie,
    );

    ui.add_enabled_ui(!using_placeholders, |ui| {
        egui::ComboBox::new("part_selection", "Part")
            .selected_text(part_name.as_str())
            .show_ui(ui, |ui| {
                for part_option in &part_names {
                    ui.selectable_value(part_name, part_option.into(), part_option);
                }
            });

        ui.add(egui::Slider::new(&mut part.parameters.centroids, 0..=20).text("Centroids"))
            .on_hover_text("Number of stuffing centroids. Bigger plushies need more centroids");

        if ui.button("Select all nodes").clicked() {
            state
                .action_items
                .push(UiActionItem::SelectNodesOfPart(part_name.clone()));
        }
    });
}

fn get_part_to_configure<'a>(
    placeholder_name: &'a mut String,
    placeholder_part: &'a mut crochet::acl::Part,
    active_part: &'a mut Option<String>,
    current_plushie: &'a mut Option<ResMut<PlushieInSimulation>>,
) -> (
    bool,
    &'a mut String,
    &'a mut crochet::acl::Part,
    Vec<String>,
) {
    let using_placeholders = active_part.is_none() || current_plushie.is_none();

    let part_names = match current_plushie {
        Some(plushie) => plushie
            .plushie
            .pattern
            .parts
            .iter()
            .map(|p| p.name.clone())
            .collect(),
        None => vec![placeholder_name.clone()],
    };

    let ui_part_name: &mut String = if let Some(active_part) = active_part.as_mut() {
        active_part
    } else {
        placeholder_name
    };

    let real_parts: Option<&mut Vec<crochet::acl::Part>> = current_plushie
        .as_mut()
        .map(|plushie| &mut plushie.plushie.pattern.parts);

    let ui_part = if let Some(r) = real_parts {
        let it: Option<&mut crochet::acl::Part> = r.iter_mut().find(|p| &p.name == ui_part_name);
        if let Some(a) = it {
            a
        } else {
            placeholder_part
        }
    } else {
        placeholder_part
    };

    (using_placeholders, ui_part_name, ui_part, part_names)
}

fn placeholders() -> (String, crochet::acl::Part) {
    (
        "(no parts)".into(),
        crochet::acl::Part {
            name: "(no parts)".into(),
            instances: 1,
            actions: Default::default(),
            parameters: Default::default(),
        },
    )
}
