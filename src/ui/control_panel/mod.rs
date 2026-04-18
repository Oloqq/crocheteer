mod parts_ui;
mod right_panel;

use crate::FIXED_UPDATE_BASE_HZ;
use crate::plushie::{DisplayMode, SetDisplayMode};
use crate::state::simulated_plushie::PlushieInSimulation;
use crate::ui::SimulationState;
use crate::ui::control_panel::parts_ui::parts_ui;
use crate::ui::control_panel::right_panel::RightPanel;
use crate::ui::ui_used_input::UiUsedInput;
use crate::ui::utils::using_resizer;
use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_egui::egui::panel::Side;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};
use crochet::force_graph::Initializer;

pub fn control_panel(
    mut state: ResMut<SimulationState>,
    mut contexts: EguiContexts,
    ui_used_input: Res<UiUsedInput>, // atomically mutable
    mut display_mode_msg: MessageWriter<SetDisplayMode>,
    mut collapsed: Local<bool>,
    mut timestep: ResMut<Time<Fixed>>,
    mut current_plushie: Option<ResMut<PlushieInSimulation>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let mut panel = RightPanel::new();
    panel.show_with_default_collapsed(ctx, &mut collapsed, |ui| {
        ui.checkbox(&mut state.paused, "Paused");
        let tickrate_response = ui
            .add(
                egui::Slider::new(&mut state.sim_speed, 1.0..=32.0)
                    .logarithmic(true)
                    .text("Tickrate"),
            )
            .on_hover_text("Multiplies the rate of simulation ticks");
        if tickrate_response.changed() {
            timestep.set_timestep_hz(FIXED_UPDATE_BASE_HZ * state.sim_speed);
        }

        ui.add(egui::Slider::new(&mut state.force_multiplier, 0.1..=2.0).text("Force multiplier"))
            .on_hover_text("Multiplies all forces applied. High values can cause glitches.");

        ui.collapsing("Display mode", |ui| {
            let previous_mode = state.display_mode;
            display_mode(ui, &mut state);
            if state.display_mode != previous_mode {
                display_mode_msg.write(SetDisplayMode {
                    mode: state.display_mode,
                });
            }
        });

        ui.collapsing("Node initialization", |ui| {
            ui.radio_value(
                &mut state.initializer,
                Initializer::RegularCylinder(12),
                "Cylinder",
            )
            .on_hover_text("Spawn all nodes at once in a shape of a cylinder.");
            ui.radio_value(&mut state.initializer, Initializer::OneByOne, "One by one")
                .on_hover_text(NODE_INITIALIZATION_OBO_HELP);
        });
        ui.collapsing("Forces", |ui| {
            ui.add(
                egui::Slider::new(&mut state.single_loop_force, 0.0..=1.0)
                    .text("Single loop force"),
            )
            .on_hover_text(FORCES_SLF_HELP);
        });
        ui.collapsing("Parts", |mut ui| {
            parts_ui(&mut ui, &mut state, &mut current_plushie);
        });

        // TEMP
        ui.add(egui::Slider::new(&mut state.centroids, 0..=20).text("Centroids"))
            .on_hover_text("Number of stuffing centroids. Bigger plushies need more centroids");
    });

    // prevent world events on resizing
    if !*collapsed && using_resizer(ctx, panel.extended_panel_id, Side::Right) {
        ui_used_input.set_true();
    }

    Ok(())
}

fn display_mode(ui: &mut Ui, state: &mut SimulationState) {
    ui.radio_value(&mut state.display_mode, DisplayMode::Pattern, "Pattern")
        .on_hover_text("Use colors defined in the pattern. Big stitches, small links.");
    // ui.radio_value(&mut state.display_mode, DisplayMode::Stitches, "Stitches"); // TODO differentiate stitch kind (sc vs inc etc)
    ui.radio_value(&mut state.display_mode, DisplayMode::Forces, "Link forces")
        .on_hover_text("Show the forces applied by links. Big links, tiny stitches");
    // TODO forces applied by centroids
    //  - present them like links? - will be barely readable. Actually this could work if actual links and stitches become hidden and s
    //  - color the stitches with a shader?
}

// long strings break rust analyzer, can't even format a file. It works if the long string is here
// cargo clean didn't help
const NODE_INITIALIZATION_OBO_HELP: &'static str = "Spawn the stitches one by one, waiting for the previous node to reach a relatively stable position before advancing.";
const FORCES_SLF_HELP: &'static str = "Controls how much the \"Front loop only\" and \"Back loop only \" nodes are pushed in/out of the creation. Can cause the plushie to rotate endlessly.";
