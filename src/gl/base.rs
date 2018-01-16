extern crate glium;
extern crate glium_text_rusttype as glium_text;

use cgmath::Matrix4;
use gl::cgtraits::AsUniform;

use self::glium_text::TextSystem;
use self::glium_text::FontTexture;
use glium::Surface;
use glium::Vertex;
use glium::index::Index;
use std::error::Error;

use std::path::Path;
use std::fs::File;

#[derive(Copy, Clone)]
pub struct Vertex3D {
    position: [f32; 3],
}

implement_vertex!(Vertex3D, position);

pub fn make_triangle(display: glium::Display) -> Result<glium::VertexBuffer<Vertex3D>, glium::vertex::BufferCreationError> {

    let vertex1 = Vertex3D { position: [-0.5, -0.5, 0.0] };
    let vertex2 = Vertex3D { position: [ 0.0,  0.5, 0.0] };
    let vertex3 = Vertex3D { position: [ 0.5, -0.5, 0.0] };
    let triangle = vec![vertex1, vertex2, vertex3];

    glium::VertexBuffer::new(&display, &triangle)
}


pub fn gl_clear(frame: &mut glium::Frame) {
    frame.clear_color(1.0, 0.0, 0.0, 1.0);
}

// todo: dynamic screen size tracking
pub struct TextDrawer {
    system: TextSystem,
    font: FontTexture,
    line: i32,
    fontsize: u32,
}

impl<'a> TextDrawer {
    pub fn println(&self, line: &'a str, frame: &mut glium::Frame, mvp: &Matrix4<f32>) {
        let string = glium_text::TextDisplay::new(&self.system, &self.font, &format!("{}", line));
        glium_text::draw(&string, &self.system, frame, mvp.as_uniform(), (0.0, 0.0, 0.0, 1.0));
    }

}

pub fn init_text(display: glium::Display, fontsize: u32) -> Result<TextDrawer, ()> {
    Ok( TextDrawer {
        system: TextSystem::new(&display),
        font: FontTexture::new(&display, File::open(&Path::new("res/font/Anonymous Pro.ttf")).unwrap(), fontsize, FontTexture::ascii_character_list()).unwrap(),
        fontsize: fontsize,
        line: 0,
    } )
}


pub fn compile_debug_program(display: glium::Display) -> Result<glium::Program, glium::ProgramCreationError> {
    
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        uniform mat4 mvp;

        void main() {
            gl_Position = mvp * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.0, 1.0, 1.0, 1.0);
        }
    "#;

    glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
}


