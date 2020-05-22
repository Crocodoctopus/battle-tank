use crate::common::*;

// general
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn direction_to_vec2f(direction: Direction) -> Vec2f {
    match direction {
        Up => Vec2(0.0, -1.0),
        Down => Vec2(0.0, 1.0),
        Left => Vec2(-1.0, 0.0),
        Right => Vec2(1.0, 0.0),
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BlockType {
    Normal,
    Solid,
    OneWay(Direction),
}

// this needs to change
pub fn block_to_uv(block_type: BlockType) -> Vec2f {
    match block_type {
        BlockType::Normal => Vec2(16., 0.),
        BlockType::Solid => Vec2(32., 0.),
        BlockType::OneWay(direction) => match direction {
            _ => unimplemented!(),
        },
    }
}
