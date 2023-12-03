use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_flycam::prelude::*;
use rand::Rng;

use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, Perlin};

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(KeyBindings {
            move_forward: KeyCode::Comma,
            move_backward: KeyCode::O,
            move_left: KeyCode::A,
            move_right: KeyCode::E,
            ..Default::default()
        })
        // .add_systems(Startup, setup_cameras) //TODO setup game camera
        .add_systems(Startup, (setup_game, spawn_player))
        .add_systems(Update, (move_player, respanw_board))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
const BOARD_SIZE_COL: usize = 48;
const BOARD_SIZE_ROW: usize = 48;

// fn setup_cameras(mut commands: Commands) {
//     // /zero.y + 3
//     let mid_x = BOARD_SIZE_COL as f32 / 2.0;
//     let mid_z = BOARD_SIZE_ROW as f32 / 2.0;
//     let look_at = Vec3::new(mid_x, 0.0, mid_z);

//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(mid_x, mid_x, mid_z * 3.0).looking_at(look_at, Vec3::Y),
//         ..default()
//     });
// }

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
struct Game {}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // reset the game state
    let mid = (BOARD_SIZE_COL + BOARD_SIZE_ROW) as f32 / 3.0;

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(mid, mid, mid),
        point_light: PointLight {
            intensity: 80000.0,
            shadows_enabled: true,
            range: mid * 10.0,
            ..default()
        },
        ..default()
    });

    generate_terrain(commands, asset_server);
}

#[derive(Component)]
struct Terrain;

fn generate_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let time_start = std::time::Instant::now();
    println!("Generating terrain");
    // let should_spawn_tree = rand::thread_rng().gen_bool(0.05);
    let fbm = Fbm::<Perlin>::new(rand::random::<u32>());
    let bound = 2.0;
    let noise = PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(BOARD_SIZE_COL, BOARD_SIZE_ROW)
        .set_x_bounds(-bound, bound)
        .set_y_bounds(-bound, bound)
        .build();
    println!("Noise generated in {:?}", time_start.elapsed());
    // spawn the game board
    let cell_scene = asset_server.load("models/tile.glb#Scene0");
    let tree_scene = asset_server.load("models/pine_snow.glb#Scene0");

    for col in 0..BOARD_SIZE_COL {
        for row in 0..BOARD_SIZE_ROW {
            let should_spawn_tree =
                noise.get_value(col, row) > 0.2 && rand::thread_rng().gen_bool(0.7);
            if should_spawn_tree {
                commands.spawn((
                    SceneBundle {
                        transform: Transform {
                            //spawn the trees
                            translation: Vec3::new(col as f32, 0.0, row as f32),
                            rotation: Quat::from_rotation_y(rand::thread_rng().gen_range(-PI..PI)),
                            ..default()
                        },
                        scene: tree_scene.clone(),
                        ..default()
                    },
                    Terrain,
                ));
            }
            commands.spawn((
                SceneBundle {
                    //spawn the tiles
                    transform: Transform::from_xyz(col as f32, 0.0, row as f32),
                    scene: cell_scene.clone(),
                    ..default()
                },
                Terrain,
            ));
        }
    }
    println!("Terrain generated in {:?}", time_start.elapsed());
}

fn respanw_board(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    terrain_query: Query<Entity, With<Terrain>>,
) {
    if keyboard_input.pressed(KeyCode::G) {
        for entity in &terrain_query {
            commands.entity(entity).despawn_recursive()
        }
        generate_terrain(commands, asset_server);
    }
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
