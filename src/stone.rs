use bevy::prelude::*;

use crate::{assets::OthelloAssets, game_master::GameMaster, settings::Settings, StonePlaced};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stone {
    Black,
    White,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Bundle)]
pub struct StoneBundle {
    pub stone: Stone,
    pub position: Position,
    pub sprite: ColorMesh2dBundle,
}

impl StoneBundle {
    pub fn new(
        stone: Stone,
        position: Position,
        settings: &Res<Settings>,
        assets: &Res<OthelloAssets>,
    ) -> Self {
        let color = match stone {
            Stone::Black => assets.stone_black.clone(),
            Stone::White => assets.stone_white.clone(),
        };
        Self {
            stone,
            position,
            sprite: ColorMesh2dBundle {
                mesh: assets.stone_shape.clone().into(),
                material: color,
                transform: Transform::from_xyz(
                    position.x as f32 * settings.cell_size().x - settings.board_size.x / 2.0
                        + settings.cell_size().x / 2.0,
                    -1.0 * (position.y as f32 * settings.cell_size().y)
                        + settings.board_size.y / 2.0
                        - settings.cell_size().y / 2.0,
                    0.0,
                ),
                ..default()
            },
        }
    }
}

pub fn handle_stone_placed(
    mut commands: Commands,
    mut game_master: ResMut<GameMaster>,
    mut ev_stone_placed: EventReader<StonePlaced>,
    settings: Res<Settings>,
    assets: Res<OthelloAssets>,
) {
    for ev in ev_stone_placed.read() {
        let stone = ev.stone;
        let position = ev.position;

        let reversible_stones = game_master.get_reversible_at(position);
        if reversible_stones.is_empty() {
            continue;
        }

        commands.spawn(StoneBundle::new(stone, position, &settings, &assets));
        for pos in reversible_stones {
            commands.get_or_spawn(StoneBundle::new(stone, pos, &settings, &assets));
        }

        game_master.next_turn();
    }
}
