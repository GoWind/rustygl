extern crate image;

use std::ptr;
use std::ffi::{CStr, CString};
use crate::glm::*;
use gl;
use gl::types::*;

pub mod texture;
pub mod camera;

use crate::render_gl::texture::Texture;
pub struct Shader {
id: gl::types::GLuint
}

#[derive(PartialEq, Eq)]
enum ImageType { RGB, RGBA}


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
    textures: Vec<Texture>
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
        Ok(Program {id: program_id, textures: Vec::new()})
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn location(&self, name: &str) -> Option<i32> {

        let mat_name = CString::new(name).unwrap();
        let mat_loc;
        unsafe {
            mat_loc = gl::GetUniformLocation(self.id(), mat_name.as_ptr() as *const i8);
        }
        if mat_loc == -1 {
            None
        } else {
            Some(mat_loc)
        }
    }
    pub fn set_uniform_mat4(&self, mat_name: &str, mat: &glm::Mat4) -> Option<i32> {
        let mat_name = CString::new(mat_name).unwrap();
        let mat_loc;
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

    pub fn set_uniform_1f(&self, name: &str, v: f32) -> Option<i32> {
        let cname = CString::new(name).unwrap();
        let loc;
        unsafe {
            loc = gl::GetUniformLocation(self.id(), cname.as_ptr() as *const i8);
        }
        if loc == -1 {
            None
        } else {
            unsafe {
                gl::Uniform1f(loc, v as gl::types::GLfloat);
            }
            Some(loc)
        }
    }

    pub fn set_uniform_vec4(&self, vec_name: &str, vec: &glm::Vec4) -> Option<i32> {

        let vec_name = CString::new(vec_name).unwrap();
        let vec_loc;
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

    pub fn set_uniform_vec3(&self, vec_name: &str, vec: &glm::Vec3) -> Option<i32> {

        let vec_name = CString::new(vec_name).unwrap();
        let vec_loc;
        unsafe {
            vec_loc = gl::GetUniformLocation(self.id(), vec_name.as_ptr() as *const i8);
        }

        if vec_loc == -1 {
            None
        } else {
            unsafe {
                gl::Uniform3fv(vec_loc, 1, glm::value_ptr(vec).as_ptr());
                Some(vec_loc)
            }
        }
    }


    // right now, I am supporting only one texture. Need to figure out how to add multiple textures
    pub fn program_load_texture(&mut self, name: &String, image_path: &String) -> Option<u32> {
        let border_colors: Vec<f32> = vec![0.0, 1.0, 0.0, 1.0];
        let mut tex: u32 = 0;
            unsafe {
                gl::GenTextures(1, &mut tex);
                gl::BindTexture(gl::TEXTURE_2D, tex);
                gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, border_colors.as_ptr());
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);    // set texture wrapping to GL_REPEAT (default wrapping method)
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
            }
            let (image_type, image_data, width, height) = load_image(image_path).unwrap();
            let image_bytes = &image_data[..];
            let (pixel_type, channels) = if image_type == ImageType::RGB {
                (gl::RGB8, gl::RGB)
            }else {
                    (gl::RGBA8, gl::RGBA)
            };
            unsafe {
                gl::TexImage2D(gl::TEXTURE_2D,
                               0,
                               pixel_type as gl::types::GLint,
                               width as i32,
                               height as i32,
                               0,
                               channels,
                               gl::UNSIGNED_BYTE,
                               image_bytes.as_ptr() as *const gl::types::GLvoid);
                gl::GenerateMipmap(gl::TEXTURE_2D);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
            self.textures.push(Texture::new(tex, name));
            return Some(tex);
    }

    pub fn set_textures(&self) {
        for i in 0..self.textures.len() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                let tex_id = self.textures[i].get_id();
                let tex_name = self.textures[i].get_name();
                gl::BindTexture(gl::TEXTURE_2D, tex_id);
                let texture_location = gl::GetUniformLocation(
                    self.id(),
                    CString::new(tex_name.into_bytes()).unwrap().as_ptr());

                gl::Uniform1i(texture_location, i as i32);
            }

        }
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

fn load_image(filename: &String) -> Result<(ImageType, Vec<u8>, u32, u32), String> {
    let k = image::open(filename).unwrap();
    match k  {
        image::DynamicImage::ImageRgb8(ref _im) => {
            let data = k.as_rgb8().unwrap();
            return Ok((ImageType::RGB, data.clone().into_raw(), data.width(), data.height()));
        }
        image::DynamicImage::ImageRgba8(ref _im) => {
            let data = k.as_rgba8().unwrap();
            Ok((ImageType::RGBA, data.clone().into_raw(), data.width(), data.height()))
        }
        _ => {
            Err(String::from(format ! ("unable to load image {}. Unsupported Type", filename)))
        }
    }
}

pub fn load_png_image(filename: &String) -> Result<(Vec<u8>, u32, u32), String> {
    let img = image::open(filename).unwrap();
    let data  = img.as_rgba8().unwrap();
    Ok((data.clone().into_raw(), data.width(), data.height()))
}

pub fn load_jpeg_image(filename: &String) -> Result<(Vec<u8>, u32, u32), String> {
    let img = image::open(filename).unwrap();
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

fn to_radians(degrees: f32) -> f32 {
    let base: f32 = pi::<f32>()  / (180 as f32);
    return base * degrees;
}
