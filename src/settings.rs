use bevy::prelude::*;

const BOARD_SIZE: u32 = 8;

#[derive(Resource)]
pub struct Settings {
    pub board_size: Vec2,
    pub board_bg_color: Color,

    pub stone_black_color: Color,
    pub stone_white_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            board_size: Vec2::new(800., 800.),
            board_bg_color: Color::hsl(126.0, 0.75, 0.32),

            stone_black_color: Color::BLACK,
            stone_white_color: Color::WHITE,
        }
    }
}

impl Settings {
    pub fn stone_size(&self) -> Vec2 {
        Vec2::new(self.board_size.x / 8., self.board_size.y / 8.)
    }

    pub fn cell_size(&self) -> Vec2 {
        Vec2::new(self.board_size.x / 8., self.board_size.y / 8.)
    }

    pub fn world_pos_2_board_pos(&self, world_pos: Vec2) -> Option<BoardVec2> {
        let board_pos = Vec2::new(
            world_pos.x + self.board_size.x / 2.,
            world_pos.y + self.board_size.y / 2.,
        );
        if board_pos.x < 0.
            || board_pos.y < 0.
            || board_pos.x > self.board_size.x
            || board_pos.y > self.board_size.y
        {
            return None;
        }

        Some(BoardVec2 {
            x: (board_pos.x / self.cell_size().x) as u32,
            y: BOARD_SIZE - 1 - (board_pos.y / self.cell_size().y) as u32,
        })
    }
}

#[derive(Debug)]
pub struct BoardVec2 {
    pub x: u32,
    pub y: u32,
}
