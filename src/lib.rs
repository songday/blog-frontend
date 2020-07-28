// https://github.com/yewstack/yew/blob/master/yew-router/examples/router_component/src/

#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub(crate) mod app;
mod component;
mod model;
mod util;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, project-name!");
}
*/

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::Model>::new().mount_to_body();
}
