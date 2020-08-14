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
    tank_states: &(impl Index<usize, Output = TankState> + ?Sized),
    zkey_down: bool,
    upkey_down: bool,
    downkey_down: bool,
    leftkey_down: bool,
    rightkey_down: bool,
) -> (
    impl Iterator<Item = usize>,
    impl Iterator<Item = (usize, Direction)>,
) {
    let mut push = vec![];
    let mut mov = vec![];

    for index in 0..tanks {
        // skip non-idle tanks
        match tank_states[index] {
            TankState::Idle => {}
            _ => continue,
        };

        //
        if zkey_down {
            push.push(index);
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
            mov.push((index, dir));
            continue;
        }
    }

    (push.into_iter(), mov.into_iter())
}

pub fn tank_push_command(
    push: impl Iterator<Item = usize>,
    us_timestamp: u64,
    static_block_types: &mut (impl IndexMut<(usize, usize), Output = Option<BlockType>> + ?Sized),
    tank_positions: &(impl Index<usize, Output = Vec2f> + ?Sized),
    tank_directions: &(impl Index<usize, Output = Direction> + ?Sized),
    tank_states: &mut (impl IndexMut<usize, Output = TankState> + ?Sized),
    id_counter: &mut u32,
    sliding_block_ids: &mut Vec<u32>,
    sliding_block_positions: &mut Vec<Vec2f>,
    sliding_block_directions: &mut Vec<Direction>,
    sliding_block_types: &mut Vec<BlockType>,
) {
    let ms_timestamp = (us_timestamp / 1000) as u16;

    for index in push {
        // position and unit direction
        let pos = tank_positions[index];
        let udir = tank_directions[index].vec2f();

        // get the center of the 2 blocks
        let tar = pos + Vec2(8., 8.) + udir * 9.; // the location of the push
        let Vec2(tx1, ty1) = tar + Vec2(udir.1, udir.0);
        let Vec2(tx2, ty2) = tar - Vec2(udir.1, udir.0);

        // get the two blocks at (txN, tyN)
        let b1_pos = (tx1 as usize / 16, ty1 as usize / 16);
        let b2_pos = (tx2 as usize / 16, ty2 as usize / 16);
        let b1 = static_block_types[b1_pos];
        let b2 = if b1_pos == b2_pos {
            None
        } else {
            static_block_types[b2_pos]
        };

        //
        let (b, (x, y)) = match (b1, b2) {
            // if both blocks exist or don't exist
            (None, None) => continue,
            (Some(_), Some(_)) => continue,
            // if either block is Solid
            (_, Some(BlockType::Solid)) => continue,
            (Some(BlockType::Solid), _) => continue,
            // otherwise
            (None, Some(blk)) => (blk, b2_pos),
            (Some(blk), None) => (blk, b1_pos),
        };

        // do stuff
        let id = *id_counter;
        *id_counter += 1;

        sliding_block_ids.push(id);
        sliding_block_positions.push(Vec2((x * 16) as f32, (y * 16) as f32));
        sliding_block_directions.push(tank_directions[index]);
        sliding_block_types.push(b);
        static_block_types[(x, y)] = None;

        tank_states[index] = TankState::Delayed {
            timestamp: ms_timestamp,
            duration: 1000,
        };
    }
}

pub fn tank_move_command(
    mov: impl Iterator<Item = (usize, Direction)>,
    us_timestamp: u64,
    tank_positions: &(impl Index<usize, Output = Vec2f> + ?Sized),
    tank_directions: &mut (impl IndexMut<usize, Output = Direction> + ?Sized),
    tank_states: &mut (impl IndexMut<usize, Output = TankState> + ?Sized),
    static_block_types: &(impl Index<(usize, usize), Output = Option<BlockType>> + ?Sized),
) {
    let ms_timestamp = (us_timestamp / 1000) as u16;

    for (index, dir) in mov {
        let pos = tank_positions[index];
        let udir = dir.vec2f();
        let utan = Vec2(udir.1, udir.0);

        let tar = pos + Vec2(8., 8.) + udir * 9.;
        let Vec2(tx1, ty1) = tar + utan;
        let Vec2(tx2, ty2) = tar - utan;

        let b1 = static_block_types[(tx1 as usize / 16, ty1 as usize / 16)];
        let b2 = static_block_types[(tx2 as usize / 16, ty2 as usize / 16)];

        // always set direction
        tank_directions[index] = dir;

        match (b1, b2) {
            (None, None) => {}
            _ => continue,
        }

        // set moving state
        tank_states[index] = TankState::Moving {
            timestamp: ms_timestamp,
            duration: 400,
            start: tank_positions[index],
        }
    }
}

pub fn tank_movement(
    tanks: usize,
    us_timestamp: u64,
    tank_positions: &mut (impl IndexMut<usize, Output = Vec2f> + ?Sized),
    tank_directions: &mut (impl IndexMut<usize, Output = Direction> + ?Sized),
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

pub fn sliding_block_movement(
    sliding_blocks: usize,
    dt: f32,
    sliding_block_positions: &mut (impl IndexMut<usize, Output = Vec2f> + ?Sized),
    sliding_block_directions: &(impl Index<usize, Output = Direction> + ?Sized),
) {
    for index in 0..sliding_blocks {
        let Vec2(dx, dy) = sliding_block_directions[index].vec2f();
        let Vec2(x, y) = sliding_block_positions[index];
        sliding_block_positions[index] = Vec2(x + 60. * dt * dx, y + 60. * dt * dy);
    }
}
