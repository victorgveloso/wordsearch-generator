use wasm_bindgen::prelude::*;

mod wasm;
mod generator;
mod model;
mod ui;

#[wasm_bindgen]
pub fn config_console_panic() {
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}