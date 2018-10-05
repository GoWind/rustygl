
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

    //vec3 pos vec3 colors vec2 texture coords
    let vertices2: Vec<f32> = vec![
         0.5,  0.5,  0.0,  1.0, 0.0, 0.0, 1.0, 1.0,
         0.5, -0.5,  0.0,  0.0, 1.0, 0.0, 1.0, 0.0,
        -0.5, -0.5,  0.0,  0.0, 0.0, 1.0, 0.0, 0.0,
        -0.5,  0.5,  0.0,  1.0, 1.0, 1.0, 0.0, 1.0
    ];

    let indices: Vec<u32> = vec![0, 1, 3,
                                 1, 2, 3];




    let mut vbo2: gl::types::GLuint = 0;
    let mut vao2: gl::types::GLuint = 0;
    let mut tex0: gl::types::GLuint = 0;
    let mut ebo:  gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao2);
        gl::GenBuffers(1, &mut vbo2);
        gl::GenBuffers(1, &mut ebo);
        gl::GenTextures(1, &mut tex0);
    }


    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 900, 700);
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
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
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


    }

    let image = render_gl::load_image(&String::from("wall.jpg")).unwrap();
    let img_bytes = &image.0[..];
    let width = image.1;
    let height = image.2;

    let border_colors: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0];
    unsafe {
        gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, border_colors.as_ptr());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
        gl::BindTexture(gl::TEXTURE_2D, tex0);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB8 as gl::types::GLint, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, img_bytes.as_ptr() as *const gl::types::GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D); // this generates only base mipmap
        // for generating sub maps , call teximage2d with arg 2 = i for each level i and then call generateMipmap
    }

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
            gl::BindBuffer(gl::TEXTURE_2D, tex0);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }


        shader_program.set_used();
        unsafe {
                gl::BindVertexArray(vao2);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

                gl::BindVertexArray(0);

        }

        window.gl_swap_window();
    }
}