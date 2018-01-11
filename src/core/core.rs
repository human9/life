extern crate glium;
use glium::Surface;
extern crate cgmath;
use cgmath::One;
use cgmath::Matrix4;
use cgmath::Point3;
use cgmath::Vector3;
use cgmath::vec3;
use gl::cgtraits::AsUniform;

use std::thread;
use std::time::Duration;
use core::window::Window;
use core::window::InputReturn;
use core::stats::Stats;
use core::input::KeyBinder;
use core::input::DefaultBindings::*;
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

        let closure = |e: glium::glutin::KeyboardInput| { println!("{:?}", e.scancode); };
        let events_loop = glium::glutin::EventsLoop::new();
        let mut binder = KeyBinder::new();
        binder.set_macos_default();
        binder.respond = |binding| {
            match binding {
                &Yes => println!("Yes"),
                &No => println!("No"),
                _ => (),
            }
        };

        let debug_program = self.window.with_display(gl::base::compile_debug_program).expect("Could not compile debug program!");
        let vertices = self.window.with_display(gl::base::make_triangle).expect("Failed making a triangle!");
        let text_drawer = self.window.with_display(gl::base::init_text).expect("Failed to initialize text rendering!");
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let projection: Matrix4<f32> = Matrix4::from(cgmath::Ortho {
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: -1.0,
            far: 1.0 });
        
        let mut view: Matrix4<f32> = Matrix4::from_translation(vec3(4.0, 3.0, 0.0));
        view = view * Matrix4::from_scale(24.0);
        let mvp = projection * view;
        let uniforms = uniform! { mvp: mvp.as_uniform() };

        loop {

            let mut current_time = self.stats.millis_elapsed();
            
            if current_time - last_time < 10.0 {
                thread::sleep(Duration::from_millis(10));
                current_time = self.stats.millis_elapsed();
            }
            let delta = current_time - last_time;

            match self.window.get_input(&events_loop, &mut binder) {
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
                frame.draw(&vertices, &indices, &debug_program, &uniforms, &Default::default()).unwrap();
                text_drawer.println("hello world", frame, &mvp);
            });
        }
    }
}
