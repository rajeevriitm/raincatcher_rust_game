use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use Color::*;
const BUCKET_HEIGHT: i32 = 15;
const BUCKET_WIDTH: i32 = 10;
const BUCKET_SLAND: i32 = 5;
#[derive(Debug)]
enum Color {
    Black,
    White,
    Red,
}
impl Color {
    fn get_rgb(&self) -> &'static str {
        match self {
            Color::Black => "rgb(0,0,0)",
            Color::Red => "rgb(240, 10, 10)",
            Color::White => "rgb(0,0,0,1)",
        }
    }
}

#[derive(Debug)]
pub struct World {
    canvas: HtmlCanvasElement,
    height: i32,
    width: i32,
}
impl World {
    pub fn new(selector: &str) -> World {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(selector).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let width = canvas.scroll_width();
        let height = canvas.scroll_height();
        World {
            canvas,
            height,
            width,
        }
    }
}
#[derive(Debug)]
pub struct RainDrop {
    radius: i32,
    centre_x: i32,
    centre_y: i32,
}
impl RainDrop {
    pub fn new() -> RainDrop {
        RainDrop {
            centre_y: 100,
            centre_x: 100,
            radius: 10,
        }
    }
}
#[derive(Debug)]
pub struct Bomb {
    radius: i32,
    centre_x: i32,
    centre_y: i32,
}
#[derive(Debug)]
pub struct Bucket {
    centre_x: i32,
    centre_y: i32,
}
impl Bucket {
    pub fn new(world: &World) -> Bucket {
        Bucket {
            centre_y: world.height - BUCKET_HEIGHT,
            centre_x: 20,
        }
    }
}
pub trait Draw {
    fn draw(&self, world: &World);
}
impl Draw for RainDrop {
    fn draw(&self, world: &World) {
        let context = get_canvas_context(&world);
        context
            .arc(
                self.centre_x.into(),
                self.centre_y.into(),
                self.radius.into(),
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        context.set_fill_style(&Black.get_rgb().into());
        context.fill();
        // context.set_fill_style(2);
    }
}
impl Draw for Bucket {
    fn draw(&self, world: &World) {
        let context = get_canvas_context(&world);
        context.move_to(self.centre_x.into(), self.centre_y.into());
        context.line_to(
            (self.centre_x + BUCKET_SLAND).into(),
            (self.centre_y + BUCKET_HEIGHT).into(),
        );
        context.line_to(
            (self.centre_x + BUCKET_SLAND + BUCKET_WIDTH).into(),
            (self.centre_y + BUCKET_HEIGHT).into(),
        );
        context.line_to(
            (self.centre_x + 2 * BUCKET_SLAND + BUCKET_WIDTH).into(),
            self.centre_y.into(),
        );
        context.set_fill_style(&Black.get_rgb().into());
        context.fill();
    }
}
fn get_canvas_context(world: &World) -> CanvasRenderingContext2d {
    let context = world
        .canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context.begin_path();
    context
}
