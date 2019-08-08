use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
#[derive(Debug)]
struct World {
    canvas: HtmlCanvasElement,
}
impl World {
    fn new(selector: &str) -> World {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(selector).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        World { canvas }
    }
}
#[derive(Debug)]
struct Drop {
    radius: u32,
    centre_x: u32,
    centre_y: u32,
}
#[derive(Debug)]
struct Bomb {
    radius: u32,
    centre_x: u32,
    centre_y: u32,
}
#[derive(Debug)]
struct Bucket {
    centre_x: u32,
    centre_y: u32,
}
trait Draw {
    fn draw(&self);
}
