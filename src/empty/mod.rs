//! The empty object

use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{DrawSettings, Drawable, Position, State};

/// An object that contains nothing and doesn't display anything.
///
/// This is not very useful on it's own, but when used together with a [`WithSize`]-wrapper, it
/// can act as a breaker in a [`Stack`].
///
/// [`WithSize`]: ../withsize/struct.WithSize.html
/// [`Stack`]: ../layout/stack/struct.Stack.html
pub struct Empty;

impl Drawable for Empty {
    fn content(&self) -> Vec<&dyn Drawable> {
        vec![]
    }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![]
    }

    fn step(&mut self) {}

    fn state(&self) -> State {
        State::Hidden
    }

    fn draw(
        &mut self,
        _canvas: &mut Canvas<Window>,
        _position: &Position,
        _settings: DrawSettings,
    ) {
    }
}
