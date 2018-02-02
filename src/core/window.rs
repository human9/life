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

    //pub fn set_keypress_cb<CB: 'a + FnMut(KeyboardInput)>(&mut self, c: CB) {
    pub fn display<DF: FnMut(&mut glium::Frame)>(&mut self, f: &mut DF) {
        let mut frame = self.display.draw();
        f(&mut frame);
        frame.finish();
    }

    /// Query the window for input
    pub fn get_input(&mut self, events: &EventsLoop, mut handlers: (&mut Handler, &mut Handler)) {
        
        self.events_loop.poll_events(|ev| {

            let ref mut handler = handlers.0;
            let ref mut ex_handler = handlers.1;
            match ev {
                Event::DeviceEvent { event, .. } => { 
                    match event {
                        DeviceEvent::Key(input) => {
                            ex_handler.key_pressed(input);
                            handler.key_pressed(input);
                        },

                        DeviceEvent::MouseMotion { delta } => {
                            ex_handler.mouse_moved(delta.0, delta.1);
                            handler.mouse_moved(delta.0, delta.1);
                        },

                        _ => (),
                    }
                },

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::ReceivedCharacter(c) => {
                        ex_handler.received_char(c);
                        handler.received_char(c);
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        ex_handler.window_mouse_moved(position.0, position.1);
                        handler.window_mouse_moved(position.0, position.1);
                    },
                    WindowEvent::Resized(x, y) => {
                        ex_handler.resized(x, y);
                        handler.resized(x, y);
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        if cfg!(target_os = "macos") {
                            ex_handler.key_pressed(input);
                            handler.key_pressed(input);
                        }
                    }
                    WindowEvent::Closed => {
                        ex_handler.shutdown(); 
                        handler.shutdown(); 
                    },
                    WindowEvent::MouseInput { button, state, .. } => {
                        ex_handler.mouse_pressed(button, state); 
                        handler.mouse_pressed(button, state); 
                    },
                    _ => (),

                },

                _ => (),
            }
        });
    }
}

pub struct Handler<'a> {
    keypress_cb: Box<'a + FnMut(KeyboardInput)>,
    received_char_cb: Box<'a + FnMut(char)>,
    mousemove_cb: Box<'a + FnMut(f64, f64)>,
    window_mousemove_cb: Box<'a + FnMut(f64, f64)>,
    mouseclick_cb: Box<'a + FnMut(MouseButton, ElementState)>,
    resize_cb: Box<'a + FnMut(u32, u32)>,
    shutdown_cb: Box<'a + FnMut()>,
}

impl<'a> Handler<'a> {

    pub fn new() -> Self {
        Handler {
            keypress_cb: Box::new(|key|{}),
            received_char_cb: Box::new(|c|{}),
            mousemove_cb: Box::new(|x, y|{}),
            window_mousemove_cb: Box::new(|x, y|{}),
            mouseclick_cb: Box::new(|button, state|{}),
            resize_cb: Box::new(|x, y|{}),
            shutdown_cb: Box::new(||{})
        }
    }

    pub fn set_keypress_cb<CB: 'a + FnMut(KeyboardInput)>(&mut self, c: CB) {
        self.keypress_cb = Box::new(c);
    }
    
    pub fn set_mousemove_cb<CB: 'a + FnMut(f64, f64)>(&mut self, c: CB) {
        self.mousemove_cb = Box::new(c);
    }

    pub fn set_window_mousemove_cb<CB: 'a + FnMut(f64, f64)>(&mut self, c: CB) {
        self.window_mousemove_cb = Box::new(c);
    }
    
    pub fn set_mouseclick_cb<CB: 'a + FnMut(MouseButton, ElementState)>(&mut self, c: CB) {
        self.mouseclick_cb = Box::new(c);
    }

    pub fn set_shutdown_cb<CB: 'a + FnMut()>(&mut self, c: CB) {
        self.shutdown_cb = Box::new(c);
    }
    
    pub fn set_resize_cb<CB: 'a + FnMut(u32, u32)>(&mut self, c: CB) {
        self.resize_cb = Box::new(c);
    }
    
    pub fn set_received_char_cb<CB: 'a + FnMut(char)>(&mut self, c: CB) {
        self.received_char_cb = Box::new(c);
    }

    fn resized(&mut self, x: u32, y: u32) {
        (self.resize_cb)(x, y);
    }

    fn shutdown(&mut self) {
        (self.shutdown_cb)();
    }

    fn mouse_moved(&mut self, x: f64, y: f64) {
        (self.mousemove_cb)(x, y);
    }
    
    fn window_mouse_moved(&mut self, x: f64, y: f64) {
        (self.window_mousemove_cb)(x, y);
    }

    fn mouse_pressed(&mut self, button: MouseButton, state: ElementState) {
        (self.mouseclick_cb)(button, state);
    }

    fn key_pressed(&mut self, key: KeyboardInput) {
        (self.keypress_cb)(key);
    }

    fn received_char(&mut self, c: char) {
        (self.received_char_cb)(c);
    }
}

fn simple_callback() {
    println!("hello");
}

