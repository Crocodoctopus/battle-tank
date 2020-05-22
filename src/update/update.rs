use super::event::Event;
use super::misc::*;
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

    // various game properties
    camera: Vec4f,
    rem_time: u64, // in microseconds
    rem_tanks: u8,

    // static blocks
    arena_width: usize,
    arena_height: usize,
    static_block_types: Box<[Option<BlockType>]>,
}

impl State {
    pub(super) fn new() -> Self {
        let arena_width = 2;
        let arena_height = 2;
        let static_block_types =
            Box::new([None, Some(BlockType::Normal), Some(BlockType::Solid), None]);

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

            camera: Vec4(0f32, 0f32, 144f32, 144f32),
            rem_time: 66,
            rem_tanks: 8,

            arena_width,
            arena_height,
            static_block_types,
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
        //
        let mut sprite_xys = Vec::with_capacity(self.arena_width * self.arena_height);
        let mut sprite_uvs = Vec::with_capacity(self.arena_width * self.arena_height);

        // static sprites (push the entire arena)
        for y in 0..self.arena_height {
            for x in 0..self.arena_width {
                if let Some(tile_type) = self.static_block_types[x + y * self.arena_height] {
                    let block_uv = block_to_uv(tile_type);
                    sprite_xys.push(Vec2((x * 16) as f32, (y * 16) as f32));
                    sprite_uvs.push(block_uv);
                }
            }
        }

        // temporary frame to test rendering
        RenderState {
            exit: self.exit,

            camera: self.camera,

            sprite_xys: sprite_xys.into_boxed_slice(),
            sprite_uvs: sprite_uvs.into_boxed_slice(),

            time: self.rem_time as u8,
            remaining_tanks: self.rem_tanks,
        }
    }
}
