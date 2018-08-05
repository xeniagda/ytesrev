extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::rect::{Point, Rect};
use sdl2::video::Window;

use window::YEvent;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Position {
    TopLeftCorner(Point),
    Center(Point),
    Rect(Rect),
}

pub trait Drawable {
    fn content(&self) -> Vec<&dyn Drawable>;
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable>;

    fn register(&mut self) {
        for ref mut content in self.content_mut() {
            content.register();
        }
    }
    fn load(&mut self) {
        for ref mut content in self.content_mut() {
            content.load();
        }
    }

    fn update(&mut self, dt: f64) {
        for ref mut content in self.content_mut() {
            content.update(dt);
        }
    }

    fn draw(&self, _canvas: &mut Canvas<Window>, _position: &Position);
}
