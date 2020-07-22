use crate::array2d::*;
use crate::common::*;
use crate::update::misc::*;

pub struct RenderState {
    // misc
    pub exit: bool,
    pub time: u8,
    pub remaining_tanks: u8,
    pub camera: Vec4<f32>,

    // static blocks
    pub static_blocks_offset: Vec2f,
    pub static_block_types: Array2D<Option<BlockType>>,

    // sliding blocks
    pub sliding_block_positions: Box<[Vec2f]>,
    pub sliding_block_types: Box<[BlockType]>,

    // tanks
    pub tank_positions: Box<[Vec2f]>,
    pub tank_directions: Box<[Direction]>,
    pub tank_states: Box<[TankState]>,
}
