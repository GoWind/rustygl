
extern crate sdl2;
extern crate gl;

pub mod render_gl;
use std::ptr;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Game", 900, 700)
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

    let vertices: Vec<f32> = vec![
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 0.6, 1.0,
        -0.5, 0.5, 0.0, 1.0, 0.0, 0.0
    ];

    let vertices2: Vec<f32> = vec![
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 1.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.6,
    ];

    let indices: Vec<u32> = vec![0,1,2,2,3,0];

    let mut vbo: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0;
    let mut vbo2: gl::types::GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::GenBuffers(1, &mut ebo);
    }

    unsafe {
        gl::GenBuffers(1, &mut vbo2);
    }


    // set up vertex array object

    let mut vao: gl::types::GLuint = 0;
    let mut vao2: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::GenVertexArrays(1, &mut vao2);
    }


    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.0, 0.0, 0.0, 0.4);
    }

    unsafe {

        //setup vao 1 and vbo1
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() *std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        // unbind vao and vbo before setting up vao2 and vbo2

        //gl::BindVertexArray(vao);
        //gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        //bind vao2 and vbo2
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
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        //unbind vao2 and vb02
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);




    }

    let mut id = 0;
    let sleep_duration = std::time::Duration::from_millis(400);
    let mut event_pump = sdl.event_pump().unwrap();
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


        shader_program.set_used();
        id = (id + 1) % 2;
        println!("id is {}", id);
        unsafe {
            if id == 1 {
                gl::BindVertexArray(vao);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                /*            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
            */
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
                std::thread::sleep(sleep_duration);
            } else {
                println!("drawing triangle");
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
                gl::BindVertexArray(vao2);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                std::thread::sleep(sleep_duration);

            }
        }

        window.gl_swap_window();
    }
}