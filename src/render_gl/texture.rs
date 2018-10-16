use gl::types::*;

pub enum TexType { RGB, RGBA }
pub struct Texture {
    tex_id: GLuint,
    name: String
}
impl Texture {
    pub fn new(tex_id: GLuint, name: &String) -> Texture {
        Texture {tex_id: tex_id, name: name.clone()}
    }
    pub fn get_id(&self) -> GLuint { self.tex_id}

    pub fn get_name(&self) -> String { self.name.clone()}

}

