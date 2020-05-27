#[macro_use]
extern crate lazy_static;
extern crate crossbeam_channel;
extern crate ezgl;
extern crate glutin;
extern crate nalgebra;

#[macro_use]
mod render;
mod array2d;
mod common;
mod io;
mod time;
mod update;

use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;

fn main() {
    // widow parameters
    let window_w = 160f64 * 5.;
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
    std::thread::spawn(move || crate::update::update_thread(render_s, input_r));

    // render thread
    std::thread::spawn(move || crate::render::render_thread(window, render_r));

    // input "thread" (forward events ot update)
    events_loop.run(move |event, _, out| {
        // end the loop on any "window destroyed" events
        if let glutin::event::Event::WindowEvent {
            event: glutin::event::WindowEvent::Destroyed,
            ..
        } = event
        {
            *out = glutin::event_loop::ControlFlow::Exit
        }

        // map and forward the event to the update thread
        if let Some(e) = map_event(event) {
            input_s.send(e);
        }
    });
}

fn map_event(event: glutin::event::Event<()>) -> Option<crate::update::event::Event> {
    use crate::update::event::*;
    use glutin::dpi::PhysicalPosition;
    use glutin::event::ElementState;
    use glutin::event::KeyboardInput;
    use glutin::event::WindowEvent;

    // https://docs.rs/glutin/0.24.0/glutin/event/enum.WindowEvent.html
    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => return Some(Event::Exit),

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
                    VirtualKeyCode::B => Key::B,
                    VirtualKeyCode::C => Key::C,
                    VirtualKeyCode::D => Key::D,
                    VirtualKeyCode::E => Key::E,
                    VirtualKeyCode::F => Key::F,
                    VirtualKeyCode::G => Key::G,
                    VirtualKeyCode::H => Key::H,
                    VirtualKeyCode::I => Key::I,
                    VirtualKeyCode::J => Key::J,
                    VirtualKeyCode::K => Key::K,
                    VirtualKeyCode::L => Key::L,
                    VirtualKeyCode::M => Key::M,
                    VirtualKeyCode::N => Key::N,
                    VirtualKeyCode::O => Key::O,
                    VirtualKeyCode::P => Key::P,
                    VirtualKeyCode::Q => Key::Q,
                    VirtualKeyCode::R => Key::R,
                    VirtualKeyCode::S => Key::S,
                    VirtualKeyCode::T => Key::T,
                    VirtualKeyCode::U => Key::U,
                    VirtualKeyCode::V => Key::V,
                    VirtualKeyCode::W => Key::W,
                    VirtualKeyCode::X => Key::X,
                    VirtualKeyCode::Y => Key::Y,
                    VirtualKeyCode::Z => Key::Z,
                    VirtualKeyCode::Up => Key::Up,
                    VirtualKeyCode::Down => Key::Down,
                    VirtualKeyCode::Left => Key::Left,
                    VirtualKeyCode::Right => Key::Right,
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
