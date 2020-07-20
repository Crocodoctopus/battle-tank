use super::event::Event;
use super::func::*;
use super::misc::*;
use crate::array2d::*;
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
    static_block_types: Array2D<Option<BlockType>>,

    // tanks
    tank_ids: Vec<u32>,
    tank_positions: Vec<Vec2f>,
    tank_directions: Vec<Direction>,
    tank_states: Vec<TankState>,
}

impl State {
    pub(super) fn new() -> Self {
        let static_block_types = Array2D::from_box(
            4,
            4,
            Box::new([
                Some(BlockType::Solid),
                Some(BlockType::Solid),
                Some(BlockType::Solid),
                Some(BlockType::Solid),
                Some(BlockType::Solid),
                None,
                None,
                Some(BlockType::Solid),
                Some(BlockType::Solid),
                None,
                None,
                Some(BlockType::Solid),
                Some(BlockType::Solid),
                Some(BlockType::Solid),
                Some(BlockType::Solid),
                Some(BlockType::Solid),
            ]),
        );

        let tank_ids = vec![0];
        let tank_positions = vec![Vec2(16f32, 16f32)];
        let tank_directions = vec![Direction::Up];
        let tank_states = vec![TankState::Idle];

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

            camera: Vec4(0f32, 0f32, 160f32, 144f32),
            rem_time: 66,
            rem_tanks: 8,

            static_block_types,

            tank_ids,
            tank_positions,
            tank_directions,
            tank_states,
        }
    }

    pub(super) fn exit(&self) -> bool {
        self.exit
    }

    pub(super) fn pre_step(
        &mut self,
        _us_frame_timestamp: u64,
        events: impl Iterator<Item = Event>,
    ) {
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

    pub(super) fn step(&mut self, us_frame_timestamp: u64, simtime: u64) {
        let dt = simtime as f32 / 1000000f32;

        // temp camera movement
        let Vec2(x, y) = *self.tank_positions.get(0).unwrap_or(&Vec2(0., 0.));
        self.camera.0 = x - self.camera.2 / 2.;
        self.camera.1 = y - self.camera.3 / 2.;

        // clamp
        if self.camera.0 < 0f32 {
            self.camera.0 = 0f32;
        }
        if self.camera.1 < 0f32 {
            self.camera.1 = 0f32;
        }

        // tanks
        let tanks = self.tank_ids.len();

        // process tank delay
        tank_delay(tanks, us_frame_timestamp, &mut self.tank_states[..]);

        // process tank AI
        let _push = tank_ai(
            tanks,
            us_frame_timestamp,
            &self.tank_positions[..],
            &mut self.tank_directions[..],
            &mut self.tank_states[..],
            self.zkey_down,
            self.upkey_down,
            self.downkey_down,
            self.leftkey_down,
            self.rightkey_down,
        );

        // process push
        /*
        tank_push(
            tanks,
            push,
            &mut self.static_block_types,
            &mut self.tank_states[..],
        );
        */

        // process tank movement
        tank_movement(
            tanks,
            us_frame_timestamp,
            &mut self.tank_positions[..],
            &self.tank_directions[..],
            &mut self.tank_states[..],
        );
    }

    pub(super) fn post_step(&mut self, _timestamp: u64) {}

    pub(super) fn render_prep(&self) -> RenderState {
        // clone region
        let x1 = (self.camera.0 / 16f32).floor() as usize;
        let y1 = (self.camera.1 / 16f32).floor() as usize;
        let x2 = ((self.camera.0 + self.camera.2) / 16f32).ceil() as usize;
        let y2 = ((self.camera.1 + self.camera.3) / 16f32).ceil() as usize;
        let static_block_types = self.static_block_types.clone_sub(x1..x2, y1..y2);

        // temporary frame to test rendering
        RenderState {
            exit: self.exit,
            time: self.rem_time as u8,
            remaining_tanks: self.rem_tanks,
            camera: self.camera,

            static_blocks_offset: Vec2((x1 * 16) as f32, (y1 * 16) as f32),
            static_block_types,

            tank_positions: self.tank_positions.clone().into_boxed_slice(),
            tank_directions: self.tank_directions.clone().into_boxed_slice(),
            tank_states: self.tank_states.clone().into_boxed_slice(),
        }
    }
}
