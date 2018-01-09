#[macro_use] extern crate glium;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate cgmath;

mod core;
use core::Core;
mod gl;

fn main() {
    let mut core = Core::initialize();
    core.mainloop();
}
