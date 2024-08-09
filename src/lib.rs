use wasm_bindgen::prelude::*;

// Expose the `greet` function to JavaScript using wasm-bindgen
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    return format!("Hello, {}!", name);
}