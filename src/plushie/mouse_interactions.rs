use bevy::prelude::*;

use crate::{
    cursor_ray::CursorRay,
    plushie::{
        animation::Rooted,
        data::{Dragging, PressHandled, Selected},
    },
    ui::UiUsedInput,
};

pub fn adding_to_selection(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
}

pub fn on_click(
    trigger: On<Pointer<Press>>,
    mut commands: Commands,
    mut press_handled: ResMut<PressHandled>,
    transforms: Query<&GlobalTransform>, // Transform is local to parent Entity, GlobalTransform is always world coordinates
    selected: Query<Entity, With<Selected>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    cursor_ray: Res<CursorRay>,
    camera: Single<&GlobalTransform, With<Camera3d>>,
    ui_used_input: Res<UiUsedInput>,
) {
    if trigger.button != PointerButton::Primary {
        return;
    }
    if ui_used_input.get() {
        return;
    }

    press_handled.0 = true;

    let ray = cursor_ray.0;
    let plane_normal = camera.forward();
    let plane = InfinitePlane3d::new(plane_normal);

    let init_dragging = |ball_world_pos: Vec3| -> Option<Dragging> {
        // fails if ray is parallel to the plane
        let dist = ray.intersect_plane(ball_world_pos, plane)?;
        let cursor_on_plane = ray.get_point(dist);
        Some(Dragging {
            offset: ball_world_pos - cursor_on_plane,
            plane,
            plane_origin: ball_world_pos,
        })
    };

    commands.entity(trigger.entity).insert(Selected);
    let ball_world_pos = transforms
        .get(trigger.entity)
        .expect("clicked ball should have a transform")
        .translation();
    if let Some(dragging) = init_dragging(ball_world_pos) {
        commands.entity(trigger.entity).insert(dragging);
        commands.entity(trigger.entity).insert(Rooted);
    }

    if adding_to_selection(&keyboard) {
        for entity in selected {
            let Ok(ball_world_pos) = transforms.get(entity) else {
                continue;
            };
            if let Some(dragging) = init_dragging(ball_world_pos.translation()) {
                commands.entity(entity).insert(dragging);
            }
        }
    } else {
        for entity in &selected {
            if entity == trigger.entity {
                continue;
            }
            commands.entity(entity).remove::<Selected>();
        }
    }
}

pub fn update_dragging(dragging: Query<(&mut Transform, &Dragging)>, cursor: Res<CursorRay>) {
    let ray = cursor.0;
    for (mut tf, drag) in dragging {
        let Some(dist) = ray.intersect_plane(drag.plane_origin, drag.plane) else {
            continue;
        };
        let cursor_on_plane = ray.get_point(dist);
        tf.translation = cursor_on_plane + drag.offset;
    }
}

pub fn stop_dragging(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    dragging: Query<Entity, With<Dragging>>,
) {
    if mouse.just_released(MouseButton::Left) {
        for entity in dragging {
            commands.entity(entity).remove::<Dragging>();
            commands.entity(entity).remove::<Rooted>(); // assumption: only dragging system applies rooted. TODO what to do with multiple systems?
        }
    }
}

pub fn deselect_on_empty_press(
    mut commands: Commands,
    mut press_handled: ResMut<PressHandled>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    selected: Query<Entity, With<Selected>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if !press_handled.0 && !adding_to_selection(&keyboard) {
            for entity in &selected {
                commands.entity(entity).remove::<Selected>();
            }
        }
        press_handled.0 = false; // reset every press
    }
}
