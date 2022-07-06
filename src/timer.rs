use std::time;

pub struct Timer {
    checked_time: time::Instant,
    last_update_tick: time::Instant,
    last_draw_tick: time::Instant,
    update_freq: time::Duration,
    draw_freq: time::Duration,
}

impl Timer {
    pub fn new(render_speed: f64, draw_speed: f64) -> Timer {
        Timer {
            checked_time: time::Instant::now(),
            last_update_tick: time::Instant::now(),
            last_draw_tick: time::Instant::now(),
            update_freq: time::Duration::from_nanos((1_000_000_000.0 / render_speed) as u64),
            draw_freq: time::Duration::from_nanos((1_000_000_000.0 / draw_speed) as u64),
        }
    }

    pub fn time_check(&mut self) {
        self.checked_time = time::Instant::now();
    }

    pub fn render_tick(&mut self) -> bool {
        if self.checked_time - self.last_update_tick >= self.update_freq {
            self.last_update_tick = self.checked_time;
            true
        } else {
            false
        }
    }

    pub fn draw_tick(&mut self) -> bool {
        if self.checked_time - self.last_draw_tick >= self.update_freq {
            self.last_draw_tick = self.checked_time;
            true
        } else {
            false
        }
    }

    pub fn interpolation_factor(&self) -> f32 {
        (self.checked_time - self.last_update_tick).as_secs_f32() / self.update_freq.as_secs_f32()
    }
}