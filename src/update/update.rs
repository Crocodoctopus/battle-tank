use crate::render::render_state::RenderState;

pub(super) struct State {}

impl State {
    pub(super) fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    pub(super) fn pre_step(&mut self, _timestamp: u64) {
        unimplemented!()
    }

    pub(super) fn step(&mut self, _timestamp: u64, _simtime: u64) {
        unimplemented!()
    }

    pub(super) fn post_step(&mut self, _timestamp: u64) {
        unimplemented!()
    }

    pub(super) fn render_prep(&self) -> RenderState {
        unimplemented!()
    }
}
