use super::event::Event;
use crate::common::*;
use crate::render::render_state::RenderState;

pub(super) struct State {
    exit: bool,
}

impl State {
    pub(super) fn new() -> Self {
        Self { exit: false }
    }

    pub(super) fn exit(&self) -> bool {
        self.exit
    }

    pub(super) fn pre_step(&mut self, _timestamp: u64, events: impl Iterator<Item = Event>) {
        use super::event::Key;
        for event in events {
            match event {
                Event::Exit => self.exit = true,
                _ => {}
            }
        }
    }

    pub(super) fn step(&mut self, _timestamp: u64, _simtime: u64) {}

    pub(super) fn post_step(&mut self, _timestamp: u64) {}

    pub(super) fn render_prep(&self) -> RenderState {
        // temporary frame to test rendering
        RenderState {
            exit: self.exit,

            camera: Vec4(0., 0., 144., 144.),

            sprite_xys: Box::new([Vec2(32., 32.), Vec2(32., 64.)]),
            sprite_uvs: Box::new([Vec2(0., 16.), Vec2(32., 16.)]),

            time: 28,
            remaining_tanks: 5,
        }
    }
}
