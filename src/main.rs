use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, (setup_cameras, setup, spawn_player))
        .add_systems(Update, move_player)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

struct Cell {}

#[derive(Component)]
struct Xp(u32);

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Xp(0),
        SceneBundle {
            transform: Transform {
                translation: Vec3::new(2.0, 0.0, 2.0),
                rotation: Quat::from_rotation_y(-PI),
                ..default()
            },
            scene: asset_server.load("models/alien.glb#Scene0"),
            ..default()
        },
    ));
}

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Cell>>,
}

const BOARD_SIZE_I: usize = 15;
const BOARD_SIZE_J: usize = 10;

fn setup_cameras(mut commands: Commands) {
    // /zero.y + 3
    let look_at = Vec3::ZERO + Vec3::X * 7.0;

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(7.0, 5.0, 20.0).looking_at(look_at, Vec3::Y),
        ..default()
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    // reset the game state

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            range: 30.0,
            ..default()
        },
        ..default()
    });

    // spawn the game board
    let cell_scene = asset_server.load("models/tile.glb#Scene0");
    let tree_scene = asset_server.load("models/tree.glb#Scene0");
    game.board = (0..BOARD_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I)
                .map(|i| {
                    let should_spawn_tree = rand::thread_rng().gen_bool(0.2);
                    if should_spawn_tree {
                        commands.spawn(SceneBundle {
                            transform: Transform {
                                //spawn the trees
                                translation: Vec3::new(i as f32, 0.0, j as f32),
                                rotation: Quat::from_rotation_x(PI / 2.0),
                                ..default()
                            },
                            scene: tree_scene.clone(),
                            ..default()
                        });
                    }
                    commands.spawn(SceneBundle {
                        //spawn the tiles
                        transform: Transform::from_xyz(i as f32, 0.0, j as f32),
                        scene: cell_scene.clone(),
                        ..default()
                    });
                    Cell {}
                })
                .collect()
        })
        .collect();
}

pub const PLAYER_SPEED: f32 = 10.0;

// control the game character
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let delta = time.delta_seconds();
        let mut direction = Vec3::ZERO;
        let mut rotation = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::NEG_Z;
            rotation = 0.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::Z;
            rotation = -PI;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::X;
            rotation = -PI / 2.;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::NEG_X;
            rotation = PI / 2.;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            transform.rotation = Quat::from_rotation_y(rotation);
            transform.translation += direction * PLAYER_SPEED * delta;
        }
    }
}
