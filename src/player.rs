use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

mod input;
mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<input::PlayerAction>::default(),
            movement::CharacterControllerPlugin,
        ))
        .add_systems(Startup, spawn_player)
        .add_systems(Update, input::handle_actions);
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture = server.load("textures/smile.png");

    commands.spawn((
        Player,
        movement::CharacterControllerBundle::new(),
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(3.)),
            texture,
            ..Default::default()
        },
        InputManagerBundle::with_map(input::PlayerAction::default_input_map()),
    ));

    let mut make_rect = move |pos: Vec2, size: Vec2| {
        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(size.x, size.y),
            PbrBundle {
                mesh: meshes.add(Cuboid::default()),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            },
        ));
    };

    make_rect(Vec2::new(100., 100.), Vec2::new(100., 100.));
    make_rect(Vec2::new(200., 100.), Vec2::new(100., 100.));
}
