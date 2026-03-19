use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, KeyboardShortcut, Modifiers},
};

use crate::ui::data::CodeEditorState;

pub fn top_panel(mut contexts: EguiContexts, state: Res<CodeEditorState>) -> Result {
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

            // ui.allocate_ui(egui::Vec2::ZERO, |ui| {
            //     let dummy_text = &mut String::new();
            //     let _ = ui.add_visible(
            //         false,
            //         egui::TextEdit::singleline(dummy_text).id(state.dummy),
            //     );
            // })
            ui.scope_builder(
                egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(
                    egui::pos2(-1000.0, -1000.0),
                    egui::Vec2::ZERO,
                )),
                |ui| {
                    let dummy_text = &mut String::new();
                    let _ = ui.add(egui::TextEdit::singleline(dummy_text).id(state.dummy));
                },
            );
        });
    });

    // this is stupid, let's just fire messages
    let ctrls = ctx
        .input_mut(|i| i.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, egui::Key::S)));

    if ctrls {
        info!("save");
        ctx.memory_mut(|m| m.request_focus(state.dummy));
    } else {
        ctx.memory_mut(|m| {
            if m.has_focus(state.dummy) {
                m.surrender_focus(state.dummy);
            }
        })
    }

    Ok(())
}

fn shortcut_button(ui: &mut egui::Ui, label: &str, shortcut: &str) -> egui::Response {
    ui.add(egui::Button::new(egui::RichText::new(label)).shortcut_text(shortcut))
}
