use anyhow::Result;
use engine::GameLoop;
use game::WalkTheDog;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[macro_use]
mod browser;
mod engine;
mod game;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    browser::spawn_local(async move {
        let game = WalkTheDog::new();

        GameLoop::start(game)
            .await
            .expect("Could not start game loop");
    });

    Ok(())
}
