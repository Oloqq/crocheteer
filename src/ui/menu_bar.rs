use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, KeyboardShortcut, Modifiers},
};

use crate::{
    project::{
        self, OpenProject, SaveProject,
        file_dialog::{FileDialogPurpose, OpenFileDialog},
    },
    ui::{data::UiState, ui_used_input::UiUsedInput},
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
                    });
                    ui.close();
                }
                ui.menu_button("Open example", |ui| {
                    if ui.button("Mushroom (FLO+BLO)").clicked() {
                        commands.trigger(OpenProject {
                            project: project::examples::public::mushroom(),
                        });
                        ui.close();
                    }
                });
                if shortcut_button(ui, "Open", "Ctrl+O").clicked() {
                    ui.close();
                    commands.trigger(OpenFileDialog {
                        purpose: FileDialogPurpose::LoadProject,
                    });
                }
                ui.separator();
                if shortcut_button(ui, "Save", "Ctrl+S").clicked() {
                    ui.close();
                    commands.trigger(SaveProject);
                }
                if ui.button("Save As").clicked() {
                    // commands.trigger(OpenFileSaveDialog);
                    ui.close();
                }

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

    let ctrls = ctx
        .input_mut(|i| i.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, egui::Key::S)));

    if ctrls {
        ui_used_input.set_true();
    }

    Ok(())
}

fn shortcut_button(ui: &mut egui::Ui, label: &str, shortcut: &str) -> egui::Response {
    ui.add(egui::Button::new(egui::RichText::new(label)).shortcut_text(shortcut))
}
