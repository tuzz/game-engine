use web_sys::HtmlCanvasElement;
use std::ops::Deref;

pub struct HtmlCanvas(pub HtmlCanvasElement);

impl Deref for HtmlCanvas {
    type Target = HtmlCanvasElement;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Send for HtmlCanvas {}
unsafe impl Sync for HtmlCanvas {}
