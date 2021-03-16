pub mod app;
pub mod canvas;
pub mod consts;
pub mod helpers;
pub mod view;

use seed::prelude::{wasm_bindgen, App};

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", app::init, app::update, view::view);
}
