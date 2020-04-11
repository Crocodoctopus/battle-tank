pub mod render_state;

use crossbeam_channel::Receiver;
use glutin::window::Window;
use glutin::ContextWrapper;
use glutin::NotCurrent;
use render_state::RenderState;

pub fn render_thread(
    _window: ContextWrapper<NotCurrent, Window>,
    _render_r: Receiver<RenderState>,
) {
    unimplemented!()
}
