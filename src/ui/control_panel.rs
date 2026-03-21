use crate::ui::input_capture::UiUsedInput;
use crate::ui::utils::using_resizer;
use crate::ui::{data::*, utils::full_height_button};
use bevy::prelude::*;
use bevy_egui::egui::panel::Side;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};

fn expanded_ui(ui: &mut egui::Ui, state: &mut UiState, collapsed: &mut bool) {
    ui.horizontal(|ui| {
        ui.heading("Side Panel");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("▶").clicked() {
                *collapsed = true;
            }
        });
    });
    ui.separator();

    let scroll_width = ui.spacing().scroll.bar_width + ui.spacing().scroll.bar_inner_margin;
    let available_width = ui.available_width() - scroll_width;
    egui::ScrollArea::vertical()
        .auto_shrink([true, true])
        .show(ui, |ui| {
            ui.set_max_width(available_width); // prevent infinite panel growth when scrollbar appears and disappears

            ui.horizontal(|ui| {
                ui.label("Color");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("random").clicked() {
                        state.r = 4; // chosen by dice roll, guaranteed to be random
                        state.g = 4; // chosen by dice roll, guaranteed to be random
                        state.b = 4; // chosen by dice roll, guaranteed to be random
                    }
                });
            });
            ui.add(egui::Slider::new(&mut state.r, 0..=255).text("R"));
            ui.add(egui::Slider::new(&mut state.g, 0..=255).text("G"));
            ui.add(egui::Slider::new(&mut state.b, 0..=255).text("B"));

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Label: ");
                ui.text_edit_singleline(&mut state.label);
            });

            egui::CollapsingHeader::new("Many labels")
                .default_open(false)
                .show(ui, |ui| {
                    for num in 0..100 {
                        let mut s = String::from("Label ");
                        for _ in 0..num {
                            s.push('a');
                        }
                        ui.label(s);
                    }
                });
        });
}

pub fn ui_example_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    captured: Res<UiUsedInput>,
    mut collapsed: Local<bool>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let extended_panel_id = egui::Id::new("side_panel_extended");

    egui::SidePanel::show_animated_between(
        ctx,
        *collapsed,
        egui::SidePanel::right(extended_panel_id).resizable(true),
        egui::SidePanel::right("side_panel_collapsed")
            .exact_width(24.0)
            .resizable(false),
        |ui, _| {
            if *collapsed {
                let response = full_height_button(
                    ui,
                    ui.id().with("collapse_toggle_right"),
                    ui.clip_rect(),
                    "◀",
                );
                if response.clicked() {
                    *collapsed = false;
                }
            } else {
                expanded_ui(ui, &mut ui_state, &mut collapsed);
            }
        },
    );

    // hacky hack to ensure grabbing the resize bar is registered as an input "wanted by egui"
    if !*collapsed && using_resizer(ctx, extended_panel_id, Side::Right) {
        captured.capture();
    }

    Ok(())
}
