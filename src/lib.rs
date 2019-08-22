use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let p = document.create_element("p").unwrap();

    p.set_inner_html("Hello, world!");
    body.append_child(&p).unwrap();

    log("Hello, console!");
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
