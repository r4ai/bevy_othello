use bevy::prelude::*;

use crate::{assets::OthelloAssets, game_master::GameMaster, settings::Settings, StonePlaced};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoneType {
    Black,
    White,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stone {
    pub stone_type: StoneType,
}

impl Stone {
    pub fn new(stone_type: StoneType) -> Self {
        Self { stone_type }
    }

    pub fn black() -> Self {
        Self {
            stone_type: StoneType::Black,
        }
    }

    pub fn white() -> Self {
        Self {
            stone_type: StoneType::White,
        }
    }

    pub fn reverse(&mut self) {
        self.stone_type = match self.stone_type {
            StoneType::Black => StoneType::White,
            StoneType::White => StoneType::Black,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
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
        let color = match stone.stone_type {
            StoneType::Black => assets.stone_black.clone(),
            StoneType::White => assets.stone_white.clone(),
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
    stones: Query<(Entity, &Position), With<Stone>>,
    settings: Res<Settings>,
    assets: Res<OthelloAssets>,
) {
    if ev_stone_placed.len() != 1 {
        game_master.stone_placed = false;
        return;
    }

    for ev in ev_stone_placed.read() {
        let placed_stone = ev.stone;
        let position = ev.position;

        let reversible_stones = game_master.get_reversible_at(position);
        if reversible_stones.is_empty() {
            continue;
        }

        commands.spawn(StoneBundle::new(
            Stone::new(placed_stone),
            position,
            &settings,
            &assets,
        ));
        for pos in reversible_stones {
            for (entity, position) in stones.iter() {
                if position == &pos {
                    println!("reverse: {:?} to {:?}", position, placed_stone);
                    commands.entity(entity).despawn();
                    commands.spawn(StoneBundle::new(
                        Stone::new(placed_stone),
                        pos,
                        &settings,
                        &assets,
                    ));
                }
            }
        }

        game_master.next_turn();
    }
}
