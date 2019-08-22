## Minimal Rust + WASM Example

An extremely minimal Rust + WASM example that
[works with GitHub pages](http://tuzz.github.io/minimal-rust-wasm).
It demonstrates how to write to the DOM and how to call a JavaScript function
from Rust. Hopefully this will serve as a helpful reference.

## Usage

```sh
$ make setup
$ make build
$ make server
```

If you're not on a Mac, you'll need to install `binaryen` manually. There are
a few more Makefile commands you might find useful (e.g. `build-watch`). For
more examples, check out the
[wasm-bindgen documentation](https://rustwasm.github.io/docs/wasm-bindgen/).
