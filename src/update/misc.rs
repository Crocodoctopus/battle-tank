use crate::common::*;

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
    Moving {
        timestamp: u16, // milliseconds
        duration: u16,  // milliseconds
        start: Vec2f,
    },
    Delayed {
        timestamp: u16, // milliseconds
        duration: u16,  // milliseconds
    },
}
