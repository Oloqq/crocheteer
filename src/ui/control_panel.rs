use crate::plushie::{DisplayMode, SetDisplayMode};
use crate::ui::SimulationState;
use crate::ui::ui_used_input::UiUsedInput;
use crate::ui::utils::full_height_button;
use crate::ui::utils::{CanGoOffscreen, require_width_for_slider, using_resizer};
use bevy::prelude::*;
use bevy_egui::egui::panel::Side;
use bevy_egui::egui::{Context, Ui};
use bevy_egui::{
    EguiContexts,
    egui::{self},
};

pub fn control_panel(
    mut state: ResMut<SimulationState>,
    mut contexts: EguiContexts,
    ui_used_input: Res<UiUsedInput>, // atomically mutable
    mut display_mode_msg: MessageWriter<SetDisplayMode>,
    mut collapsed: Local<bool>,
    mut timestep: ResMut<Time<Fixed>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let mut panel = RightPanel::new();
    panel.show_with_default_collapsed(ctx, &mut collapsed, |ui| {
        ui.checkbox(&mut state.paused, "Paused");
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
                egui::Slider::new(&mut state.force_multiplier, 0.1..=2.0).text("Force multiplier"),
            )
            .on_hover_text("Multiplies all forces applied. High values can cause glitches.");

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

        ui.separator();
        ui.add(egui::Slider::new(&mut state.centroids, 0..=20).text("Centroids"))
                .on_hover_text("Number of stuffing centroids. Bigger plushies need more centroids. Acts as maximum when using \"Nodes per centroid\" setting");
            // ui.add(egui::Slider::new(&mut state.nodes_per_centroid, 0..=100).text("Nodes per centroid"))
            //     .on_hover_text("");
    });

    // prevent world events on resizing
    if !*collapsed && using_resizer(ctx, panel.extended_panel_id, Side::Right) {
        ui_used_input.set_true();
    }

    Ok(())
}

pub struct RightPanel {
    pub extended_panel_id: egui::Id,
}

impl RightPanel {
    pub fn new() -> Self {
        Self {
            extended_panel_id: egui::Id::new("right_side_panel_extended"),
        }
    }

    pub fn show_with_default_collapsed<R>(
        &mut self,
        ctx: &Context,
        collapsed: &mut bool,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) {
        egui::SidePanel::show_animated_between(
            ctx,
            *collapsed,
            egui::SidePanel::right(self.extended_panel_id).resizable(true),
            egui::SidePanel::right("right_side_panel_collapsed")
                .exact_width(24.0)
                .resizable(false),
            |ui, _| {
                if *collapsed {
                    self.collapsed_ui(collapsed, ui);
                } else {
                    ui.horizontal(|ui| {
                        ui.heading("Simulation     "); // spaces prevent overlapping with the right-aligned button
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("▶").clicked() {
                                *collapsed = true;
                            }
                        });
                    });
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .auto_shrink([true, true])
                        .show(ui, |ui| {
                            let scroll_width = ui.spacing().scroll.bar_width
                                + ui.spacing().scroll.bar_inner_margin;
                            let available_width = ui.available_width() - scroll_width;
                            ui.set_max_width(available_width); // prevent infinite panel growth when scrollbar appears and disappears
                            require_width_for_slider(ui); // make sure the sliding part of the slider is on screen with CanGoOffscreen
                            CanGoOffscreen::new().show(ui, |ui| {
                                add_contents(ui);
                            });
                        });
                }
            },
        );
    }

    fn collapsed_ui(&mut self, collapsed: &mut bool, ui: &mut Ui) {
        let response = full_height_button(
            ui,
            ui.id().with("collapse_toggle_right"),
            ui.clip_rect(),
            "◀",
        );
        if response.clicked() {
            *collapsed = false;
        }
    }
}
