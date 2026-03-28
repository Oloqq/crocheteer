mod camera;
mod cursor_ray;
mod plushie;
pub mod project;
mod ui;

use std::{io::Cursor, time::Duration};

use bevy::{
    ecs::{
        schedule::{LogLevel, ScheduleBuildSettings},
        system::NonSendMarker,
    },
    prelude::*,
    render::RenderPlugin,
    winit::{UpdateMode, WINIT_WINDOWS},
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings};

use crate::{cursor_ray::CursorRayPlugin, project::Project};

// a yarn I work with 5mm hook generally yields 5mm big stitches
// TODO define in pattern? define in project settings? do we support variable hook size? if we support, is it variable per part or can it change in arbitrary positions?
const HOOK_SIZE: f32 = 5e-4;
const FIXED_UPDATE_BASE_HZ: f64 = 64.0;

pub fn app(project: Project) -> App {
    let mut app = App::new();
    unambiguous_schedules(&mut app);
    window(&mut app);
    visible_3d_world(&mut app);
    app.add_plugins(ui::UiPlugin {
        initial_pattern: project.pattern.clone(), // TODO initialize through project module
    });
    app.add_plugins(plushie::PlushiePlugin);
    crate::project::startup::apply_settings(&mut app, &project.simulation_config);
    app.insert_resource(project.simulation_config);
    app
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
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Crocheteer".into(),
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                synchronous_pipeline_compilation: true, // compile shaders before application starts
                ..default()
            }),
    );
    app.insert_resource(bevy::winit::WinitSettings {
        focused_mode: UpdateMode::reactive(Duration::from_secs_f64(1.0 / 144.0)),
        unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f64(1.0 / 20.0)),
    });
    app.add_plugins(bevy_framepace::FramepacePlugin);
    app.add_systems(
        Startup,
        |mut settings: ResMut<bevy_framepace::FramepaceSettings>| {
            settings.limiter = bevy_framepace::Limiter::from_framerate(144.0);
        },
    );
    app.add_systems(Startup, set_window_icon);
}

// _: NonSendMarker is required, otherwise winit_windows.windows.len() is 0 :))))))
// WINIT_WINDOWS is "thread_local!(...)", which means each thread gets a copy of the constant
// but it is only populated in main thread
// NonSendMarker forces the system to run on main thread
fn set_window_icon(_: NonSendMarker) {
    WINIT_WINDOWS.with(|winit_windows| {
        let winit_windows = winit_windows.borrow();
        let (icon_rgba, icon_width, icon_height) = {
            let bytes = include_bytes!("../assets/images/icon.png"); // TODO make a real icon
            let img = ::image::ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()
                .expect("Failed to guess image format")
                .decode()
                .expect("Failed to decode icon")
                .into_rgba8();
            let (width, height) = img.dimensions();
            (img.into_raw(), width, height)
        };

        let icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

        for window in winit_windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
    });
}

fn visible_3d_world(app: &mut App) {
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(CursorRayPlugin);
    app.add_plugins(InfiniteGridPlugin);
    app.add_systems(PreStartup, |mut commands: Commands| {
        commands.spawn(InfiniteGridBundle {
            settings: InfiniteGridSettings {
                fadeout_distance: 5.0,
                scale: 100.0, // 1 cell = 0.01 world units = 10 cm
                ..default()
            },
            ..default()
        });
    });
}
