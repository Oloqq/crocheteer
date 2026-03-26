use crate::ui::ui_used_input::UiUsedInput;
use crate::ui::utils::{CanGoOffscreen, require_width_for_slider, using_resizer};
use crate::ui::{data::*, utils::full_height_button};
use bevy::prelude::*;
use bevy_egui::egui::UiBuilder;
use bevy_egui::egui::panel::Side;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};

fn expanded_ui(
    ui: &mut egui::Ui,
    state: &mut UiState,
    collapsed: &mut bool,
    mut timestep: ResMut<Time<Fixed>>,
) {
    ui.horizontal(|ui| {
        ui.heading("Controls    "); // spaces prevent overlapping with the right-aligned button
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
            require_width_for_slider(ui); // make sure the sliding part of the slider is on screen with CanGoOffscreen

            CanGoOffscreen::new().show(ui, |ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut state.sim_speed, 1.0..=32.0)
                            .logarithmic(true)
                            .text("Tickrate"),
                    )
                    .on_hover_text("Multiplies the rate of simulation ticks")
                    .changed()
                {
                    timestep.set_timestep_hz(64.0 * state.sim_speed);
                }
            });

            // ui.selectable_label(true, "bruh");
            // ui.selectable_label(false, "bruh");
            // ui.checkbox(&mut true, "pause");
            // ui.checkbox(&mut false, "pause");
            // ui.label("Global force multiplier");
            // if ui
            //     .add(egui::Slider::new(&mut state.sim_speed, 0.1..=32.0))
            //     .changed()
            // {
            //     // TODO
            // }

            // ui.horizontal(|ui| {
            //     ui.label("Color");
            //     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            //         if ui.button("random").clicked() {
            //             state.r = 4; // chosen by dice roll, guaranteed to be random
            //             state.g = 4; // chosen by dice roll, guaranteed to be random
            //             state.b = 4; // chosen by dice roll, guaranteed to be random
            //         }
            //     });
            // });
            // ui.add(egui::Slider::new(&mut state.r, 0..=255).text("R"));
            // ui.add(egui::Slider::new(&mut state.g, 0..=255).text("G"));
            // ui.add(egui::Slider::new(&mut state.b, 0..=255).text("B"));

            // ui.separator();
            // ui.horizontal(|ui| {
            //     ui.label("Label: ");
            //     ui.text_edit_singleline(&mut state.label);
            // });

            // egui::CollapsingHeader::new("Many labels")
            //     .default_open(false)
            //     .show(ui, |ui| {
            //         for num in 0..100 {
            //             let mut s = String::from("Label ");
            //             for _ in 0..num {
            //                 s.push('a');
            //             }
            //             ui.label(s);
            //         }
            //     });
        });
}

pub fn control_panel(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    ui_used_input: Res<UiUsedInput>,
    mut collapsed: Local<bool>,
    timestep: ResMut<Time<Fixed>>,
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
                expanded_ui(ui, &mut ui_state, &mut collapsed, timestep);
            }
        },
    );

    // prevent world events on resizing
    if !*collapsed && using_resizer(ctx, extended_panel_id, Side::Right) {
        ui_used_input.set_true();
    }

    Ok(())
}
