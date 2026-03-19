use bevy::{picking::PickingSystems, prelude::*, window::PrimaryWindow};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CursorRayPlugin;

#[derive(Resource)]
pub struct CursorRay(pub Ray3d);

impl Plugin for CursorRayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorRay(Ray3d::new(Vec3::ZERO, Dir3::NEG_Z)))
            .add_systems(PreUpdate, update_cursor_ray.before(PickingSystems::Hover));
    }
}

pub fn update_cursor_ray(
    mut cursor_ray: ResMut<CursorRay>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (main_camera, main_camera_transform) = *camera;
    // Get the cursor position in the world
    if let Some(cursor_in_window) = primary_window.cursor_position() {
        if let Ok(world_pos) =
            main_camera.viewport_to_world(main_camera_transform, cursor_in_window)
        {
            cursor_ray.0 = world_pos;
        }
    }
}
