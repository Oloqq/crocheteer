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
    let mut context = PartContext::from_state(&mut state.active_part, current_plushie);
    let (using_placeholders, part_name, part, all_part_names): (
        bool,
        &mut String,
        &mut crochet::force_graph::simulated_plushie::Part,
        Vec<String>,
    ) = match &mut context {
        PartContext::Placeholder { name, part } => {
            let all_names = vec![name.clone()];
            (true, name, part, all_names)
        }
        PartContext::Active {
            name,
            part,
            all_names,
        } => (false, name, part, all_names.clone()),
    };

    ui.add_enabled_ui(!using_placeholders, |ui| {
        egui::ComboBox::new("part_selection", "Part")
            .selected_text(part_name.as_str())
            .show_ui(ui, |ui| {
                for part_option in &all_part_names {
                    ui.selectable_value(part_name, part_option.into(), part_option);
                }
            });

        ui.add(egui::Slider::new(&mut part.centroids_wanted, 0..=20).text("Centroids"))
            .on_hover_text(CENTROID_NUMBER_HELP);

        if ui.button("Select all nodes").clicked() {
            state
                .action_items
                .push(UiActionItem::SelectNodesOfPart(part_name.clone()));
        }
    });
}

enum PartContext<'a> {
    Placeholder {
        name: String,
        part: crochet::force_graph::simulated_plushie::Part,
    },
    Active {
        name: &'a mut String,
        part: &'a mut crochet::force_graph::simulated_plushie::Part,
        all_names: Vec<String>,
    },
}

impl<'a> PartContext<'a> {
    fn from_state(
        active_part: &'a mut Option<String>,
        current_plushie: &'a mut Option<ResMut<PlushieInSimulation>>,
    ) -> Self {
        let (active_part, plushie) = match (active_part.as_mut(), current_plushie.as_mut()) {
            (Some(name), Some(p)) => (name, p),
            _ => return Self::placeholder(),
        };

        let all_names = plushie
            .definition
            .pattern
            .parts
            .iter()
            .map(|p| p.name.clone())
            .collect();

        let part = plushie
            .plushie
            .parts
            .iter_mut()
            .find(|p| p.name() == active_part);

        match part {
            Some(part) => Self::Active {
                name: active_part,
                part,
                all_names,
            },
            None => Self::placeholder(),
        }
    }

    fn placeholder() -> Self {
        Self::Placeholder {
            name: "(no parts)".into(),
            part: crochet::force_graph::simulated_plushie::Part::mock("(no parts)".into()),
        }
    }
}

const CENTROID_NUMBER_HELP: &'static str = "Number of stuffing centroids. Bigger plushies need more centroids. Changes will be visible only if simulation is running.";
