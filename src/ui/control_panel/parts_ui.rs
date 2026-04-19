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
        &mut crochet::acl::Part,
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

        ui.add(egui::Slider::new(&mut part.parameters.centroids, 0..=20).text("Centroids"))
            .on_hover_text("Number of stuffing centroids. Bigger plushies need more centroids");

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
        part: crochet::acl::Part,
    },
    Active {
        name: &'a mut String,
        part: &'a mut crochet::acl::Part,
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
            .plushie
            .pattern
            .parts
            .iter()
            .map(|p| p.name.clone())
            .collect();

        let part = plushie
            .plushie
            .pattern
            .parts
            .iter_mut()
            .find(|p| &p.name == active_part);

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
            part: crochet::acl::Part {
                name: "(no parts)".into(),
                instances: 1,
                actions: Default::default(),
                parameters: Default::default(),
            },
        }
    }
}
