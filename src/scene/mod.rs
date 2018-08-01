extern crate sdl2;

use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum Action {
    Continue,
    Exit
}

pub trait Drawable {
    fn update(&mut self, _dt: f64) {}
    fn draw(&self, _canvas: &mut Canvas<Window>);
}

pub trait Scene {
    fn update(&mut self, _dt: f64) -> Action;
    fn event(&mut self, _event: Event) -> Action;
    fn draw(&self, canvas: &mut Canvas<Window>);
}

pub struct DrawableWrapper<T: Drawable>(pub T);

impl <T: Drawable> Scene for DrawableWrapper<T> {
    fn update(&mut self, _dt: f64) -> Action { Action::Continue }
    fn event(&mut self, _event: Event) -> Action { Action::Continue }
    fn draw(&self, canvas: &mut Canvas<Window>) {
        self.0.draw(canvas);
    }
}
