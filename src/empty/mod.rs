use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{Drawable, State, Position, DrawSettings};

pub struct Empty;

impl Drawable for Empty {
    fn content(&self) -> Vec<&dyn Drawable> { vec![] }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![] }

    fn step(&mut self) { }

    fn state(&self) -> State {
        State::Hidden
    }

    fn draw(&mut self, _canvas: &mut Canvas<Window>, _position: &Position, _settings: DrawSettings) { }
}
