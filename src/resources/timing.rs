pub struct Timing {
    pub updates_per_second: u32,
    pub pause_updates_after: f64,
    pub time_since_update: f64,
}

impl Timing {
    pub fn fixed_update_time(&self) -> f64 {
        1.0 / self.updates_per_second as f64
    }
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            updates_per_second: 5,
            pause_updates_after: 1.0,
            time_since_update: 0.0,
        }
    }
}