extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::video::Window;

use window::YEvent;

use loadable::Loadable;

pub enum Action {
    Continue,
    Next,
    Exit
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Position {
    TopLeftCorner(Point),
    Center(Point),
}

pub trait Drawable {
    fn update(&mut self, _dt: f64) {}
    fn draw(&self, _canvas: &mut Canvas<Window>, _position: &Position);
}

pub trait Scene: Loadable {
    fn update(&mut self, _dt: f64) -> Action;
    fn event(&mut self, _event: YEvent) -> Action;
    fn draw(&self, canvas: &mut Canvas<Window>);
}


#[allow(unused)]
pub struct DrawableWrapper<T: Drawable>(pub T);

impl <T: Drawable> Loadable for DrawableWrapper<T> {
    fn register(&mut self) {}
    fn load(&mut self) {}
}

impl <T: Drawable> Scene for DrawableWrapper<T> {
    fn update(&mut self, _dt: f64) -> Action { Action::Continue }
    fn event(&mut self, event: YEvent) -> Action {
        match event {
            YEvent::Next => Action::Next,
            _ => Action::Continue
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        self.0.draw(canvas, &Position::TopLeftCorner(Point::new(0, 0)));
    }
}
