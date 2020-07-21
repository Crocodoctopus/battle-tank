use super::misc::*;
use crate::common::*;
use std::ops::{Index, IndexMut};

pub fn tank_delay(
    tanks: usize,
    us_timestamp: u64,
    tank_states: &mut (impl IndexMut<usize, Output = TankState> + ?Sized),
) {
    // calculate ms timestamp as u16
    let ms_timestamp = (us_timestamp / 1000) as u16;

    for index in 0..tanks {
        // skip non-delayed tanks
        let (timestamp, duration) = match tank_states[index] {
            TankState::Delayed {
                timestamp,
                duration,
            } => (timestamp, duration),
            _ => continue,
        };

        // idle
        let diff = (ms_timestamp).wrapping_sub(timestamp);
        if diff > duration {
            tank_states[index] = TankState::Idle;
        }
    }
}

pub fn tank_ai(
    tanks: usize,
    us_timestamp: u64,
    tank_positions: &(impl Index<usize, Output = Vec2f> + ?Sized),
    tank_directions: &mut (impl IndexMut<usize, Output = Direction> + ?Sized),
    tank_states: &mut (impl IndexMut<usize, Output = TankState> + ?Sized),
    zkey_down: bool,
    upkey_down: bool,
    downkey_down: bool,
    leftkey_down: bool,
    rightkey_down: bool,
) -> impl Iterator<Item = usize> {
    let ms_timestamp = (us_timestamp / 1000) as u16;

    let mut v = Vec::new();

    for index in 0..tanks {
        // skip non-idle tanks
        match tank_states[index] {
            TankState::Idle => {}
            _ => continue,
        };

        //
        if zkey_down {
            v.push(index);
            continue;
        }

        // move
        let mdir = if rightkey_down {
            Some(Direction::Right)
        } else if leftkey_down {
            Some(Direction::Left)
        } else if upkey_down {
            Some(Direction::Up)
        } else if downkey_down {
            Some(Direction::Down)
        } else {
            None
        };

        if let Some(dir) = mdir {
            tank_states[index] = TankState::Moving {
                timestamp: ms_timestamp,
                duration: 400,
                start: tank_positions[index],
            };
            tank_directions[index] = dir;
        }
    }

    v.into_iter()
}

pub fn tank_push(
    tanks: usize,
    us_timestamp: u64,
    push: impl Iterator<Item = usize>,
    static_block_types: &mut (impl IndexMut<(usize, usize), Output = Option<BlockType>> + ?Sized),
    tank_positions: &(impl Index<usize, Output = Vec2f> + ?Sized),
    tank_directions: &(impl Index<usize, Output = Direction> + ?Sized),
    tank_states: &mut (impl IndexMut<usize, Output = TankState> + ?Sized),
) {
    let ms_timestamp = (us_timestamp / 1000) as u16;

    for index in push {
        // position/direction
        let position = tank_positions[index];
        let direction = tank_directions[index];

        // integer position/direction
        let dir = direction.vec2f();
        let (block_x, block_y) = (
            (position.0 / 16. + dir.0) as usize,
            (position.1 / 16. + dir.1) as usize,
        );

        match static_block_types[(block_x, block_y)] {
            // normal blocks can always be pushed
            Some(BlockType::Normal) => {
                static_block_types[(block_x, block_y)] = None;
                // push sliding BlockType::Normal
            }
            // push if direction ==, otherwise destroy
            Some(BlockType::OneWay(block_direction)) => {
                static_block_types[(block_x, block_y)] = None;
                //if direction == block_direction {
                // push sliding BlockType::OneWay
                //}
            }
            _ => continue,
        };

        tank_states[index] = TankState::Delayed {
            timestamp: ms_timestamp,
            duration: 1000,
        };
    }
}

pub fn tank_movement(
    tanks: usize,
    us_timestamp: u64,
    tank_positions: &mut (impl IndexMut<usize, Output = Vec2f> + ?Sized),
    tank_directions: &(impl Index<usize, Output = Direction> + ?Sized),
    tank_states: &mut (impl IndexMut<usize, Output = TankState> + ?Sized),
) {
    let ms_timestamp = (us_timestamp / 1000) as u16;

    for index in 0..tanks {
        // skip the non-moving tanks
        let (timestamp, duration, start) = match tank_states[index] {
            TankState::Moving {
                timestamp,
                duration,
                start,
            } => (timestamp, duration, start),
            _ => continue,
        };

        // calculate the position of the tank
        let diff = (ms_timestamp).wrapping_sub(timestamp);
        let ratio = clamp(0.0, diff as f32 / duration as f32, 1.0);
        let direction_vec = tank_directions[index].vec2f();
        tank_positions[index] = start + direction_vec * ratio * 8.0;

        // transition the tank into idle maybe
        if diff > duration {
            tank_states[index] = TankState::Idle;
        }
    }
}
