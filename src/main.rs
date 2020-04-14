#[macro_use]
extern crate lazy_static;
extern crate crossbeam_channel;
extern crate ezgl;
extern crate glutin;
extern crate nalgebra;

#[macro_use]
mod render;
mod common;
mod io;
mod time;
mod update;

use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;

fn main() {
    // widow parameters
    let window_w = 144f64 * 5.;
    let window_h = 144f64 * 5.;

    // window creation
    let events_loop = EventLoop::new();
    let window = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (3, 0),
            opengles_version: (3, 0),
        })
        .with_vsync(true)
        .build_windowed(
            WindowBuilder::new()
                .with_resizable(false)
                .with_title("Blockhop")
                .with_inner_size(glutin::dpi::LogicalSize::new(window_w, window_h)),
            &events_loop,
        )
        .unwrap();

    // sender-receiver pair for update -> render messages
    let (render_s, render_r) = crossbeam_channel::bounded(0);

    // sender-receiver pair for input -> update messages
    let (input_s, input_r) = crossbeam_channel::unbounded();

    // update thread
    // std::thread::spawn(move || crate::update::update_thread(render_s, input_r));

    // render thread
    std::thread::spawn(move || crate::render::render_thread(window, render_r));

    //////////////////////
    // temporary frame to test rendering
    use crate::common::*;
    use crate::render::render_state::RenderState;
    render_s
        .send(RenderState {
            kill: false,

            camera: Vec4(0., 0., 144., 144.),

            sprite_xys: Box::new([Vec2(32., 32.), Vec2(32., 64.)]),
            sprite_uvs: Box::new([Vec2(0., 16.), Vec2(32., 16.)]),

            time: 28,
            remaining_tanks: 5,
        })
        .unwrap();
    //////////////////////

    // input "thread" (forward events ot update)
    events_loop.run(move |event, _, out| {
        let event = match map_event(event) {
            Some(e) => e,
            None => return (),
        };

        use glutin::event_loop::ControlFlow;
        match input_s.send(event) {
            Ok(_) => {}
            Err(_) => *out = ControlFlow::Exit,
        }
    });
}

fn map_event(event: glutin::event::Event<()>) -> Option<crate::update::event::Event> {
    use crate::update::event::*;
    use glutin::dpi::PhysicalPosition;
    use glutin::event::ElementState;
    use glutin::event::KeyboardInput;
    use glutin::event::WindowEvent;

    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            WindowEvent::CursorMoved {
                position: PhysicalPosition { x, y },
                ..
            } => return Some(Event::MouseMove(x as f32, y as f32)),

            WindowEvent::MouseInput { state, button, .. } => {
                use glutin::event::MouseButton;
                let button = match button {
                    MouseButton::Left => Key::Rmb,
                    MouseButton::Right => Key::Lmb,
                    _ => return None,
                };

                return match state {
                    ElementState::Pressed => Some(Event::KeyDown(button)),
                    ElementState::Released => Some(Event::KeyUp(button)),
                };
            }

            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => {
                use glutin::event::VirtualKeyCode;
                let key = match key {
                    VirtualKeyCode::A => Key::A,
                    VirtualKeyCode::D => Key::D,
                    VirtualKeyCode::S => Key::S,
                    VirtualKeyCode::W => Key::W,
                    _ => return None,
                };

                return match state {
                    ElementState::Pressed => Some(Event::KeyDown(key)),
                    ElementState::Released => Some(Event::KeyUp(key)),
                };
            }
            _ => return None,
        },
        _ => return None,
    };
}
