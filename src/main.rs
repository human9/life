#[macro_use]
extern crate glium;
extern crate cgmath;

mod core;
use core::Core;
mod gl;

fn main() {
    let mut core = Core::initialize();
    core.mainloop();
}

