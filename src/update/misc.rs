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
        Direction::Up => Vec2(0.0, -1.0),
        Direction::Down => Vec2(0.0, 1.0),
        Direction::Left => Vec2(-1.0, 0.0),
        Direction::Right => Vec2(1.0, 0.0),
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

#[derive(Copy, Clone, Debug)]
pub enum TankState {
    Idle,
    Moving,
    Delayed(u32), // microsecond counter
}
