pub struct Timing {
    pub updates_per_second: u32,
    pub fixed_update_time: f64,
    pub pause_updates_after: f64,
    pub time_since_update: f64,
}

impl Default for Timing {
    fn default() -> Self {
        let updates_per_second = 5;
        let fixed_update_time = 1.0 / updates_per_second as f64;

        Self {
            updates_per_second,
            fixed_update_time,
            pause_updates_after: 1.0,
            time_since_update: 0.0,
        }
    }
}
