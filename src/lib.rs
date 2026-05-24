use leptos::prelude::*;
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod stdlib;

use crate::app::App;


#[wasm_bindgen(start)]
pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log::info!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App /> }
    });
}
