extern crate glium;
use glium::Surface;
extern crate cgmath;
extern crate image;
use cgmath::One;
use cgmath::Matrix4;
use cgmath::Point3;
use cgmath::Vector3;
use cgmath::vec3;
use gl::cgtraits::AsUniform;
use std::path::Path;

use std::thread;
use std::time::Duration;
use core::window::Window;
use core::window::Handler;
use core::stats::Stats;
use core::input::KeyBinder;
use core::input::DefaultBindings::*;
use gl;

use conrod;
use support;

pub struct Core {
    pub window: Window,
    stats: Stats,
}

pub enum Status {
    Complete,
    Closed,
}

impl Core {

    pub fn initialize() -> Core {
        Core {
            window: Window::new(),
            stats: Stats::new(),
            //config: Config::new(),
        }
    }

    pub fn mainloop<DF: FnMut(&mut glium::Frame)>(&mut self, mut extern_handler: &mut Handler, mut f: DF) -> Result<Status, ()> {

        let mut last_time = self.stats.millis_elapsed();

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

        let display = self.window.clone_display();

        let mut ui = conrod::UiBuilder::new([800., 600.]).theme(support::theme()).build();
        let ids = support::Ids::new(ui.widget_id_generator());
        let font_path = Path::new("res/font/Anonymous Pro.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        fn load_img(display: &glium::Display, path: &Path) -> glium::texture::Texture2d {
            let rgba_image = image::open(&Path::new(path)).unwrap().to_rgba();
            let image_dimensions = rgba_image.dimensions();
            let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
            let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
            texture
        }
        let path = Path::new("res/img/turtle.png");

        let mut image_map = conrod::image::Map::new();
        let turtle = image_map.insert(load_img(&display, path));

        let mut app = support::DemoApp::new(turtle);
        let mut renderer = support::conrod_backend::Renderer::new(&display).unwrap();


        let scale: f32 = 30.0;
        let debug_program = self.window.with_display(gl::base::compile_debug_program).expect("Could not compile debug program!");
        let vertices = self.window.with_display(gl::base::make_triangle).expect("Failed making a triangle!");
        let text_drawer = gl::base::init_text(self.window.clone_display(), scale as u32).expect("Failed to initialize text rendering!");
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let mut projection: Matrix4<f32> = Matrix4::from(cgmath::Ortho {
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: -1.0,
            far: 1.0 });
        
        //let mut view: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
        let mut view = Matrix4::from_scale(scale);
        let mut lines = Vec::new();
        let mut mvp = projection * view;
        let uniforms = uniform! { mvp: mvp.as_uniform() };

        let mut line = String::new();

        let mut shutdown = false;
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

                });
                handler.set_received_char_cb(|c| {
                    //println!("{}", c.escape_unicode());
                    match c {
                        '\r' => {
                            lines.push(line.clone());
                            line.clear();
                        },
                        '\u{7f}' => { 
                            if cfg!(target_os = "macos") {
                                line.pop();
                            }
                        },
                        '\u{8}' => { 
                            if cfg!(target_os = "linux") {
                                line.pop();
                            }
                        },
                        _ => line.push(c),
                    }
                });
                handler.set_keypress_cb(|key| {

                });
                handler.set_shutdown_cb(|| {
                    shutdown = true;
                });
                self.window.get_input(&events_loop, &ui, (&mut handler, &mut extern_handler));
            }
            view = Matrix4::from_scale(scale);
            mvp = projection * view;

            last_time = current_time;

            support::gui(&mut ui.set_widgets(), &ids, &mut app);


            let mut frame = self.window.clone_display().draw();
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primites, &image_map);
            }
                
            frame.clear_color(0.0, 0.0, 0.0, 1.0);
            f(&mut frame);

            frame.draw(&vertices, &indices, &debug_program, &uniforms, &Default::default()).unwrap();
            text_drawer.println(&line, &mut frame, &mvp);
            &lines.iter().enumerate().for_each(|(i, l)| {
                let v = Matrix4::from_translation(vec3(0.0, (lines.len() as f32 - i as f32) * scale, 0.0)) * Matrix4::from_scale(scale);
                text_drawer.println(&l, &mut frame, &(projection * v));
            });
            frame.finish();

            if shutdown == true {
                println!("See ya!");
                return Ok(Status::Closed)
            }
        }
    Ok(Status::Complete)
    }
}
