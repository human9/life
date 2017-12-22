extern crate glium;
use std::collections::HashMap;
use glium::glutin::{DeviceEvent, WindowEvent, Event, ElementState, VirtualKeyCode, KeyboardInput};

pub struct KeyBinder<E> {
    pub bindings: HashMap<u32, E>,
}

impl<E> KeyBinder<E> {
    pub fn new() -> KeyBinder<E> {
        KeyBinder {
            bindings: HashMap::new(),
        }
    }

    pub fn bind_key(&mut self, key: KeyboardInput, e: E) {
        self.bindings.insert(key.scancode, e); 
    }
}

pub enum DefaultBindings {
    yes,
    no,
    up,
    down,
    left,
    right,
}
