// use crate::model::Draw;
use model::*;
use wasm_bindgen::prelude::*;
mod model;
use model::{RainDrop, World};
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let world = World::new("game-of-drops");
    let drop = RainDrop::new();
    drop.draw(&world);
    let bucket = Bucket::new(&world);
    bucket.draw(&world);
    Ok(())
}
