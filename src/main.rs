mod assets;
mod game_master;
mod settings;
mod stone;

use assets::{setup_assets, OthelloAssets};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};
use game_master::{board_sync_system, GameMaster};
use settings::Settings;
use stone::{Position, Stone, StoneBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Settings::default())
        .insert_resource(GameMaster::default())
        .add_event::<StonePlaced>()
        .add_systems(PreStartup, setup_assets)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, board_sync_system)
        .add_systems(Update, cursor_system)
        .run();
}

fn setup(
    mut commands: Commands,
    settings: Res<Settings>,
    assets: Res<OthelloAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Spawn board
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: settings.board_bg_color,
            custom_size: Some(settings.board_size),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    // Spawn initial stones
    commands.spawn(StoneBundle::new(
        Stone::Black,
        Position { x: 3, y: 3 },
        &settings,
        &assets,
    ));
    commands.spawn(StoneBundle::new(
        Stone::Black,
        Position { x: 4, y: 4 },
        &settings,
        &assets,
    ));
    commands.spawn(StoneBundle::new(
        Stone::White,
        Position { x: 3, y: 4 },
        &settings,
        &assets,
    ));
    commands.spawn(StoneBundle::new(
        Stone::White,
        Position { x: 4, y: 3 },
        &settings,
        &assets,
    ));
}

fn cursor_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    settings: Res<Settings>,
    mut game_master: ResMut<GameMaster>,
    mut ev_stone_placed: EventWriter<StonePlaced>,
) {
    if game_master.stone_placed {
        return;
    }

    let (camera, camera_transofrm) = q_camera.single();
    let window = q_window.single();

    let cursor_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transofrm, cursor))
        .map(|ray| ray.origin.truncate());

    for event in mouse_button_input_events.read() {
        if event.state != ButtonState::Released {
            continue;
        }
        if let Some(cursor_pos) = cursor_pos {
            match event.button {
                MouseButton::Left => {
                    if let Some(board_pos) = settings.world_pos_2_board_pos(cursor_pos) {
                        println!("Left click at {:?}", board_pos);
                        ev_stone_placed.send(StonePlaced {
                            stone: game_master.turn,
                            position: Position {
                                x: board_pos.x,
                                y: board_pos.y,
                            },
                        });
                        game_master.stone_placed = true;
                    }
                }
                MouseButton::Right => {
                    if let Some(board_pos) = settings.world_pos_2_board_pos(cursor_pos) {
                        println!("Right click at {:?}", board_pos);
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Event)]
struct StonePlaced {
    stone: Stone,
    position: Position,
}

#[derive(Component)]
struct MainCamera;
