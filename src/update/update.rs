use super::event::Event;
use crate::common::*;
use crate::render::render_state::RenderState;

pub(super) struct State {
    exit: bool,

    // some keyboard stuff
    upkey_down: bool,
    downkey_down: bool,
    leftkey_down: bool,
    rightkey_down: bool,
    zkey_down: bool,

    upkey_was_down: bool,
    downkey_was_down: bool,
    leftkey_was_down: bool,
    rightkey_was_down: bool,
    zkey_was_down: bool,
}

impl State {
    pub(super) fn new() -> Self {
        Self {
            exit: false,
            upkey_down: false,
            downkey_down: false,
            leftkey_down: false,
            rightkey_down: false,
            zkey_down: false,
            upkey_was_down: false,
            downkey_was_down: false,
            leftkey_was_down: false,
            rightkey_was_down: false,
            zkey_was_down: false,
        }
    }

    pub(super) fn exit(&self) -> bool {
        self.exit
    }

    pub(super) fn pre_step(&mut self, _timestamp: u64, events: impl Iterator<Item = Event>) {
        self.upkey_was_down = self.upkey_down;
        self.downkey_was_down = self.downkey_down;
        self.rightkey_was_down = self.rightkey_down;
        self.leftkey_was_down = self.leftkey_down;
        self.zkey_was_down = self.zkey_down;

        use super::event::Key;
        for event in events {
            match event {
                Event::Exit => dbg!(self.exit = true),
                Event::KeyDown(Key::Up) => dbg!(self.upkey_down = true),
                Event::KeyDown(Key::Down) => dbg!(self.downkey_down = true),
                Event::KeyDown(Key::Left) => dbg!(self.leftkey_down = true),
                Event::KeyDown(Key::Right) => dbg!(self.rightkey_down = true),
                Event::KeyUp(Key::Up) => dbg!(self.upkey_down = false),
                Event::KeyUp(Key::Down) => dbg!(self.downkey_down = false),
                Event::KeyUp(Key::Left) => dbg!(self.leftkey_down = false),
                Event::KeyUp(Key::Right) => dbg!(self.rightkey_down = false),
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
