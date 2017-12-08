use std::thread;
use std::time::Duration;
use core::window::Window;
use core::window::InputReturn;
use core::stats::Stats;
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
        }
    }

    pub fn mainloop(&mut self) {

        let mut last_time = self.stats.millis_elapsed();

        loop {

            let mut current_time = self.stats.millis_elapsed();
            
            if current_time - last_time < 10.0 {
                thread::sleep(Duration::from_millis(10));
                current_time = self.stats.millis_elapsed();
            }
            let delta = current_time - last_time;

            match self.window.get_input() {
                InputReturn::Shutdown => {
                    println!("-> Shutdown");
                    return;
                },
                InputReturn::Menu => {
                    println!("-> Menu");
                },
                InputReturn::Input(input_set) => {
                    //println!("{:?}", input_set.direction);
                },
            }

            last_time = current_time;

            self.window.display(gl::base::gl_clear);
        }
    }
}
