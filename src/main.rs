extern crate life;
extern crate sifter;
extern crate petgraph;
extern crate glium;
extern crate cgmath;
use std::env;
use cgmath::Matrix4;
use life::*;
use sifter::*;

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use glium::glutin::{ElementState, MouseButton};
use glium::Surface;
use life::core::Core;
use life::core::window::Handler;

fn main() {
    let mut core = Core::initialize();

    // let life core handle the mainloop

    let mut args = env::args();
    args.next(); // consume first useless arg
    let filename = match args.next() {
        Some(arg) => arg,
        None => panic!("Fuck!"),
    };

    let graph = sif_to_petgraph(&read_file(&filename).unwrap());
    
    let mut isdown = false;
    let (mut m_x ,mut m_y) = (0., 0.);


    let mut projection: Matrix4<f32> = Matrix4::from(cgmath::Ortho {
        left: 0.0,
        right: 800.0,
        bottom: 0.0,
        top: 600.0,
        near: -1.0,
        far: 1.0 });

    let mut handler = Handler::new();
    
    handler.set_window_mousemove_cb(|x, y| {
        m_x = x;
        m_y = y;
    });
    handler.set_mouseclick_cb(|button, state| {
        if button == MouseButton::Left {
            match state {
                Pressed => isdown = true,
                Released => isdown = false,
            }
        }
    });
    let vertices = core.window.with_display(gl::base::make_triangle).expect("Failed making a triangle!");
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    // mainloops should have an exit condition... that way we can do more mainloops afterwards...
    // maybe
    
    core.mainloop(&mut handler, |frame, delta, matrix| {

        //
        frame.clear_color(1.0, 0.0, 0.0, 0.0);
    });

}
