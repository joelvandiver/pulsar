use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn banner() -> String {
    pulsar_core::banner::banner()
}
