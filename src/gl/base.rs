extern crate glium;
extern crate glium_text;

use glium_text::TextSystem;
use glium_text::FontTexture;
use glium::Surface;
use glium::Vertex;
use glium::index::Index;
use std::error::Error;

use std::path::Path;
use std::fs::File;

#[derive(Copy, Clone)]
pub struct Vertex2D {
    position: [f32; 2],
}

implement_vertex!(Vertex2D, position);


pub fn make_triangle(display: glium::Display) -> Result<glium::VertexBuffer<Vertex2D>, glium::vertex::BufferCreationError> {

    let vertex1 = Vertex2D { position: [-0.5, -0.5] };
    let vertex2 = Vertex2D { position: [ 0.0,  0.5] };
    let vertex3 = Vertex2D { position: [ 0.5, -0.5] };
    let triangle = vec![vertex1, vertex2, vertex3];

    glium::VertexBuffer::new(&display, &triangle)
}


pub fn gl_clear(frame: &mut glium::Frame) {
    frame.clear_color(1.0, 0.0, 0.0, 1.0);
}

//pub fn init_text(display: glium::Display) -> Result<(TextSystem, FontTexture), glium::ProgramCreationError> {
  //  Ok( (TextSystem::new(&display), glium_text::FontTexture::new(&display, File::open(&Path::new("res/font/DroidSans.ttf")).unwrap(), 32).unwrap()) )
//}


pub fn compile_debug_program(display: glium::Display) -> Result<glium::Program, glium::ProgramCreationError> {

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.0, 0.5, 0.5, 1.0);
        }
    "#;

    glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
}


