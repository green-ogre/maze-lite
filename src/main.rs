use avian2d::PhysicsPlugins;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};
use maze::MazePlugin;
use player::PlayerPlugin;

pub mod animated_sprites;
pub mod maze;
pub mod player;

fn main() {
    let title = "Basic Example - Press Space to change Texture and H to show/hide tilemap.";
    App::default()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from(title),
                        resolution: WindowResolution::new(1200.0, 1200.0),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                    ..default()
                }),
        )
        .add_plugins((
            MazePlugin,
            // for crisp sprites if we want that
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MazePlugin,
            PhysicsPlugins::default(),
            PlayerPlugin,
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .insert_resource(avian2d::prelude::Gravity::ZERO)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
