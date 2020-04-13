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
        ezgl::gl::ClearColor(0., 0., 0., 1.);
        ezgl::bind_vao();
        context
    };

    // load resources
    let textures = render_io::load_textures_from("resources/textures");
    let programs = render_io::load_programs_from("resources/shaders");

    // some constants
    const MAX_TANKS: usize = 5;
    const MAX_EXPLOSIONS: usize = MAX_TANKS;
    const MAX_BLOCKS: usize = 100;
    const MAX_SPRITES: usize = MAX_TANKS + MAX_EXPLOSIONS + MAX_BLOCKS;

    // all the sprite data gets dumped into these things
    let mut xy_data: Box<[(f32, f32)]> =
        vec![<_>::default(); MAX_SPRITES * 4].into_boxed_slice();
    let mut uv_data: Box<[(f32, f32)]> =
        vec![<_>::default(); MAX_SPRITES * 4].into_boxed_slice();

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
    	// camera
    	let view_transform = camera(frame.camera.0, frame.camera.1, frame.camera.2, frame.camera.3);

    	// indices
    	let mut xy_data_index: usize = 0;
    	let mut uv_data_index: usize = 0;

        let blocks_origin = frame.blocks_origin;
        let Vec2(blocks_size_x, blocks_size_y) = frame.blocks_size;
        for y in 0..blocks_size_y {
            for x in 0..blocks_size_x {
                let pos = Vec2(x as f32, y as f32) * Vec2(16., 16.);
                xy_data[xy_data_index + 0] = (blocks_origin + pos + Vec2(0., 0.)).tuple();
                xy_data[xy_data_index + 1] = (blocks_origin + pos + Vec2(16., 0.)).tuple();
                xy_data[xy_data_index + 2] = (blocks_origin + pos + Vec2(16., 16.)).tuple();
                xy_data[xy_data_index + 3] = (blocks_origin + pos + Vec2(0., 16.)).tuple();
                xy_data_index += 4
            }
        }

        // block uv
        for &uv in frame.block_uvs.into_iter() {
            uv_data[uv_data_index + 0] = (uv + Vec2(0., 0.)).tuple();
            uv_data[uv_data_index + 1] = (uv + Vec2(16., 0.)).tuple();
            uv_data[uv_data_index + 2] = (uv + Vec2(16., 16.)).tuple();
            uv_data[uv_data_index + 3] = (uv + Vec2(0., 16.)).tuple();
            uv_data_index += 4;
        }

        // sprite xy
        for &xy in frame.sprite_xys.into_iter() {
        	xy_data[xy_data_index + 0] = (xy + Vec2(0., 0.)).tuple(); 
        	xy_data[xy_data_index + 1] = (xy + Vec2(16., 0.)).tuple();
        	xy_data[xy_data_index + 2] = (xy + Vec2(16., 16.)).tuple();
        	xy_data[xy_data_index + 3] = (xy + Vec2(0., 16.)).tuple(); 
        	xy_data_index += 4;
        }

        // sprite uv
        for &uv in frame.sprite_uvs.into_iter() {
        	uv_data[uv_data_index + 0] = (uv + Vec2(0., 0.)).tuple(); 
        	uv_data[uv_data_index + 1] = (uv + Vec2(16., 0.)).tuple();
        	uv_data[uv_data_index + 2] = (uv + Vec2(16., 16.)).tuple();
        	uv_data[uv_data_index + 3] = (uv + Vec2(0., 16.)).tuple();
        	uv_data_index += 4;
        }

        // upload the buffer data
        xy_buf.splice(0, &xy_data[0..xy_data_index * 4]).unwrap();
        uv_buf.splice(0, &uv_data[0..uv_data_index * 4]).unwrap();

        // gl
        ezgl::Draw::start_tri_draw(xy_data_index as u32 * 2, &programs["sprite"], &ibo)
        	.with_buffer(&xy_buf, 0)
        	.with_buffer(&uv_buf, 1)
        	.with_uniform(ezgl::GLSLAny::Mat3(view_transform), 0)
        	.with_texture(&textures["spritesheet.png"], 1)
        	.draw();

        unsafe {
            window.swap_buffers().unwrap();
            ezgl::gl::Clear(ezgl::gl::COLOR_BUFFER_BIT);
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