
extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate game;

use crate::glm::*;
use game::render_gl;
use game::render_gl::camera::*;
use std::ptr;

#[allow(unused_variables, non_snake_case)]
fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

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
        &CString::new(include_str!("lightingmap.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("lightingmap.frag")).unwrap()
    ).unwrap();

    let mut shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    let lamp_vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("lamp.vert")).unwrap()
    ).unwrap();

    let lamp_frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("lamp.frag")).unwrap()
    ).unwrap();

    let lamp_shader_program = render_gl::Program::from_shaders(
        &[lamp_vert_shader, lamp_frag_shader]
    ).unwrap();

    // set up vertex buffer object
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
    }


    let vertices: Vec<f32> = vec![
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
        0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
        0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
        0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
        0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

        0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
        0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
        0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
        0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
        0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
        0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
        0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
        0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
        0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
        0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0];

    let mut vbo2: gl::types::GLuint = 0;
    let mut vao2: gl::types::GLuint = 0;
    let mut ebo:  gl::types::GLuint = 0;
    let mut lamp_vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao2);
        gl::GenVertexArrays(1, &mut lamp_vao);
        gl::GenBuffers(1, &mut vbo2);
        gl::GenBuffers(1, &mut ebo);
    }


    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
    }

    unsafe {


        gl::BindVertexArray(vao2);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::EnableVertexAttribArray(0);
        //vertex co-ords
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        // normal of vertex co-ords
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        gl::BindVertexArray(lamp_vao);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );
    }


    /*
      Set up textures
      */
    let mut view = Mat4::identity();
    let mut projection = Mat4::identity();
    let mut model = Mat4::identity();
    model = rotate(&model, to_radians(45.0), &make_vec3(&[0.0, 1.0, 0.0]));
    projection = perspective(800.0/600.0 as f32, to_radians(45.0), 0.1, 100.0);
    let mut event_pump = sdl.event_pump().unwrap();


    let op = shader_program.program_load_texture(&String::from("material.diffuse"),
                                        &String::from("container2.png"));
    let op2 = shader_program.program_load_texture(&String::from("material.specular"),
                                        &String::from("container2_specular.png"));
    shader_program.set_used();
    let camera_pos = make_vec3(&[0.0, 0.0, 7.0]);
    let camera_up =  make_vec3(&[0.0, 1.0, 0.0]);
    let camera_front =  make_vec3(&[0.0, 0.0, -1.0]);
    let camera_speed = 0.3;


    let mut cam = Camera::new(&camera_pos, &camera_front, &camera_up, 0.3);
    shader_program.set_textures();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::KeyDown{keycode: k, ..} => {
                    let key_code = k.unwrap();

                    match key_code {
                        sdl2::keyboard::Keycode::W => {
                            cam.update_movement(CameraMovement::Front);
                        }
                        sdl2::keyboard::Keycode::S => {
                            cam.update_movement(CameraMovement::Back);
                        }
                        sdl2::keyboard::Keycode::A => {
                            cam.update_movement(CameraMovement::Left);
                        }
                        sdl2::keyboard::Keycode::D => {
                            cam.update_movement(CameraMovement::Right);
                        }
                        _ => {}
                    }
                }
                _ => {},
            }
        }

        view = cam.look_at();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }

        shader_program.set_used();
        let lightPos = make_vec3(&[4.0, 6.0, 3.0]);
        shader_program.set_uniform_vec3("viewPos", &cam.position());

        shader_program.set_uniform_1f("material.shininess", 32.0);
        shader_program.set_uniform_vec3("light.position", &lightPos);
        shader_program.set_uniform_vec3("light.ambient", &make_vec3(&[0.2, 0.2, 0.2]));
        shader_program.set_uniform_vec3("light.diffuse", &make_vec3(&[0.5, 0.5, 0.5]));
        shader_program.set_uniform_vec3("light.specular", &make_vec3(&[1.0, 1.0, 1.0]));


        shader_program.set_uniform_mat4("view", &view).unwrap();
        shader_program.set_uniform_mat4("perspective", &projection).unwrap();
        shader_program.set_uniform_mat4("model", &model).unwrap();



        unsafe {
            gl::BindVertexArray(vao2);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }

        lamp_shader_program.set_used();
        let mut lamp_model = Mat4::identity();
        lamp_model = scale(&lamp_model, &make_vec3(&[0.3, 0.3, 0.3]));
        lamp_model = translate(&lamp_model, &lightPos);

        lamp_shader_program.set_uniform_mat4("view", &view).unwrap();
        lamp_shader_program.set_uniform_mat4("perspective", &projection).unwrap();
        lamp_shader_program.set_uniform_mat4("model", &lamp_model).unwrap();

        unsafe {
            gl::BindVertexArray(lamp_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }
        window.gl_swap_window();
    }
}
fn to_radians(degrees: f32) -> f32 {
    let base: f32 = pi::<f32>()  / (180 as f32);
    return base * degrees;

}