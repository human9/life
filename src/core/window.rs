extern crate glium;
extern crate cgmath;

use std::error::Error;
use cgmath::num_traits::clamp;

use glium::glutin::{DeviceEvent, WindowEvent, Event, ElementState, VirtualKeyCode, EventsLoop, KeyboardInput, MouseButton, MouseScrollDelta};
use cgmath::Vector2;
use core::input::{KeyBinder, DefaultBindings};

pub struct Window {
    display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
}

impl Window {
    pub fn new() -> Window {
        let events_loop = glium::glutin::EventsLoop::new();

        let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(800, 600)
            .with_title("life");

        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            //.with_multisampling(2)
        ;

        let display = glium::Display::new(window, context, &events_loop).unwrap();

        //display.gl_window().set_cursor_state(glium::glutin::CursorState::Grab).unwrap();
        //display.gl_window().set_cursor(glium::glutin::MouseCursor::NoneCursor);

        Window { 
            display: display,
            events_loop: events_loop,
        }
    }

    pub fn query_size(&self) -> Option<(u32, u32)> {
        self.display.gl_window().get_inner_size()
    }
    
    pub fn with_display<R, E>(&mut self, f: fn(glium::Display) -> Result<R, E>) -> Result<R, E> {

        f(self.display.clone())
    }

    pub fn clone_display(&self) -> glium::Display {
        self.display.clone()
    }

    pub fn display(&mut self, f: &(Fn(&mut glium::Frame))) -> Result<(), Box<Error>> {

        let mut frame = self.display.draw();
        f(&mut frame);

        frame.finish()?;

        Ok( () )
    }

    /// Query the window for input
    pub fn get_input<E: EventHandler>(&mut self, events: &EventsLoop, handler: &E) {
        
        self.events_loop.poll_events(|ev| {

            match ev {
                Event::DeviceEvent { event, .. } => { 
                    match event {
                        DeviceEvent::Key(input) => {
                            handler.key_pressed(input);
                        },

                        DeviceEvent::Motion { axis, value } => {
                            if axis < 2 {
                                //pointer[axis as usize] += value;
                            }
                        },

                        _ => (),
                    }
                },

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(x, y) => {
                        handler.resized(x, y);
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        if cfg!(target_os = "macos") {
                            handler.key_pressed(input);
                        }
                    }
                    WindowEvent::Closed => {
                        handler.shutdown(); 
                    },
                    _ => (),

                },

                _ => (),
            }
        });
    }
}

pub trait EventHandler {
    fn resized(&self, x: u32, y: u32);
    fn shutdown(&self, );
    fn mouse_moved(x: f64, y: f64);
    fn mouse_pressed(button: MouseButton, state: ElementState);
    fn key_pressed(&self, key: KeyboardInput);
}
