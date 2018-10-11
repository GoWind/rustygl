
extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use glm::*;

pub mod render_gl;
use std::ptr;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut timer = sdl.timer().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_flags().debug().set();

    let window = video_subsystem
        .window("Game", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // set up shader program

    use std::ffi::{CString};
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    // set up vertex buffer object
    unsafe {
      gl::Enable(gl::DEBUG_OUTPUT);
    }



    let vertices2 = render_gl::load_cube_vertices();



    let mut vbo2: gl::types::GLuint = 0;
    let mut vao2: gl::types::GLuint = 0;
    let mut ebo:  gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao2);
        gl::GenBuffers(1, &mut vbo2);
        gl::GenBuffers(1, &mut ebo);
    }


    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.4, 1.0, 0.4);
    }

    unsafe {


        gl::BindVertexArray(vao2);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices2.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices2.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );


    }

    /*
      Set up textures
      */
    let tex0 = render_gl::set_texture(&String::from("wall.jpg"));
    let mut view = Mat4::identity();
    let mut projection = Mat4::identity();
    let mut model = rotate(&Mat4::identity(), to_radians(-55.0), &make_vec3(&[1.0, 0.0, 0.0]));
    let view  = translate(&Mat4::identity(), &make_vec3(&[0.0, 0.0, -3.0]));
    projection = perspective(800.0/600.0 as f32, to_radians(45.0), 0.1, 100.0);
    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        shader_program.set_used();
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }


        model = rotate(&model, timer.ticks() as f32 * to_radians(45.0), &make_vec3(&[0.5, 1.0, 0.0]));
        shader_program.set_uniform_mat4("model", &model).unwrap();
        shader_program.set_uniform_mat4("view", &view).unwrap();
        shader_program.set_uniform_mat4("perspective", &projection).unwrap();


        unsafe {
            gl::BindVertexArray(vao2);
            gl::BindTexture(gl::TEXTURE_2D, tex0);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }

        window.gl_swap_window();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
fn to_radians(degrees: f32) -> f32 {
   let base: f32 = pi::<f32>()  / (180 as f32);
    return base * degrees;

}
