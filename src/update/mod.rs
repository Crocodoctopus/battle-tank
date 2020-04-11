use crate::render::render_state::RenderState;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use event::Event;
use update::State;

pub mod event;
mod update;

pub fn update_thread(render_s: Sender<RenderState>, _input_r: Receiver<Event>) {
    // state setup
    let mut state = State::new();

    // the most amount of time an update can simulate
    let max_time_per_update = 250_000; // microseconds, 250ms

    // loop time
    let mut game_timestamp = crate::time::get_microseconds_as_u64();
    loop {
        // pre-step
        state.pre_step(game_timestamp);

        // step (part 1)
        let real_timestamp = crate::time::get_microseconds_as_u64();
        while real_timestamp - game_timestamp > max_time_per_update {
            state.step(game_timestamp, max_time_per_update);
            game_timestamp += max_time_per_update;
        }

        // step (part 2)
        state.step(game_timestamp, real_timestamp - game_timestamp);
        game_timestamp = real_timestamp;

        // post-step
        state.post_step(game_timestamp);

        // render prep
        render_s.send(state.render_prep()).unwrap();
    }
}
