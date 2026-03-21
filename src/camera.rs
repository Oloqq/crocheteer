use bevy::prelude::*;
use bevy_editor_cam::{
    DefaultEditorCamPlugins,
    extensions::{dolly_zoom::DollyZoomTrigger, look_to::LookToTrigger},
    prelude::{EditorCam, projections},
};
use bevy_egui::EguiStartupSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MeshPickingPlugin, DefaultEditorCamPlugins));
        app.add_systems(PreStartup, setup_view.before(EguiStartupSet::InitContexts));
        app.add_systems(PostUpdate, (toggle_projection, switch_direction));
    }
}

fn setup_view(mut commands: Commands) {
    let cam_trans = Transform::from_xyz(0.0, 0.1, 0.1).looking_at(Vec3::ZERO, Vec3::Y);
    commands
        .spawn((
            Camera3d::default(),
            Camera::default(),
            EditorCam {
                orbit_constraint: default(),
                last_anchor_depth: -cam_trans.translation.length() as f64,
                orthographic: projections::OrthographicSettings {
                    scale_to_near_clip: 1_000_f32, // Needed for SSAO to work in ortho // allegedly (it's copied from an example)
                    ..Default::default()
                },
                ..Default::default()
            },
            cam_trans,
        ))
        .with_children(|parent| {
            parent.spawn((
                DirectionalLight {
                    illuminance: 5000.0,
                    ..default()
                },
                Transform::default(), // rotation relative to camera, identity = same direction as camera
            ));
        });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        affects_lightmapped_meshes: true,
    });
}

fn toggle_projection(
    keys: Res<ButtonInput<KeyCode>>,
    mut dolly: MessageWriter<DollyZoomTrigger>,
    cam: Query<Entity, With<EditorCam>>,
    mut toggled: Local<bool>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        *toggled = !*toggled;
        let target_projection = if *toggled {
            Projection::Orthographic(OrthographicProjection::default_3d())
        } else {
            Projection::Perspective(PerspectiveProjection::default())
        };
        dolly.write(DollyZoomTrigger {
            target_projection,
            camera: cam.single().unwrap(),
        });
    }
}

fn switch_direction(
    keys: Res<ButtonInput<KeyCode>>,
    mut look_to: MessageWriter<LookToTrigger>,
    cam: Query<(Entity, &Transform, &EditorCam)>,
) {
    let (camera, transform, editor) = cam.single().unwrap();
    if keys.just_pressed(KeyCode::Digit1) {
        look_to.write(LookToTrigger::auto_snap_up_direction(
            Dir3::X,
            camera,
            transform,
            editor,
        ));
    }
    if keys.just_pressed(KeyCode::Digit2) {
        look_to.write(LookToTrigger::auto_snap_up_direction(
            Dir3::Z,
            camera,
            transform,
            editor,
        ));
    }
    if keys.just_pressed(KeyCode::Digit3) {
        look_to.write(LookToTrigger::auto_snap_up_direction(
            Dir3::NEG_X,
            camera,
            transform,
            editor,
        ));
    }
    if keys.just_pressed(KeyCode::Digit4) {
        look_to.write(LookToTrigger::auto_snap_up_direction(
            Dir3::NEG_Z,
            camera,
            transform,
            editor,
        ));
    }
    if keys.just_pressed(KeyCode::Digit5) {
        look_to.write(LookToTrigger::auto_snap_up_direction(
            Dir3::Y,
            camera,
            transform,
            editor,
        ));
    }
    if keys.just_pressed(KeyCode::Digit6) {
        look_to.write(LookToTrigger::auto_snap_up_direction(
            Dir3::NEG_Y,
            camera,
            transform,
            editor,
        ));
    }
}
