use std::ops::Deref;
use web_sys::Window;

pub struct BrowserWindow(pub Window);

impl Deref for BrowserWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Send for BrowserWindow {}
unsafe impl Sync for BrowserWindow {}
