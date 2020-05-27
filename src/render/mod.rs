mod render_io;
pub mod render_state;

use crate::common::*;
use crossbeam_channel::Receiver;
use glutin::window::Window;
use glutin::ContextWrapper;
use glutin::NotCurrent;
use render_state::RenderState;

pub fn render_thread(window: ContextWrapper<NotCurrent, Window>, render_r: Receiver<RenderState>) {
    // set gl context
    let window = unsafe {
        let context = window.make_current().unwrap();
        ezgl::gl::load_with(|s| context.get_proc_address(s) as *const _);
        ezgl::gl::ClearColor(1., 1., 1., 1.);
        ezgl::bind_vao();
        context
    };

    // load resources
    let textures = render_io::load_textures_from("resources/textures");
    let programs = render_io::load_programs_from("resources/shaders");

    // some constants
    const MAX_TANKS: usize = 5;
    const MAX_EXPLOSIONS: usize = MAX_TANKS;
    const MAX_BLOCKS: usize = (160/16 + 1) * (144/16 + 1);
    const MAX_SPRITES: usize = MAX_TANKS + MAX_EXPLOSIONS + MAX_BLOCKS;

    // all the sprite data gets dumped into these things
    let mut xy_data: Box<[(f32, f32)]> = vec![<_>::default(); MAX_SPRITES * 4].into_boxed_slice();
    let mut uv_data: Box<[(f32, f32)]> = vec![<_>::default(); MAX_SPRITES * 4].into_boxed_slice();

    // gl buffers
    let mut xy_buf = ezgl::Buffer::<(f32, f32)>::from(ezgl::gl::ARRAY_BUFFER, &xy_data);
    let mut uv_buf = ezgl::Buffer::<(f32, f32)>::from(ezgl::gl::ARRAY_BUFFER, &uv_data);
    let ibo = ezgl::Buffer::from(
        ezgl::gl::ELEMENT_ARRAY_BUFFER,
        &(0..MAX_SPRITES as u16).into_iter().fold(
            Vec::<u16>::with_capacity(MAX_SPRITES * 6),
            |mut acc, v| {
                acc.extend([v * 4, v * 4 + 1, v * 4 + 2, v * 4 + 2, v * 4 + 3, v * 4].iter());
                acc
            },
        ),
    );

    // process frames sent from update
    for (i, frame) in render_r.iter().enumerate() {
        if frame.exit {
            break;
        }

        unsafe {
            ezgl::gl::Clear(ezgl::gl::COLOR_BUFFER_BIT);
        }

        // camera
        let view_transform = camera(
            (frame.camera.0/0.25).floor() * 0.25,
            (frame.camera.1/0.25).floor() * 0.25,
            (frame.camera.2/0.25).floor() * 0.25,
            (frame.camera.3/0.25).floor() * 0.25,
        );

        //
        let mut sprite_counter = 0usize;

        // static blocks
        let Vec2(x_offset, y_offset) = frame.static_blocks_offset;
        frame.static_block_types.for_each(|x, y, block_type_opt| {
            if let Some(block_type) = block_type_opt {
                let x = (x * 16) as f32;
                let y = (y * 16) as f32;
                xy_data[sprite_counter * 4 + 0] = (x + x_offset + 0., y + y_offset + 0.);
                xy_data[sprite_counter * 4 + 1] = (x + x_offset + 16., y + y_offset + 0.);
                xy_data[sprite_counter * 4 + 2] = (x + x_offset + 16., y + y_offset + 16.);
                xy_data[sprite_counter * 4 + 3] = (x + x_offset + 0., y + y_offset + 16.);
                let Vec2(u_offset, v_offset) = crate::update::misc::block_to_uv(*block_type);
                uv_data[sprite_counter * 4 + 0] = (u_offset + 0. + 0.05, v_offset + 0. + 0.05);
                uv_data[sprite_counter * 4 + 1] = (u_offset + 16. - 0.05, v_offset + 0. + 0.05);
                uv_data[sprite_counter * 4 + 2] = (u_offset + 16. - 0.05, v_offset + 16. - 0.05);
                uv_data[sprite_counter * 4 + 3] = (u_offset + 0. + 0.05, v_offset + 16. - 0.05);
                sprite_counter += 1;
            }
        });

        // upload the buffer data
        xy_buf.splice(0, &xy_data[0..sprite_counter * 4]).unwrap();
        uv_buf.splice(0, &uv_data[0..sprite_counter * 4]).unwrap();

        // gl
        ezgl::Draw::start_tri_draw(sprite_counter as u32 * 2, &programs["sprite"], &ibo)
            .with_buffer(&xy_buf, 0)
            .with_buffer(&uv_buf, 1)
            .with_uniform(ezgl::GLSLAny::Mat3(view_transform), 0)
            .with_texture(&textures["spritesheet.png"], 1)
            .draw();

        unsafe {
            window.swap_buffers().unwrap();
        }
    }
}

fn camera(x: f32, y: f32, w: f32, h: f32) -> ezgl::Mat3 {
    use nalgebra::*;

    let mut matrix = Matrix3::identity();
    matrix *= Matrix3::new_nonuniform_scaling(&Vector2::new(2. / w, -2. / h));
    matrix *= Matrix3::new_translation(&Vector2::new(-w / 2. - x, -h / 2. - y));

    let mut t = ezgl::Mat3([0., 0., 0., 0., 0., 0., 0., 0., 0.]);
    t.0.clone_from_slice(matrix.as_slice());
    t
}
