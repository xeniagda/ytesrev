extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;

use window::YEvent;
use drawable::{Drawable, Position, State, DrawSettings};

#[allow(unused)]
pub enum Action {
    Continue,
    Next,
}

pub trait Scene {
    fn update(&mut self, _dt: f64);
    fn event(&mut self, _event: YEvent);
    fn action(&self) -> Action;

    fn as_drawable(&self) -> &dyn Drawable;
    fn as_mut_drawable(&mut self) -> &mut dyn Drawable;
}


#[allow(unused)]
pub struct DrawableWrapper<T: Drawable>(pub T);

impl <T: Drawable> Scene for DrawableWrapper<T> {
    fn update(&mut self, dt: f64) { self.0.update(dt); }
    fn event(&mut self, event: YEvent) {
        match event {
            YEvent::Step => {
                self.0.step();
            }
            _ => {}
        }
    }

    fn action(&self) -> Action {
        if self.0.state() == State::Hidden {
            Action::Next
        } else {
            Action::Continue
        }
    }

    fn as_drawable(&self) -> &dyn Drawable { self }
    fn as_mut_drawable(&mut self) -> &mut dyn Drawable { self }
}

impl <T: Drawable> Drawable for DrawableWrapper<T> {
    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &Position, settings: DrawSettings) {
        self.0.draw(canvas, position, settings);
    }

    fn step(&mut self) {
        self.0.step();
    }

    fn state(&self) -> State {
        self.0.state()
    }

    fn content(&self) -> Vec<&dyn Drawable> {
        vec![&self.0]
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![&mut self.0]
    }
}
