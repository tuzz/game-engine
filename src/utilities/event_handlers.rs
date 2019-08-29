use wasm_bindgen::{prelude::*, JsCast};
use js_sys::Function;

pub fn register_handler<H: Fn(JsValue) + 'static, R: Fn(&Function)>(handler: H, register: R) {
    let boxed = Box::new(handler) as Box<dyn Fn(_)>;
    let closure = Closure::wrap(boxed);
    let js_func = closure.as_ref().unchecked_ref();

    register(js_func);
    closure.forget();
}

pub fn single_use_handler<H: FnOnce(JsValue) + 'static, R: Fn(&Function)>(handler: H, register: R) {
    let closure = Closure::once_into_js(handler);
    let js_func = closure.as_ref().unchecked_ref();

    register(js_func);
}
