extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;

use window::YEvent;
use drawable::{Drawable, Position};

#[allow(unused)]
pub enum Action {
    Continue,
    Next,
    Exit
}

pub trait Scene {
    fn update(&mut self, _dt: f64) -> Action;
    fn event(&mut self, _event: YEvent) -> Action;

    fn as_drawable(&self) -> &dyn Drawable;
    fn as_mut_drawable(&mut self) -> &mut dyn Drawable;
}


#[allow(unused)]
pub struct DrawableWrapper<T: Drawable>(pub T);

impl <T: Drawable> Scene for DrawableWrapper<T> {
    fn update(&mut self, dt: f64) -> Action { self.0.update(dt); Action::Continue }
    fn event(&mut self, event: YEvent) -> Action {
        match event {
            YEvent::Step => if self.0.step() { Action::Continue } else { Action::Next }
            YEvent::Next => Action::Next,
            _ => Action::Continue
        }
    }

    fn as_drawable(&self) -> &dyn Drawable { self }
    fn as_mut_drawable(&mut self) -> &mut dyn Drawable { self }
}

impl <T: Drawable> Drawable for DrawableWrapper<T> {
    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &Position) {
        self.0.draw(canvas, position);
    }

    fn step(&mut self) -> bool {
        self.0.step()
    }

    fn content(&self) -> Vec<&dyn Drawable> {
        vec![&self.0]
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![&mut self.0]
    }
}
