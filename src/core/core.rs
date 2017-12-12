extern crate glium;
use glium::Surface;

use std::thread;
use std::time::Duration;
use core::window::Window;
use core::window::InputReturn;
use core::stats::Stats;
use gl;

pub struct Core {
    window: Window,
    stats: Stats,
}

impl Core {

    pub fn initialize() -> Core {
        Core {
            window: Window::new(),
            stats: Stats::new(),
        }
    }

    pub fn mainloop(&mut self) {

        let mut last_time = self.stats.millis_elapsed();

        let debug_program = self.window.with_display(gl::base::compile_debug_program).expect("Could not compile default program!");
        let vertices = self.window.with_display(gl::base::make_triangle).expect("Failed making triangle");
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);



        loop {

            let mut current_time = self.stats.millis_elapsed();
            
            if current_time - last_time < 10.0 {
                thread::sleep(Duration::from_millis(10));
                current_time = self.stats.millis_elapsed();
            }
            let delta = current_time - last_time;

            match self.window.get_input() {
                InputReturn::Shutdown => {
                    println!("-> Shutdown");
                    return;
                },
                InputReturn::Menu => {
                    println!("-> Menu");
                },
                InputReturn::Input(input_set) => {
                    //println!("{:?}", input_set.direction);
                },
            }

            last_time = current_time;

            //self.window.display(&gl::base::gl_clear);
            // client could define a list of closure pointers like so which can be passed down
            // but I will implement some debug things first
            self.window.display(&| frame | {
                frame.clear_color(1.0, 1.0, 1.0, 1.0);
                frame.draw(&vertices, &indices, &debug_program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            });
        }
    }
}
