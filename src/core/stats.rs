use std::time::Instant;

pub struct Stats {
    beginning_of_time: Instant,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            beginning_of_time: Instant::now(),
        }
    }

    pub fn millis_elapsed(&self) -> f32 {
        (self.beginning_of_time.elapsed().as_secs() as f32 * 1e3) + (self.beginning_of_time.elapsed().subsec_nanos() as f32 / 1e6)
    }
}

