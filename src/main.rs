
extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use glm::*;

pub mod render_gl;
use std::ptr;

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

    //vec3 pos vec3 colors vec2 texture coords
    let vertices2: Vec<f32> = vec![
         0.5,  0.5,  0.0,  1.0, 1.0,
         0.5, -0.5,  0.0,  1.0, 0.0,
        -0.5, -0.5,  0.0,  0.0, 0.0,
        -0.5,  0.5,  0.0,  0.0, 1.0
    ];

    let indices: Vec<u32> = vec![0, 1, 3,
                                 1, 2, 3];



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

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );

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

    let border_colors: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0];

    let mut view = Mat4::identity();
    let mut projection = Mat4::identity();
    let model = rotate(&Mat4::identity(), friggin_radians(-55.0), &make_vec3(&[1.0, 0.0, 0.0]));
    let view  = translate(&Mat4::identity(), &make_vec3(&[0.0, 0.0, -3.0]));
    projection = perspective(800.0/600.0 as f32, friggin_radians(45.0), 0.1, 100.0);
    println!("projection matrix is {}", projection);
    let t2 = projection * view * model;
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
        }


        unsafe {
            let ident = Mat4::identity();
            let c_string = CString::new("c").unwrap().as_bytes_with_nul().as_ptr();
            let c_loc = gl::GetUniformLocation(shader_program.id(), c_string as *const i8);
            let ck = make_vec4(&vec![1.0, 0.5, 0.5, 0.0]);
            let mut count = 0;
            let active = gl::GetProgramiv(shader_program.id(), gl::ACTIVE_UNIFORMS, &mut count);
            let mut length = 0;
            let mut size =0;
            let mut k = 0 as u32;
            let mut r = CString::new("gokkamokka").unwrap().as_bytes_with_nul().as_ptr();
            let _ = gl::GetActiveUniform(shader_program.id(), 0 as u32, 20, &mut length, &mut size, &mut k, r as *mut i8);


            /*
            let mut pkr = CString::new("perspective").unwrap().as_bytes_with_nul().as_ptr();
            let pers_string = std::ffi::CStr::from_ptr(pkr as *const i8);
            let pers_loc = gl::GetUniformLocation(shader_program.id(), pkr as *const i8);
            gl::UniformMatrix4fv(pers_loc, 1, gl::FALSE, projection.as_slice().as_ptr());
            println!("pers_loc piece of shit {:?}", pers_loc);
            */

            /*
            let mut vkr = CString::new("view").unwrap().as_bytes_with_nul().as_ptr();
            let vkr_string = std::ffi::CStr::from_ptr(vkr as *const i8);
            let vkr_loc = gl::GetUniformLocation(shader_program.id(), vkr as *const i8);
            gl::UniformMatrix4fv(vkr_loc, 1, gl::FALSE, view.as_slice().as_ptr());
            println!("vkr_loc piece of shit {:?}", vkr_loc);
            */



            /*
            let mut r = CString::new("gokkamakkamakkamakka").unwrap().as_bytes_with_nul().as_ptr();
            let _ = gl::GetActiveUniform(shader_program.id(), 1 as u32, 20, &mut length, &mut size, &mut k, r as *mut i8);
            */

            /*
            println!("iden_loc piece of shit {:?}", iden_loc);
            println!("{:?}, len is {}, {}, {}", std::ffi::CStr::from_ptr(r as *const i8), std::ffi::CStr::from_ptr(r as *const i8).to_bytes().len() , size, k);
            */
            let mut mkr = CString::new("model").unwrap();
            let mkr_loc = gl::GetUniformLocation(shader_program.id(), mkr.as_ptr() as *const i8);
            gl::UniformMatrix4fv(mkr_loc, 1, gl::FALSE, model.as_slice().as_ptr());
            println!("mkr_loc piece of shit {:?}", mkr_loc);

            let mut r = CString::new("c").unwrap();
            //let c_string = std::ffi::CStr::from_ptr(r as *const i8);
            let c_loc = gl::GetUniformLocation(shader_program.id(), r.as_ptr() as *const i8);
            gl::Uniform4fv(c_loc, 1, value_ptr(&ck).as_ptr());
            println!("c_loc piece of shit {:?}", c_loc);

        }



        unsafe {
                gl::BindVertexArray(vao2);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                gl::BindVertexArray(0);
        }

        window.gl_swap_window();
    }
}
// I can't belive there isn't a friggin easy fun to do this stupid
// multiplication. Screw pompous, ritualistic libraries full of
// shitty types
fn friggin_radians(degrees: f32) -> f32 {
   let base: f32 = pi::<f32>()  / (180 as f32);
    return base * degrees;

}
