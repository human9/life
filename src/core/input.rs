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

    pub fn get_binding(&self, key: KeyboardInput) -> Option<&E> {
        self.bindings.get(&key.scancode)
    }

    pub fn is_bound(&self, key: KeyboardInput) -> bool {
        self.bindings.contains_key(&key.scancode)
    }
}

#[derive(Debug)]
pub enum DefaultBindings {
    yes,
    no,
    up,
    down,
    rleft,
    right,
}
