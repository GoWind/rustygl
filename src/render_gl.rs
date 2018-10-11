use gl;
use std::ptr;
use std::ffi::{CStr, CString};
extern crate nalgebra_glm as glm;

use gl::types::*;
extern crate image;

pub struct Shader {
id: gl::types::GLuint
}


impl Shader {
    fn from_source(source: & CStr, kind: gl::types::GLenum) -> Result < Shader, String > {
        let id = shader_from_source(source, kind) ?;
        Ok(Shader{id})
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader( self.id)
        }
    }
}



pub fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind)};
        unsafe {
            gl::ShaderSource(id, 1, & source.as_ptr(), ptr::null());
            gl::CompileShader(id);
        }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, & mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, & mut len);
        }
        let error: CString = create_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                ptr::null_mut(),
                error.as_ptr() as * mut gl::types::GLchar);
        }
        return Err(error.to_string_lossy().into_owned());

    } else {
        Ok(id)
    }
}

fn create_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec < u8 > = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer)}

}

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram()};
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id());}
        }

        unsafe { gl::LinkProgram(program_id); }


        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_cstring_with_len(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }
        for shader in shaders {
            unsafe {gl::DetachShader(program_id, shader.id());}
        }
        Ok(Program {id: program_id})
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn set_uniform_mat4(&self, mat_name: &str, mat: &glm::Mat4) -> Option<i32> {
        let mut mat_name = CString::new(mat_name).unwrap();
        let mut mat_loc;
        unsafe {
            mat_loc = gl::GetUniformLocation(self.id(), mat_name.as_ptr() as *const i8);
        }
        if mat_loc == -1 {
            None
        } else {
            unsafe {
                gl::UniformMatrix4fv(mat_loc, 1, gl::FALSE, mat.as_slice().as_ptr());
            }
            Some(mat_loc)
        }

    }

    pub fn set_uniform_vec4(&self, vec_name: &str, vec: &glm::Vec4) -> Option<i32> {

        let mut vec_name = CString::new(vec_name).unwrap();
        let mut vec_loc;
        unsafe {
            vec_loc = gl::GetUniformLocation(self.id(), vec_name.as_ptr() as *const i8);
        }

        if vec_loc == -1 {
            None
        } else {
            unsafe {
                gl::Uniform4fv(vec_loc, 1, glm::value_ptr(vec).as_ptr());
                Some(vec_loc)
            }
        }
    }

    // right now, I am supporting only one texture. Need to figure out how to add multiple textures
    pub fn set_texture(&self, image_path: &String) -> Option<u32> {
        let border_colors: Vec<f32> = vec![0.0, 1.0, 0.0, 1.0];

        let mut tex: u32 = 0;
        unsafe {
            gl::GenTextures(1,  &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, border_colors.as_ptr());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	// set texture wrapping to GL_REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
        }
        let (image_vec, width, height) = load_jpeg_image(image_path).unwrap();
        let image_bytes:&[u8] = &image_vec[..];
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB8 as gl::types::GLint,
                           width as i32, height as i32,
                           0,
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           image_bytes.as_ptr() as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Some(tex)
    }
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub fn load_png_image(filename: &String) -> Result<(Vec<u8>, u32, u32), String> {
    let mut img = image::open(filename).unwrap();
    let data  = img.as_rgba8().unwrap();
    Ok((data.clone().into_raw(), data.width(), data.height()))
}

pub fn load_jpeg_image(filename: &String) -> Result<(Vec<u8>, u32, u32), String> {
    let mut img = image::open(filename).unwrap();
    let data  = img.as_rgb8().unwrap();
    if data.len() == 0 {
        return Err(String::from(format!("{}: empty jpeg image",filename)))
    }
    Ok((data.clone().into_raw(), data.width(), data.height()))
}


pub fn load_cube_vertices() -> Vec<f32> {
    vec![-0.5, -0.5, -0.5, 0.0, 0.0,
         0.5, -0.5, -0.5, 1.0, 0.0,
         0.5, 0.5, -0.5, 1.0, 1.0,
         0.5, 0.5, -0.5, 1.0, 1.0,
         -0.5, 0.5, -0.5, 0.0, 1.0,
         -0.5, -0.5, -0.5, 0.0, 0.0,
         -0.5, -0.5, 0.5, 0.0, 0.0,
         0.5, -0.5, 0.5, 1.0, 0.0,
         0.5, 0.5, 0.5, 1.0, 1.0,
         0.5, 0.5, 0.5, 1.0, 1.0,
         -0.5, 0.5, 0.5, 0.0, 1.0,
         -0.5, -0.5, 0.5, 0.0, 0.0,
         -0.5, 0.5, 0.5, 1.0, 0.0,
         -0.5, 0.5, -0.5, 1.0, 1.0,
         -0.5, -0.5, -0.5, 0.0, 1.0,
         -0.5, -0.5, -0.5, 0.0, 1.0,
         -0.5, -0.5, 0.5, 0.0, 0.0,
         -0.5, 0.5, 0.5, 1.0, 0.0,
         0.5, 0.5, 0.5, 1.0, 0.0,
         0.5, 0.5, -0.5, 1.0, 1.0,
         0.5, -0.5, -0.5, 0.0, 1.0,
         0.5, -0.5, -0.5, 0.0, 1.0,
         0.5, -0.5, 0.5, 0.0, 0.0,
         0.5, 0.5, 0.5, 1.0, 0.0,
         -0.5, -0.5, -0.5, 0.0, 1.0,
         0.5, -0.5, -0.5, 1.0, 1.0,
         0.5, -0.5, 0.5, 1.0, 0.0,
         0.5, -0.5, 0.5, 1.0, 0.0,
         -0.5, -0.5, 0.5, 0.0, 0.0,
         -0.5, -0.5, -0.5, 0.0, 1.0,
         -0.5, 0.5, -0.5, 0.0, 1.0,
         0.5, 0.5, -0.5, 1.0, 1.0,
         0.5, 0.5, 0.5, 1.0, 0.0,
         0.5, 0.5, 0.5, 1.0, 0.0,
         -0.5, 0.5, 0.5, 0.0, 0.0,
         -0.5, 0.5, -0.5, 0.0, 1.0]
}

pub fn set_texture(filename: &String) -> GLuint {
    let mut k : gl::types::GLuint = 0;
    let image = load_jpeg_image(filename).unwrap();
    let img_bytes = &image.0[..];
    let width = image.1;
    let height = image.2;
    let border_colors: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0];

    unsafe {
        gl::GenTextures(1, &mut k);
        gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, border_colors.as_ptr());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
        gl::BindTexture(gl::TEXTURE_2D, k);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB8 as gl::types::GLint, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, img_bytes.as_ptr() as *const gl::types::GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    k

}