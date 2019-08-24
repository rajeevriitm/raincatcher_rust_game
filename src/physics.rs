#[derive(Debug)]
struct AnimateClosure<T: FnMut(f64)> {
    duartion: u32,
    closure: T,
    timer: Timer,
}
impl<T: FnMut(f64)> AnimateClosure<T> {
    fn execute(&mut self) {
        let time_elapsed = self.timer.time_elapsed;
        (self.closure)(time_elapsed);
    }
}
#[derive(Debug)]
pub struct Timer {
    start_time: Option<f64>,
    pub time_elapsed: f64,
    ticks: u32,
}
impl Timer {
    pub fn new() -> Timer {
        // let start_time = web_sys::window().unwrap().performance().unwrap().now();
        Timer {
            start_time: None,
            time_elapsed: 0.0,
            ticks: 0,
        }
    }
    pub fn set_time(&mut self, time: f64) {
        if let Some(old_time) = self.start_time {
            self.time_elapsed = (time - old_time) / 5.0;
        }
        self.start_time = Some(time);
        self.ticks += 1;
    }
}
#[derive(Debug)]
pub enum Color {
    Black,
    Brown,
    Red,
    Blue,
}
impl Color {
    pub fn get_rgb(&self) -> &'static str {
        match self {
            Blue => "rgb(0,119,190)",
            Black => "rgb(21,21,60)",
            Brown => "rgb(140,52,52)",
            Red => "rgb(168,0,0)",
        }
    }
}
