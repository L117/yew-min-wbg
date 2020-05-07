#![recursion_limit = "512"]

mod components;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();
    web_logger::init();
    yew::start_app::<components::App>();
    Ok(())
}
