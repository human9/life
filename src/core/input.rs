extern crate glium;
extern crate serde_json;
use std::collections::HashMap;
use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;
use glium::glutin::{DeviceEvent, WindowEvent, Event, ElementState, VirtualKeyCode, KeyboardInput};

pub struct KeyBinder<E: Debug + Serialize + DeserializeOwned> {
    pub bindings: HashMap<u32, E>,
    pub respond: fn(&E),
}

impl KeyBinder<DefaultBindings> {
    pub fn set_macos_default(&mut self) {
        self.bindings.insert(123, DefaultBindings::Left);
        self.bindings.insert(124, DefaultBindings::Right);
        self.bindings.insert(125, DefaultBindings::Down);
        self.bindings.insert(126, DefaultBindings::Up);
        self.bindings.insert(6, DefaultBindings::Yes);
        self.bindings.insert(7, DefaultBindings::No);
        self.bindings.insert(36, DefaultBindings::Menu);
        self.bindings.insert(53, DefaultBindings::Escape);
    }
    pub fn set_windows_default(&mut self) {
        self.bindings.insert(75, DefaultBindings::Left);
        self.bindings.insert(77, DefaultBindings::Right);
        self.bindings.insert(80, DefaultBindings::Down);
        self.bindings.insert(72, DefaultBindings::Up);
        self.bindings.insert(44, DefaultBindings::Yes);
        self.bindings.insert(45, DefaultBindings::No);
        self.bindings.insert(28, DefaultBindings::Menu);
        self.bindings.insert(1, DefaultBindings::Escape);
    }
    pub fn set_linux_default(&mut self) {
        self.bindings.insert(75, DefaultBindings::Left);
        self.bindings.insert(77, DefaultBindings::Right);
        self.bindings.insert(80, DefaultBindings::Down);
        self.bindings.insert(72, DefaultBindings::Up);
        self.bindings.insert(44, DefaultBindings::Yes);
        self.bindings.insert(45, DefaultBindings::No);
        self.bindings.insert(28, DefaultBindings::Menu);
        self.bindings.insert(1, DefaultBindings::Escape);
    }
}

impl<E: Debug + Serialize + DeserializeOwned> KeyBinder<E> {
    pub fn new() -> KeyBinder<E> {
        KeyBinder {
            bindings: HashMap::new(),
            respond: |r|{},
        }
    }

    pub fn from_json(data: &str) -> KeyBinder<E> {
        let h = serde_json::from_str(data).unwrap();
        KeyBinder {
            bindings: h,
            respond: |r|{},
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
    }
    pub fn save_bindings(&mut self) {
        let serialized = serde_json::to_string_pretty(&self.bindings).unwrap();
        println!("Keymap:\n{}", serialized);
    }

    pub fn get_binding(&self, key: KeyboardInput) -> Option<&E> {
        self.bindings.get(&key.scancode)
    }

    pub fn is_bound(&self, key: KeyboardInput) -> bool {
        self.bindings.contains_key(&key.scancode)
    }

    pub fn process_input(&self, key: KeyboardInput) {
        if let Some(binding) = self.get_binding(key) {
            (self.respond)(binding);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DefaultBindings {
    Yes,
    No,
    Up,
    Down,
    Left,
    Right,
    Menu,
    Escape,
}
