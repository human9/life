extern crate glium;

use glium::Surface;

pub fn gl_clear(frame: &mut glium::Frame) {
    frame.clear_color(1.0, 0.0, 0.0, 1.0);
}
