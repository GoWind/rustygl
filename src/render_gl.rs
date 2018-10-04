use gl;
use std::ptr;
use std::ffi::{CStr, CString};

use gl::types::*;

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


