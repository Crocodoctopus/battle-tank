use crate::common::*;

pub struct RenderState {
    // kill the render thread
    pub kill: bool,

    // just the camera folks, move along
    pub camera: Vec4<f32>,

    // block
    pub blocks_origin: Vec2<f32>,
    pub blocks_size: Vec2<u8>,
    pub block_uvs: Box<[Vec2<f32>]>,

    // sprite
    pub sprite_xys: Box<[Vec2<f32>]>,
    pub sprite_uvs: Box<[Vec2<f32>]>,

    // draw data
    pub time: u8,
    pub remaining_tanks: u8,
}
