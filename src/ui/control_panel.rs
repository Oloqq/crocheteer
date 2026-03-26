use crate::plushie::{DisplayMode, SetDisplayMode};
use crate::ui::ui_used_input::UiUsedInput;
use crate::ui::utils::{CanGoOffscreen, require_width_for_slider, using_resizer};
use crate::ui::{data::*, utils::full_height_button};
use bevy::prelude::*;
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
    mut display_mode_msg: MessageWriter<SetDisplayMode>,
) {
    ui.horizontal(|ui| {
        ui.heading("Simulation    "); // spaces prevent overlapping with the right-aligned button
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

            ui.checkbox(&mut state.paused, "Paused");
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

                ui.add(
                    egui::Slider::new(&mut state.force_multiplier, 0.1..=2.0)
                        .text("Force multiplier"),
                )
                .on_hover_text("Multiplies all forces applied. High values can cause glitches.");
            });

            ui.collapsing("Display mode", |ui| {
                let previous_mode = state.display_mode;
                ui.radio_value(&mut state.display_mode, DisplayMode::Pattern, "Pattern")
                    .on_hover_text("Use colors defined in the pattern. Big stitches, small links.");
                // ui.radio_value(&mut state.display_mode, DisplayMode::Stitches, "Stitches"); // TODO differentiate stitch kind (sc vs inc etc)
                ui.radio_value(&mut state.display_mode, DisplayMode::Forces, "Link forces")
                    .on_hover_text("Show the forces applied by links. Big links, tiny stitches");
                // TODO link forces with grabbable stitches (just make stitches bigger, or keep them small visually but spawn bigger invisible spheres on top. Ideally they would visually get bigger when hovered over)
                // TODO forces applied by centroids
                //  - present them like links? - will be barely readable. Actually this could work if actual links and stitches become hidden and s
                //  - color the stitches with a shader?
                if state.display_mode != previous_mode {
                    display_mode_msg.write(SetDisplayMode {
                        mode: state.display_mode,
                    });
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
    ui_used_input: Res<UiUsedInput>, // atomically mutable
    display_mode_msg: MessageWriter<SetDisplayMode>,
    mut collapsed: Local<bool>,
    timestep: ResMut<Time<Fixed>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let extended_panel_id = egui::Id::new("side_panel_extended");

    // TODO make it a Widget, that only takes the expanded ui as lambda
    // that way there's no pain in passing ECS components and no indentation hell
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
                expanded_ui(
                    ui,
                    &mut ui_state,
                    &mut collapsed,
                    timestep,
                    display_mode_msg,
                );
            }
        },
    );

    // prevent world events on resizing
    if !*collapsed && using_resizer(ctx, extended_panel_id, Side::Right) {
        ui_used_input.set_true();
    }

    Ok(())
}
