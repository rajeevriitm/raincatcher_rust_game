//todo replace refcell with cell as bucket is copy
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod model;
use model::{Direction, State, World};
mod physics;
// #[wasm_bindgen]
// extern "C" {
//     fn openModal();
// }
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let mut world = World::new("game-of-drops");
    // let mut drop = RainDrop::new(100.0);
    // let bucket = Bucket::new(&world);
    // let mut timer = Timer::new();
    // let i: i32 = drop.centre_y.into();
    // drop.draw(&world.canvas);
    // world.add_new_drop();
    // world.remove_drop();
    let closure_cell = Rc::new(RefCell::new(None));
    let closure_cell_clone = Rc::clone(&closure_cell);
    // let bucket = Rc::new(RefCell::new(bucket));
    let bucket_clone_keydown = Rc::clone(&world.bucket);
    let bucket_clone_keyup = Rc::clone(&world.bucket);
    *closure_cell.borrow_mut() = Some(Closure::wrap(Box::new(move |x: f64| {
        // world.clear_canvas();
        if world.state == State::Active {
            world.add_new_drop();
            world.check_collision();
        }
        world.animation_cycle(x);
        world.show_score();
        // bucket.borrow().draw(&world.canvas);
        // drop.move_to_point(drop.centre_y + DROP_VELOCITY * (timer.time_elapsed()));
        // drop.draw(&world.canvas);
        request_animation_frame(closure_cell_clone.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(_)>));
    request_animation_frame(closure_cell.borrow().as_ref().unwrap());
    let keydown_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if event.key() == "ArrowRight" {
            bucket_clone_keydown.borrow_mut().direction = Direction::Right;
        // bucket_clone.borrow_mut().centre_x += 10.0;
        } else if event.key() == "ArrowLeft" {
            bucket_clone_keydown.borrow_mut().direction = Direction::Left;
            // bucket_clone.borrow_mut().centre_x -= 10.0;
        }
    }) as Box<dyn FnMut(_)>);
    let keyup_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let key = event.key();
        let direction = bucket_clone_keyup.borrow().direction;
        if (key == "ArrowRight" && direction == Direction::Right)
            || (key == "ArrowLeft" && direction == Direction::Left)
        {
            bucket_clone_keyup.borrow_mut().direction = Direction::None;
        }
    }) as Box<dyn FnMut(_)>);
    request_keydown_event(&keydown_closure);
    request_keyup_event(&keyup_closure);
    keydown_closure.forget();
    keyup_closure.forget();
    Ok(())
}
fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
fn request_keydown_event(f: &Closure<dyn FnMut(web_sys::KeyboardEvent)>) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_onkeydown(Some(f.as_ref().unchecked_ref()))
}
fn request_keyup_event(f: &Closure<dyn FnMut(web_sys::KeyboardEvent)>) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_onkeyup(Some(f.as_ref().unchecked_ref()))
}
// fn log(arg: &JsValue) {
//     web_sys::console::log_1(arg);
// }
