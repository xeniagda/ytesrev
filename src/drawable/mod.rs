extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::rect::{Point, Rect};
use sdl2::video::Window;

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Position {
    TopLeftCorner(Point),
    Center(Point),
    Rect(Rect),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum State {
    Working,
    Final,
    Hidden,
}

impl Position {
    pub fn into_rect_with_size(self, width: u32, height: u32) -> Rect {
        match self {
            Position::TopLeftCorner(point) => {
                Rect::new(point.x, point.y, width, height)
            }
            Position::Center(point) => {
                Rect::new(point.x - width as i32 / 2, point.y - height as i32 / 2, width, height)
            }
            Position::Rect(rect) => {
                let center_x = rect.x() + rect.width() as i32 / 2;
                let center_y = rect.y() + rect.height() as i32 / 2;
                Rect::new(center_x - width as i32 / 2, center_y - height as i32 / 2, width, height)
            }
        }
    }
}

pub trait Drawable {
    fn content(&self) -> Vec<&dyn Drawable>;
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable>;

    fn register(&mut self) {
        for content in &mut self.content_mut() {
            content.register();
        }
    }
    fn load(&mut self) {
        for content in &mut self.content_mut() {
            content.load();
        }
    }

    fn step(&mut self);
    fn state(&self) -> State;

    fn update(&mut self, dt: f64) {
        for content in &mut self.content_mut() {
            content.update(dt);
        }
    }

    fn draw(&mut self, _canvas: &mut Canvas<Window>, _position: &Position);
}
