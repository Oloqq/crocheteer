use bevy_egui::{egui::panel::Side, prelude::*};

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
