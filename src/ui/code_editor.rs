use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, PopupAnchor},
};
use egui_code_editor::CodeEditor;

use crate::ui::{data::CodeEditorState, input_capture::InputCaptured};

pub fn code_editor_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<CodeEditorState>,
    captured: Res<InputCaptured>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    ctx.style_mut(|style| {
        style.animation_time = 0.05; // default is 0.1 seconds
        style.interaction.interact_radius = 0.0;
    });

    egui::SidePanel::show_animated_between(
        ctx,
        state.expanded,
        egui::SidePanel::left("left_side_panel_collapsed")
            .exact_width(24.0)
            .resizable(false),
        egui::SidePanel::left("left_side_panel_extended").resizable(true),
        |ui, expansion| {
            if expansion > 0.5 {
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    // Toolbar
                    ui.horizontal(|ui| {
                        ui.heading("Pattern");
                        if ui.button("◀").clicked() {
                            state.expanded = false;
                        }
                        if ui.button("💾 Save").clicked() { /* ... */ }
                        if ui.button("💾 Load").clicked() { /* ... */ }
                        if ui.button("View").clicked() { /* ... */ }
                    });
                    ui.separator();

                    let scroll = egui::ScrollArea::vertical()
                        .id_salt("code_editor_scroll")
                        .auto_shrink(false) // fills available space
                        .stick_to_bottom(true); // <-- this is what you want

                    // Editor fills remaining space
                    scroll.show(ui, |ui| {
                        let response = CodeEditor::default()
                            .id_source("code_editor")
                            .with_rows(20)
                            .with_fontsize(14.0)
                            .with_theme(state.theme)
                            .with_syntax(state.syntax.clone())
                            .with_numlines(true)
                            .vscroll(false)
                            .show(ui, &mut state.code);
                        // .show_with_completer(ui, &mut state.code, Completer stored in App)

                        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let text_rect = response.text_clip_rect;
                            if text_rect.contains(mouse_pos) {
                                let relative_pos = mouse_pos - text_rect.min;

                                let cursor = response.galley.cursor_from_pos(relative_pos);
                                println!("curra {:?}", cursor);
                                if cursor.index == 0 {
                                    egui::Tooltip::always_open(
                                        ctx.clone(),
                                        ui.layer_id(),
                                        egui::Id::new("token_tooltip"),
                                        PopupAnchor::Pointer,
                                    )
                                    .show(|ui| {
                                        ui.label("bruh");
                                    });
                                }
                            }
                        }
                    });
                });
            } else {
                let response = full_height_button(ui, ui.clip_rect(), "▶");
                if response.clicked() {
                    state.expanded = true;
                }
            }
        },
    );

    // hacky hack to ensure grabbing the resize bar is registered as an input "wanted by egui"
    let panel_id = egui::Id::new("left_side_panel_extended");
    let grab_radius =
        ctx.style().interaction.resize_grab_radius_side + ctx.style().interaction.interact_radius;

    if state.expanded
        && ctx
            .input(|i| i.pointer.hover_pos())
            .and_then(|pointer_pos| {
                ctx.memory(|mem| {
                    mem.data
                        .get_temp::<egui::containers::panel::PanelState>(panel_id)
                        .map(|panel| {
                            egui::Rect::from_min_max(
                                panel.rect.min - egui::vec2(grab_radius, 0.0),
                                panel.rect.max,
                            )
                            .contains(pointer_pos)
                        })
                })
            })
            .unwrap_or(false)
    {
        captured.capture();
    }

    Ok(())
}

fn full_height_button(ui: &mut egui::Ui, rect: egui::Rect, label: &str) -> egui::Response {
    let response = ui.interact(
        rect,
        ui.id().with("left_collapse_toggle"),
        egui::Sense::click(),
    );

    let fill = if response.is_pointer_button_down_on() {
        ui.visuals().widgets.active.bg_fill
    } else if response.hovered() {
        ui.visuals().widgets.hovered.bg_fill
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, 0.0, fill);

    // Draw your label/arrow however you want (no hitbox for events)
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(14.0),
        ui.visuals().text_color(),
    );

    return response;
}
