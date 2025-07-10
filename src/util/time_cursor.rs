/// Simple cursor to keep track of a time-of-day in minutes, with convenience methods
/// specific to the guidebook-plan application.
///
#[derive(Debug, Clone)]
pub struct TimeCursor {
    pub cursor: u32,
}

impl TimeCursor {
    pub fn new(start: u32) -> Self {
        Self { cursor: start }
    }

    pub fn set(&mut self, minutes: u32) {
        self.cursor = minutes;
    }

    pub fn add(&mut self, duration: u32) {
        self.cursor += duration;
    }

    pub fn minutes_remaining(&self, now: u32, duration: u32) -> u32 {
        let elapsed = if now > self.cursor {
            now - self.cursor
        } else {
            0
        };
        duration.saturating_sub(elapsed)
    }

    pub fn pretty(&self) -> String {
        let hours = self.cursor / 60;
        let minutes = self.cursor % 60;
        format!("{:02}:{:02}", hours, minutes)
    }
}
