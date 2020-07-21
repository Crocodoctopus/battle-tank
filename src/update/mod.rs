use crate::common::*;
use crate::render::render_state::RenderState;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use event::Event;
use update::State;

pub mod event;
mod func;
pub mod misc;
mod update;

pub fn time<T>(mut func: impl FnMut() -> T) -> (u32, T) {
    let start = crate::time::get_microseconds_as_u64() as u32;
    let ret = func();
    let end = crate::time::get_microseconds_as_u64() as u32;
    (end.wrapping_sub(start), ret)
}

pub fn update_thread(render_s: Sender<RenderState>, input_r: Receiver<Event>) {
    // frametime stuff
    let mut frametimes: Vec<Vec4<u32>> = Vec::new();

    // state setup
    let mut state = State::new();

    // the most amount of time an update can simulate
    let max_time_per_update = 250_000; // microseconds, 250ms

    // loop time
    let mut game_timestamp = crate::time::get_microseconds_as_u64();
    loop {
        // pre-step
        let (t1, _) = time(|| state.pre_step(game_timestamp, input_r.try_iter()));

        // step (part 1)
        let (t2, _) = time(|| {
            let real_timestamp = crate::time::get_microseconds_as_u64();
            while real_timestamp - game_timestamp > max_time_per_update {
                state.step(game_timestamp, max_time_per_update);
                game_timestamp += max_time_per_update;
            }

            // step (part 2)
            state.step(game_timestamp, real_timestamp - game_timestamp);
            game_timestamp = real_timestamp;
        });

        // post-step
        let (t3, _) = time(|| state.post_step(game_timestamp));

        // render prep
        let (t4, rs) = time(|| state.render_prep());

        // time
        frametimes.push(Vec4(t1, t2, t3, t4));
        if frametimes.len() == 120 {
            let Vec4(t1_acc, t2_acc, t3_acc, t4_acc) =
                frametimes.iter().fold(Vec4(0, 0, 0, 0), |a, &b| a + b);
            let Vec4(t1_acc, t2_acc, t3_acc, t4_acc) = Vec4(
                t1_acc as f32 / 1_000f32 / 120f32,
                t2_acc as f32 / 1_000f32 / 120f32,
                t3_acc as f32 / 1_000f32 / 120f32,
                t4_acc as f32 / 1_000f32 / 120f32,
            );

            /*println!(
                "Frametime: {:?}ms\n  prestep: {:?}ms\n  step: {:?}ms\n  poststep: {:?}ms\n  render step: {:?}ms",
                t1_acc + t2_acc + t3_acc + t4_acc,
                t1_acc,
                t2_acc,
                t3_acc,
                t4_acc
            );*/
            frametimes.clear();
        }

        // send render state and wait
        render_s.send(rs).unwrap();

        // break on exit flag
        if state.exit() {
            break;
        }
    }
}
