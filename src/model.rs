use crate::physics::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
// use wasm_bindgen::JsValue;
// use web_sys::HtmlImageElement;
use web_sys::{CanvasRenderingContext2d, Element};
use Color::*;
const BUCKET_HEIGHT: f64 = 30.0;
const BUCKET_BOTTOM_WIDTH: f64 = 30.0;
const BUCKET_SLAND: f64 = 15.0;
pub const DROP_VELOCITY: f64 = 0.8 / 5.0;
const DROP_DISTANCE: f64 = 80.0;
pub const BUCKET_SPEED: f64 = 2.5 / 5.0;
// const DROP_INDENT: f64 = 5.0;
const BUCKET_WIDTH: f64 = BUCKET_BOTTOM_WIDTH + 2.0 * BUCKET_SLAND;
const LOOSE_LIFE_DURATION: f64 = 75.0;
const LIFE_COUNT: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Right = 1,
    Left = -1,
    None = 0,
}

pub struct World {
    pub canvas: CanvasRenderingContext2d,
    raindrops: Vec<RainDrop>,
    pub bucket: Rc<RefCell<Bucket>>,
    height: f64,
    width: f64,
    pub timer: Timer,
    score: u32,
    animations: Vec<AnimateClosure>,
    life: Life,
    pub state: State,
    highscore: u32,
    sound: Sound,
}
impl World {
    pub fn new(selector: &str) -> World {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(selector).unwrap();
        let width = canvas.scroll_width();
        assert!((DROP_DISTANCE / DROP_VELOCITY) * BUCKET_SPEED < width.into());
        let height = canvas.scroll_height();
        let canvas: CanvasRenderingContext2d = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        canvas.set_font("15px Arial");
        canvas.set_text_align("right");
        let bucket = Bucket::new(height as f64 - BUCKET_HEIGHT);
        let bucket = Rc::new(RefCell::new(bucket));
        // let image = HtmlImageElement::new().unwrap();
        // image.set_src("heart1.png");
        let life_node = document.get_element_by_id("life").unwrap();
        let life = Life {
            count: LIFE_COUNT,
            life_node,
        };
        let dropped = web_sys::HtmlAudioElement::new_with_src("dropped.mp3").unwrap();
        let end = web_sys::HtmlAudioElement::new_with_src("end.mp3").unwrap();
        let sound = Sound { dropped, end };
        World {
            canvas,
            height: height.into(),
            width: width.into(),
            raindrops: vec![],
            timer: Timer::new(),
            bucket,
            score: 0,
            animations: vec![],
            life,
            state: State::Active,
            highscore: 0,
            sound,
        }
    }
    pub fn clear_canvas(&self) {
        self.canvas.clear_rect(0.0, 0.0, self.width, self.height);
    }
    pub fn add_new_drop(&mut self) {
        match self.raindrops.last() {
            Some(raindrop) => {
                if raindrop.centre_y > DROP_DISTANCE {
                    let delta_x = self.get_random_distance();
                    // web_sys::console::log_1(&delta_x.into());
                    // random plus or minus
                    let x = if (delta_x as i64) % 2 == 0 {
                        // (raindrop.centre_x + delta_x) % self.width
                        limit_values(
                            raindrop.centre_x + delta_x,
                            raindrop.radius + 20.0,
                            self.width - 20.0,
                        )
                    } else {
                        limit_values(
                            raindrop.centre_x - delta_x,
                            raindrop.radius + 10.0,
                            self.width - 10.0,
                        )
                        // (self.width + raindrop.centre_x - delta_x) % self.width
                    };
                    self.raindrops.push(RainDrop::new(x));
                }
            }
            None => {
                self.raindrops
                    .push(RainDrop::new(self.get_random_distance() % self.width));
            }
        }
    }
    pub fn check_collision(&mut self) {
        if let Some(raindrop) = self.raindrops.first() {
            let y = raindrop.centre_y;
            let x = raindrop.centre_x;
            let bucket_x = self.bucket.borrow().centre_x;
            let drop_is_bomb = raindrop.bomb;
            let catch_condition = y > (self.height - raindrop.radius - BUCKET_HEIGHT)
                && y < (self.height - BUCKET_HEIGHT / 2.0)
                && x > bucket_x
                && x < bucket_x + BUCKET_WIDTH;
            if catch_condition {
                if drop_is_bomb {
                    self.lose_life();
                } else {
                    self.legal_catch();
                }
                self.raindrops.remove(0);
            }
            if y > (self.height) {
                if !drop_is_bomb {
                    self.lose_life();
                }
                self.raindrops.remove(0);
            }
            if self.life.count == 0 {
                self.lost_all_lives();
            }
        }
    }
    fn lost_all_lives(&mut self) {
        self.sound.end.play().unwrap();
        self.state = State::End;
        self.raindrops = vec![];
        if self.score > self.highscore {
            self.highscore = self.score;
        }
        self.reset_game();
    }
    fn legal_catch(&mut self) {
        self.score += 1;
    }
    fn lose_life(&mut self) {
        // let canvas = &self.canvas;
        self.sound.dropped.play().unwrap();
        let width = self.width;
        let height = self.height;
        let closure = move |x: f64, canvas: &CanvasRenderingContext2d| {
            let rgba = format!("rgba(225,0,0,{})", x / LOOSE_LIFE_DURATION);
            canvas.set_fill_style(&rgba.into());
            canvas.fill_rect(0.0, 0.0, width, height);
        };
        let animation = AnimateClosure::new(
            LOOSE_LIFE_DURATION,
            Box::new(closure) as Box<dyn Fn(f64, &CanvasRenderingContext2d)>,
        );
        self.animations.push(animation);
        self.life.remove_life();
    }
    pub fn show_score(&mut self) {
        self.canvas.set_fill_style(&Black.get_rgb().into());
        let score = format!("score: {}", self.score.to_string(),);
        let highscore = format!("highscore: {}", self.highscore.to_string());
        self.canvas
            .fill_text(&score, self.width - 5.0, 15.0)
            .unwrap();
        self.canvas
            .fill_text(&highscore, self.width - 5.0, 30.0)
            .unwrap();
        // crate::log(&score.into());
    }
    fn get_random_distance(&self) -> f64 {
        let value = js_sys::Math::random() * (DROP_DISTANCE / DROP_VELOCITY) * BUCKET_SPEED;
        // web_sys::console::log_1(&value.into());
        f64::min(self.width, value)
    }
    pub fn animation_cycle(&mut self, time: f64) {
        self.clear_canvas();
        self.timer.set_time(time);
        let time_elapsed = self.timer.time_elapsed;
        // web_sys::console::log_1(&(self.raindrops.len() as u32).into());
        for raindrop in self.raindrops.iter_mut() {
            raindrop.move_to_point(raindrop.centre_y + DROP_VELOCITY * time_elapsed);
            raindrop.draw(&self.canvas);
        }
        let mut bucket = self.bucket.borrow_mut();
        let to_point = limit_values(
            bucket.centre_x + bucket.direction as i64 as f64 * BUCKET_SPEED * time_elapsed,
            -BUCKET_WIDTH,
            self.width,
        );
        bucket.move_to_point(to_point);
        bucket.draw(&self.canvas);
        // crate::log(&(self.animations.len() as u32).into());
        // web_sys::console::log_1(&(self.animations.len() as u32).into());
        self.animations.retain(|animation| animation.duration > 0.0);
        // crate::log(&(self.animations.len() as u32).into());
        for animation in self.animations.iter_mut() {
            animation.timer.set_time(time);
            animation.execute(&self.canvas);
        }
    }
    fn reset_game(&mut self) {
        self.life.reset_life();
        self.score = 0;
        self.state = State::Active;
    }
}
#[derive(Debug)]
pub struct RainDrop {
    pub radius: f64,
    pub centre_x: f64,
    pub centre_y: f64,
    bomb: bool,
}
impl RainDrop {
    pub fn new(x: f64) -> RainDrop {
        RainDrop {
            centre_y: -10.0,
            centre_x: x,
            radius: 10.0,
            bomb: js_sys::Math::random() > 0.8,
        }
    }
}
#[derive(Debug)]
pub struct Bucket {
    pub centre_x: f64,
    centre_y: f64,
    pub direction: Direction,
}
impl Bucket {
    pub fn new(centre_y: f64) -> Bucket {
        Bucket {
            centre_y: centre_y,
            centre_x: 20.0,
            direction: Direction::None,
        }
    }
    pub fn move_to_point(&mut self, x: f64) {
        self.centre_x = x;
    }
}
pub trait Draw {
    fn draw(&self, html_canvas: &CanvasRenderingContext2d);
    fn move_to_point(&mut self, to_point: f64);
}
impl Draw for RainDrop {
    fn draw(&self, html_canvas: &CanvasRenderingContext2d) {
        html_canvas.begin_path();
        html_canvas
            .arc(
                self.centre_x,
                self.centre_y,
                self.radius,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        let color = if self.bomb { Red } else { Blue };
        html_canvas.set_fill_style(&color.get_rgb().into());
        html_canvas.fill();
        // context.set_fill_style(2);
    }
    fn move_to_point(&mut self, to_point: f64) {
        self.centre_y = to_point;
    }
}
impl Draw for Bucket {
    fn draw(&self, html_canvas: &CanvasRenderingContext2d) {
        html_canvas.begin_path();
        html_canvas.move_to(self.centre_x.into(), self.centre_y);
        html_canvas.line_to(self.centre_x + BUCKET_SLAND, self.centre_y + BUCKET_HEIGHT);
        html_canvas.line_to(
            self.centre_x + BUCKET_SLAND + BUCKET_BOTTOM_WIDTH,
            self.centre_y + BUCKET_HEIGHT,
        );
        html_canvas.line_to(
            self.centre_x + 2.0 * BUCKET_SLAND + BUCKET_BOTTOM_WIDTH,
            self.centre_y,
        );
        html_canvas.set_fill_style(&Brown.get_rgb().into());
        html_canvas.fill();
    }
    fn move_to_point(&mut self, to_point: f64) {
        self.centre_x = to_point;
    }
}
#[derive(Debug)]
struct Life {
    count: u32,
    life_node: Element,
}
impl Life {
    fn remove_life(&mut self) {
        self.set_class("lost-life", self.count - 1);
        self.count -= 1;
    }
    fn reset_life(&mut self) {
        for index in 0..LIFE_COUNT {
            self.set_class("win-life", index)
        }
        self.count = LIFE_COUNT;
    }
    fn set_class(&self, class_name: &str, index: u32) {
        let element = self.life_node.children().get_with_index(index).unwrap();
        element.set_class_name(class_name);
    }
}
#[derive(Debug, PartialEq)]
pub enum State {
    Active,
    End,
}
#[derive(Debug)]
struct Sound {
    dropped: web_sys::HtmlAudioElement,
    end: web_sys::HtmlAudioElement,
}
fn limit_values(value: f64, min: f64, max: f64) -> f64 {
    // value.rem_euclid(max - min)
    ((value + max - 2.0 * min) % (max - min)) + min
    // f64::max(min, (value + max) % max)
}
