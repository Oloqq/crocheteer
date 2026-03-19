mod camera;
mod cursor_ray;
mod ui;

use std::time::Duration;

use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    winit::UpdateMode,
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings};

use crate::{cursor_ray::CursorRayPlugin, ui::world_input};

pub fn app() -> App {
    let mut app = App::new();
    unambiguous_schedules(&mut app);
    window(&mut app);
    visible_3d_world(&mut app);
    app.add_plugins(ui::UiPlugin);
    app.add_systems(Update, say_click.run_if(world_input));
    app
}

fn say_click(mouse: Res<ButtonInput<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        info!("click");
    }
}

fn unambiguous_schedules(app: &mut App) {
    app.edit_schedule(Startup, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Error,
            ..default()
        });
    })
    .edit_schedule(Update, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Error,
            ..default()
        });
    })
    .edit_schedule(FixedUpdate, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Error,
            ..default()
        });
    });
}

fn window(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Crocheteer".into(),
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(bevy::winit::WinitSettings {
        focused_mode: UpdateMode::reactive(Duration::from_secs_f64(1.0 / 144.0)),
        unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f64(1.0 / 20.0)),
    })
    .add_plugins(bevy_framepace::FramepacePlugin)
    .add_systems(
        Startup,
        |mut settings: ResMut<bevy_framepace::FramepaceSettings>| {
            settings.limiter = bevy_framepace::Limiter::from_framerate(144.0);
        },
    );
}

fn visible_3d_world(app: &mut App) {
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(CursorRayPlugin);
    app.add_plugins(InfiniteGridPlugin);
    app.add_systems(PreStartup, |mut commands: Commands| {
        commands.spawn(InfiniteGridBundle {
            settings: InfiniteGridSettings {
                fadeout_distance: 500.0,
                ..default()
            },
            ..default()
        });
    });
}
