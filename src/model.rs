use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use Color::*;
const BUCKET_HEIGHT: f64 = 15.0;
const BUCKET_WIDTH: f64 = 10.0;
const BUCKET_SLAND: f64 = 5.0;
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
    height: f64,
    width: f64,
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
            height: height.into(),
            width: width.into(),
        }
    }
}
#[derive(Debug)]
pub struct RainDrop {
    pub radius: f64,
    pub centre_x: f64,
    pub centre_y: f64,
}
impl RainDrop {
    pub fn new() -> RainDrop {
        RainDrop {
            centre_y: 100.0,
            centre_x: 100.0,
            radius: 10.0,
        }
    }
}
#[derive(Debug)]
pub struct Bomb {
    radius: f64,
    centre_x: f64,
    centre_y: f64,
}
#[derive(Debug)]
pub struct Bucket {
    centre_x: f64,
    centre_y: f64,
}
impl Bucket {
    pub fn new(world: &World) -> Bucket {
        Bucket {
            centre_y: world.height - BUCKET_HEIGHT,
            centre_x: 20.0,
        }
    }
}
pub trait Draw {
    fn draw(&self, world: &World);
    fn move_to_point(&mut self, to_point: f64);
}
impl Draw for RainDrop {
    fn draw(&self, world: &World) {
        let context = get_canvas_context(&world);
        context
            .arc(
                self.centre_x,
                self.centre_y,
                self.radius,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        context.set_fill_style(&Black.get_rgb().into());
        context.fill();
        // context.set_fill_style(2);
    }
    fn move_to_point(&mut self, to_point: f64) {
        self.centre_y = to_point;
    }
}
impl Draw for Bucket {
    fn draw(&self, world: &World) {
        let context = get_canvas_context(&world);
        context.move_to(self.centre_x.into(), self.centre_y);
        context.line_to(self.centre_x + BUCKET_SLAND, self.centre_y + BUCKET_HEIGHT);
        context.line_to(
            self.centre_x + BUCKET_SLAND + BUCKET_WIDTH,
            self.centre_y + BUCKET_HEIGHT,
        );
        context.line_to(
            self.centre_x + 2.0 * BUCKET_SLAND + BUCKET_WIDTH,
            self.centre_y,
        );
        context.set_fill_style(&Black.get_rgb().into());
        context.fill();
    }
    fn move_to_point(&mut self, to_point: f64) {
        self.centre_x = to_point;
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
