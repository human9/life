extern crate life;
extern crate glium;
use life::*;

use glium::glutin::{ElementState, MouseButton};
use life::core::Core;
use life::core::window::Handler;

fn main() {
    let mut core = Core::initialize();

    // let life core handle the mainloop

    let mut isdown = false;
    let (mut m_x ,mut m_y) = (0., 0.);

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
    core.mainloop(&mut handler);

}
