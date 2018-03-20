use core::window::Window;

pub struct Core {
    pub window: Window,
}

pub enum Status {
    Complete,
    Closed,
}

impl Core {

    pub fn initialize() -> Core {
        Core {
            window: Window::new(),

        }
    }
}
