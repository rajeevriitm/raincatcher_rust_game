// use crate::model::Draw;
use model::*;
use wasm_bindgen::prelude::*;
mod model;
use model::{RainDrop, World};
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
const DROP_VELOCITY: f64 = 12.0;
use std::time::Instant;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let world = World::new("game-of-drops");
    let mut drop = RainDrop::new();
    let bucket = Bucket::new(&world);
    let timer = Instant::now();
    drop.draw(&world);
    bucket.draw(&world);
    loop {
        drop.move_to_point(drop.centre_y + DROP_VELOCITY * (timer.elapsed().as_secs() as f64));
        drop.draw(&world);
    }

    Ok(())
}
