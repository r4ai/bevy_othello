use bevy::{app::AppExit, prelude::*};

use crate::stone::{Position, Stone, StoneType};

#[derive(Resource)]
pub struct GameMaster {
    pub turn: StoneType,
    pub stone_placed: bool,
    pub board: [[Option<StoneType>; 8]; 8],
    skipped_count: u32,
}

impl Default for GameMaster {
    fn default() -> Self {
        Self {
            turn: StoneType::Black,
            stone_placed: false,
            board: [[None; 8]; 8],
            skipped_count: 0,
        }
    }
}

impl GameMaster {
    pub fn next_turn(&mut self) {
        self.turn = match self.turn {
            StoneType::Black => StoneType::White,
            StoneType::White => StoneType::Black,
        };
        self.stone_placed = false;
        if self.does_exist_placable_pos() {
            self.skipped_count = 0;
        } else {
            self.skipped_count += 1;
        }
    }

    pub fn is_placable_at(&self, pos: Position) -> bool {
        for x in (-1)..=1 {
            for y in (-1)..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                if !self.get_reversible(pos, Direction { x, y }).is_empty() {
                    return true;
                }
            }
        }
        false
    }

    pub fn does_exist_placable_pos(&self) -> bool {
        for x in 0..8 {
            for y in 0..8 {
                if self.is_placable_at(Position {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                }) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_reversible_at(&self, pos: Position) -> Vec<Position> {
        let mut reversible_stones = Vec::with_capacity(8);
        for x in (-1)..=1 {
            for y in (-1)..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                reversible_stones.extend(self.get_reversible(pos, Direction { x, y }));
            }
        }
        reversible_stones
    }

    fn get_reversible(&self, pos: Position, dir: Direction) -> Vec<Position> {
        if pos.x >= 8 || pos.y >= 8 {
            return vec![];
        }

        if self.board[pos.y as usize][pos.x as usize].is_some() {
            return vec![];
        }

        let mut reversible_stones = Vec::with_capacity(8);
        let mut x = pos.x as i32;
        let mut y = pos.y as i32;
        loop {
            x += dir.x;
            y += dir.y;
            if x >= 8 || y >= 8 || x < 0 || y < 0 {
                reversible_stones.clear();
                break;
            }
            if let Some(stone) = self.board[y as usize][x as usize] {
                if stone == self.turn {
                    break;
                }
                reversible_stones.push(Position {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            } else {
                reversible_stones.clear();
                break;
            }
        }
        reversible_stones
    }
}

struct Direction {
    x: i32,
    y: i32,
}

pub fn board_sync_system(mut game_master: ResMut<GameMaster>, stones: Query<(&Stone, &Position)>) {
    for (stone, pos) in stones.iter() {
        if pos.x >= 8 || pos.y >= 8 {
            eprintln!("Invalid position: {:?} ({:?})", pos, stone);
            continue;
        }
        game_master.board[pos.y as usize][pos.x as usize] = Some(stone.stone_type);
    }
}

pub fn game_end_system(game_master: ResMut<GameMaster>, mut exit: EventWriter<AppExit>) {
    if game_master.skipped_count >= 1 {
        println!("Game end");
        exit.send(AppExit);
    }
}
