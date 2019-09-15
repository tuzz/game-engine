use wasm_bindgen::prelude::*;

#[allow(unused_macros)]
macro_rules! log {
    ($($t:tt)*) => (crate::utilities::log(&format_args!($($t)*).to_string()))
}

#[allow(unused_macros)]
macro_rules! alert {
    ($($t:tt)*) => (crate::utilities::alert(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = window)]
    pub fn alert(s: &str);
}

