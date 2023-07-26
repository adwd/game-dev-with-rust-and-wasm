use crate::{
    browser,
    engine::{self, *},
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;

use super::{red_hat_boy::*, walk::Walk};

pub enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog::Loading {}
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let json = browser::fetch_json("rhb.json").await?;
                let sheet: Sheet = serde_wasm_bindgen::from_value(json)
                    .expect("Could not convert rhb.json into a Sheet struct");
                let background = engine::load_image("BG.png").await?;
                let stone = engine::load_image("Stone.png").await?;

                let rhb = RedHatBoy::new(sheet, engine::load_image("rhb.png").await?);
                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy: rhb,
                    background: Image::new(background, Point { x: 0, y: 0 }),
                    stone: Image::new(stone, Point { x: 150, y: 546 }),
                })))
            }
            WalkTheDog::Loaded(_) => Err(anyhow!("Error: Game is already initialized")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let WalkTheDog::Loaded(walk) = self {
            if keystate.is_pressed("ArrowRight") {
                walk.boy.run_right();
            }

            if keystate.is_pressed("Space") {
                walk.boy.jump();
            }

            if keystate.is_pressed("ArrowDown") {
                walk.boy.slide();
            }

            walk.boy.update();
        }
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });

        if let WalkTheDog::Loaded(walk) = self {
            walk.background.draw(renderer);
            walk.boy.draw(renderer);
            walk.stone.draw(renderer);
        }
    }
}
