extern crate glium;
extern crate serde_json;
use std::collections::HashMap;
use std::fmt::Debug;
use serde::Serialize;
use glium::glutin::{DeviceEvent, WindowEvent, Event, ElementState, VirtualKeyCode, KeyboardInput};

pub struct KeyBinder<E: Debug + Serialize> {
    pub bindings: HashMap<u32, E>,
}

impl<E: Debug + Serialize> KeyBinder<E> {
    pub fn new() -> KeyBinder<E> {
        KeyBinder {
            bindings: HashMap::new(),
        }
    }

    pub fn bind_key(&mut self, key: KeyboardInput, e: E) {
        if key.state != ElementState::Pressed {
            return
        }
        if self.is_bound(key) {
            println!("{} -> {:?} already bound", key.scancode, self.get_binding(key));
        }
        self.bindings.insert(key.scancode, e); 
        let serialized = serde_json::to_string_pretty(&self.bindings).unwrap();
        println!("Keymap:\n{}", serialized);
    }

    pub fn get_binding(&self, key: KeyboardInput) -> Option<&E> {
        self.bindings.get(&key.scancode)
    }

    pub fn is_bound(&self, key: KeyboardInput) -> bool {
        self.bindings.contains_key(&key.scancode)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DefaultBindings {
    yes,
    no,
    up,
    down,
    left,
    right,
}
