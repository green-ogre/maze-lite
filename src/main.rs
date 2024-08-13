use bevy::prelude::*;
use maze::MazePlugin;

pub mod maze;

fn main() {
    App::default()
        .add_plugins((DefaultPlugins, MazePlugin))
        .run();
}
