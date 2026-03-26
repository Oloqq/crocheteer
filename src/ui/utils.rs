use bevy_egui::{
    egui::{Rect, Ui, UiBuilder, panel::Side},
    prelude::*,
};

pub fn full_height_button(
    ui: &mut egui::Ui,
    id: egui::Id,
    rect: egui::Rect,
    label: &str,
) -> egui::Response {
    let response = ui.interact(rect, id, egui::Sense::click());

    let fill = if response.is_pointer_button_down_on() {
        ui.visuals().widgets.active.bg_fill
    } else if response.hovered() {
        ui.visuals().widgets.hovered.bg_fill
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, 0.0, fill);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(14.0),
        ui.visuals().text_color(),
    );

    return response;
}

pub fn using_resizer(ctx: &egui::Context, id: egui::Id, side: Side) -> bool {
    let grab_radius =
        ctx.style().interaction.resize_grab_radius_side + ctx.style().interaction.interact_radius;
    ctx.input(|i| i.pointer.hover_pos())
        .and_then(|pointer_pos| {
            ctx.memory(|mem| {
                mem.data
                    .get_temp::<egui::containers::panel::PanelState>(id)
                    .map(|panel| {
                        match side {
                            Side::Left => egui::Rect::from_min_max(
                                panel.rect.min,
                                panel.rect.max + egui::vec2(grab_radius, 0.0),
                            ),
                            Side::Right => egui::Rect::from_min_max(
                                panel.rect.min - egui::vec2(grab_radius, 0.0),
                                panel.rect.max,
                            ),
                        }
                        .contains(pointer_pos)
                    })
            })
        })
        .unwrap_or(false)
}

pub fn using_resizer_bottom(ctx: &egui::Context, id: egui::Id) -> bool {
    let grab_radius =
        ctx.style().interaction.resize_grab_radius_side + ctx.style().interaction.interact_radius;
    ctx.input(|i| i.pointer.hover_pos())
        .and_then(|pointer_pos| {
            ctx.memory(|mem| {
                mem.data
                    .get_temp::<egui::containers::panel::PanelState>(id)
                    .map(|panel| {
                        egui::Rect::from_min_max(
                            panel.rect.min - egui::vec2(0.0, grab_radius),
                            panel.rect.max,
                        )
                        .contains(pointer_pos)
                    })
            })
        })
        .unwrap_or(false)
}

/// Allocates horizontal space needed for a slider and a few characters
pub fn require_width_for_slider(ui: &mut Ui) {
    let rect = ui.cursor();
    let mut child_ui = ui.new_child(UiBuilder::new().max_rect(rect));
    child_ui.set_clip_rect(Rect::NOTHING);
    child_ui.add(egui::Slider::new(&mut 0.0, 1.0..=2.0).text("  ")); // text to make space for a few characters as well
    ui.allocate_space(egui::Vec2::new(child_ui.min_size().x, 0.0));
}

/// Using this in a SidePanel, allows the SidePanel to be resized in a way that hides part of the content
pub struct CanGoOffscreen {}

impl CanGoOffscreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show<R>(&self, parent: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) {
        let mut child_ui = parent.new_child(UiBuilder::new());
        add_contents(&mut child_ui);
        parent.allocate_space(egui::Vec2::new(0.0, child_ui.min_size().y));
    }
}
