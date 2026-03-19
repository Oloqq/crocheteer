use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, PopupAnchor, panel::Side},
};
use egui_code_editor::CodeEditor;

use crate::ui::{
    data::CodeEditorState,
    input_capture::InputCaptured,
    utils::{full_height_button, using_resizer},
};

pub fn code_editor_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<CodeEditorState>,
    captured: Res<InputCaptured>,
    mut collapsed: Local<bool>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let extended_panel_id = egui::Id::new("left_side_panel_extended");

    egui::SidePanel::show_animated_between(
        ctx,
        *collapsed,
        egui::SidePanel::left(extended_panel_id).resizable(true),
        egui::SidePanel::left("left_side_panel_collapsed")
            .exact_width(24.0)
            .resizable(false),
        |ui, shrinkation| {
            if shrinkation < 0.5 {
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    // Toolbar
                    ui.horizontal(|ui| {
                        ui.heading("Pattern");
                        if ui.button("◀").clicked() {
                            *collapsed = true;
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
                let response = full_height_button(
                    ui,
                    ui.id().with("collapse_toggle_left"),
                    ui.clip_rect(),
                    "▶",
                );
                if response.clicked() {
                    *collapsed = false;
                }
            }
        },
    );

    // hacky hack to ensure grabbing the resize bar is registered as an input "wanted by egui"
    if !*collapsed && using_resizer(ctx, extended_panel_id, Side::Left) {
        captured.capture();
    }

    Ok(())
}
