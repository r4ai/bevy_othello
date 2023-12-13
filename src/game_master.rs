use bevy::prelude::*;

use crate::stone::{Position, Stone};

#[derive(Resource)]
pub struct GameMaster {
    pub turn: Stone,
    pub stone_placed: bool,
    pub board: [[Option<Stone>; 8]; 8],
}

impl Default for GameMaster {
    fn default() -> Self {
        Self {
            turn: Stone::Black,
            stone_placed: false,
            board: [[None; 8]; 8],
        }
    }
}

impl GameMaster {
    pub fn next_turn(&mut self) {
        self.turn = match self.turn {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        };
        self.stone_placed = false;
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
        game_master.board[pos.y as usize][pos.x as usize] = Some(*stone);
    }
}
