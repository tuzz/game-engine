use std::ops::Deref;

use web_sys::Window as Win;

pub struct Window {
    pub window: Win,
}

impl Window {
    pub fn new(window: Win) -> Self {
        Self { window }
    }
}

impl Deref for Window {
    type Target = Win;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
