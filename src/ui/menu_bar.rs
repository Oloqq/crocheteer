use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};

use crate::{
    project::{self, OpenProject},
    ui::{data::UiState, shortcuts::ShortcutAction, ui_used_input::UiUsedInput},
};

pub fn top_panel(
    mut contexts: EguiContexts,
    ui_used_input: Res<UiUsedInput>,
    mut console_state: ResMut<UiState>,
    mut commands: Commands,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Project").clicked() {
                    commands.trigger(OpenProject {
                        project: Default::default(),
                        filename: None,
                    });
                    ui.close();
                }
                ui.menu_button("Open example", |ui| {
                    if ui.button("Mushroom (FLO+BLO)").clicked() {
                        commands.trigger(OpenProject {
                            project: project::examples::public::mushroom(),
                            filename: None,
                        });
                        ui.close();
                    }
                });
                ShortcutAction::Open.button(ui, &mut commands);
                ui.separator();
                ShortcutAction::Quicksave.button(ui, &mut commands);
                ShortcutAction::SaveAs.button(ui, &mut commands);
                ui.separator();
                if ui.button("Exit").clicked() {
                    commands.write_message(AppExit::Success);
                    ui.close();
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    ui.close();
                }
                if ui.button("Redo").clicked() {
                    ui.close();
                }
                ui.separator();
                if ui.button("Cut").clicked() {
                    ui.close();
                }
                if ui.button("Copy").clicked() {
                    ui.close();
                }
                if ui.button("Paste").clicked() {
                    ui.close();
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    ui.close();
                }
            });

            if ui.button("Console").clicked() {
                console_state.console_visible = !console_state.console_visible;
            }
            if ui.button("Charts").clicked() {
                console_state.charts_visible = !console_state.charts_visible;
            }
        });
    });

    if ShortcutAction::consume_keybinds(ctx, &mut commands) {
        ui_used_input.set_true();
    }

    Ok(())
}
