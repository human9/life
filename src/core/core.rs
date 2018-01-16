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
use core::window::Handler;
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
            //config: Config::new(),
        }
    }

    pub fn mainloop(&mut self) {

        let mut last_time = self.stats.millis_elapsed();

        let closure = |e: glium::glutin::KeyboardInput| { println!("{:?}", e.scancode); };
        let events_loop = glium::glutin::EventsLoop::new();
        let mut binder = KeyBinder::new();
        if cfg!(target_os = "linux") {
            binder.set_linux_default();
        }
        if cfg!(target_os = "windows") {
            binder.set_windows_default();
        }
        if cfg!(target_os = "macos") {
            binder.set_macos_default();
        }
        binder.respond = |binding| {
            match binding {
                &Yes => println!("Yes"),
                &No => println!("No"),
                _ => (),
            }
        };

        let debug_program = self.window.with_display(gl::base::compile_debug_program).expect("Could not compile debug program!");
        let vertices = self.window.with_display(gl::base::make_triangle).expect("Failed making a triangle!");
        let text_drawer = gl::base::init_text(self.window.clone_display(), 24).expect("Failed to initialize text rendering!");
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let mut projection: Matrix4<f32> = Matrix4::from(cgmath::Ortho {
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: -1.0,
            far: 1.0 });
        
        //let mut view: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
        let mut view = Matrix4::from_scale(24.0);
        let mut mvp = projection * view;
        let uniforms = uniform! { mvp: mvp.as_uniform() };


        let scale: f32 = 24.0;


        let mut line = String::new();
        loop {

            let mut current_time = self.stats.millis_elapsed();
            
            if current_time - last_time < 10.0 {
                thread::sleep(Duration::from_millis(10));
                current_time = self.stats.millis_elapsed();
            }
            let delta = current_time - last_time;

            {
                let mut handler = Handler::new();
                handler.set_resize_cb(|x, y| {

                    projection = Matrix4::from(cgmath::Ortho {
                        left: 0.0,
                        right: x as f32,
                        bottom: 0.0,
                        top: y as f32,
                        near: -1.0,
                        far: 1.0 });
                    view = Matrix4::from_scale(scale);
                    mvp = projection * view;

                });
                handler.set_received_char_cb(|c| {
                    line.push(c);
                });
                self.window.get_input(&events_loop, &mut handler);
            }

            last_time = current_time;

            let mut frame = self.window.clone_display().draw();
            frame.clear_color(1.0, 1.0, 1.0, 1.0);
            frame.draw(&vertices, &indices, &debug_program, &uniforms, &Default::default()).unwrap();
            text_drawer.println(&line, &mut frame, &mvp);
            frame.finish();
        }
    }
}
