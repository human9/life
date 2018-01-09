extern crate glium;
extern crate cgmath;

use std::error::Error;
use cgmath::num_traits::clamp;

use glium::glutin::{DeviceEvent, WindowEvent, Event, ElementState, VirtualKeyCode, EventsLoop, KeyboardInput};
use cgmath::Vector2;
use core::input::{KeyBinder, DefaultBindings};

pub struct Window {
    display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
    input_set: InputSet, // The last inputset
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
            input_set: InputSet::new(),
        }
    }
    
    pub fn with_display<R, E>(&mut self, f: fn(glium::Display) -> Result<R, E>) -> Result<R, E> {

        f(self.display.clone())
    }

    pub fn display(&mut self, f: &(Fn(&mut glium::Frame))) -> Result<(), Box<Error>> {

        let mut frame = self.display.draw();
        f(&mut frame);

        frame.finish()?;

        Ok( () )
    }

    /// Query the window for input
    pub fn get_input(&mut self, events: &EventsLoop, binder: &mut KeyBinder<DefaultBindings>) -> InputReturn {
        
        let mut shutdown = false;
        let mut menu = false;

        let mut dir = self.input_set.direction;
        let mut pointer = Vector2::new(0.0, 0.0);

        self.events_loop.poll_events(|ev| {

            match ev {
                Event::DeviceEvent { event, .. } => { 
                    match event {
                        DeviceEvent::Key(input) => {
                            binder.bind_key(input, DefaultBindings::yes);
                        },

                        DeviceEvent::Motion { axis, value } => {
                            if axis < 2 {
                                pointer[axis as usize] += value;
                            }
                        },

                        _ => (),
                    }
                },

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        binder.bind_key(input, DefaultBindings::yes);
                    }
                    WindowEvent::Closed => shutdown = true,
                    _ => (),

                },

                _ => (),
            }
        });

        if shutdown {
            return InputReturn::Shutdown;
        }
        if menu {
            return InputReturn::Menu;
        }
        
        for axis in 0..1 {
            dir[axis] = clamp(dir[axis], -1.0, 1.0);
        }

        self.input_set.direction = dir;
        self.input_set.pointer = pointer;

        InputReturn::Input(self.input_set)

    }
}

/// Various mapping-agnostic input values
#[derive(Copy, Clone)]
pub struct InputSet {
    pub direction: Vector2<f32>, // the XY direction being held, from -1 to 1
    pub pointer: Vector2<f64>, // the on-screen location of the pointer
}

impl InputSet {
    pub fn new() -> InputSet {
        InputSet {
            direction: Vector2::new(0.0, 0.0),
            pointer: Vector2::new(0.0, 0.0),
        }
    }
}

/// The possible special cases to return as input
pub enum InputReturn {
    Shutdown,
    Menu,
    Input(InputSet),
}
