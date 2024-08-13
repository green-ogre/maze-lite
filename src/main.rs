use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};
use maze::MazePlugin;

pub mod maze;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Basic Example - Press Space to change Texture and H to show/hide tilemap.",
                ),
                resolution: WindowResolution::new(1200.0, 1200.0),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest())
            .set(LogPlugin {
    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    ..default()
})
        )
            .add_plugins(MazePlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
