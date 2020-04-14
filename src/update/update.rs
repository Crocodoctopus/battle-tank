use crate::common::*;
use crate::render::render_state::RenderState;

pub(super) struct State {}

impl State {
    pub(super) fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    pub(super) fn pre_step(&mut self, _timestamp: u64) {}

    pub(super) fn step(&mut self, _timestamp: u64, _simtime: u64) {}

    pub(super) fn post_step(&mut self, _timestamp: u64) {}

    pub(super) fn render_prep(&self) -> RenderState {
        // temporary frame to test rendering
        RenderState {
            kill: false,

            camera: Vec4(0., 0., 144., 144.),

            sprite_xys: Box::new([Vec2(32., 32.), Vec2(32., 64.)]),
            sprite_uvs: Box::new([Vec2(0., 16.), Vec2(32., 16.)]),

            time: 28,
            remaining_tanks: 5,
        }
    }
}
