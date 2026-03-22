use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, KeyboardShortcut, Modifiers},
};

use crate::ui::{data::ConsoleState, ui_used_input::UiUsedInput};

pub fn top_panel(
    mut contexts: EguiContexts,
    ui_used_input: Res<UiUsedInput>,
    mut console_state: ResMut<ConsoleState>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if shortcut_button(ui, "New", "Ctrl+N").clicked() {
                    ui.close();
                }
                if shortcut_button(ui, "Open", "Ctrl+O").clicked() {
                    ui.close();
                }
                ui.separator();
                if shortcut_button(ui, "Save", "Ctrl+S").clicked() {
                    ui.close();
                }
                ui.menu_button("Nested", |ui| {
                    if ui.button("Stuff").clicked() {
                        ui.close();
                    }
                });
                ui.separator();
                if ui.button("Exit").clicked() {
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
                console_state.visible = !console_state.visible;
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
