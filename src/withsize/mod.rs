//! Make an object have a static size

use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{DrawSettings, Drawable, Position, State, KnownSize};

/// A wrapper that makes an object have a known size
pub struct WithSize<T: Drawable> {
    /// The size of the object
    pub size: (u32, u32),
    /// The inner object
    pub inner: T,
}

impl<T: Drawable> WithSize<T> {
    /// Create a new WithSize wrapper
    pub fn new(size: (u32, u32), inner: T) -> WithSize<T> {
        WithSize { size, inner }
    }
}

impl<T: Drawable> Drawable for WithSize<T> {
    fn content(&self) -> Vec<&dyn Drawable> {
        vec![&self.inner]
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![&mut self.inner]
    }

    fn load(&mut self) {
        self.inner.load();
    }

    fn update(&mut self, dt: f64) {
        self.inner.update(dt);
    }

    fn step(&mut self) {
        self.inner.step()
    }

    fn state(&self) -> State {
        self.inner.state()
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position, settings: DrawSettings) {
        self.inner.draw(canvas, pos, settings);
    }
}

impl<T: Drawable> KnownSize for WithSize<T> {
    fn width(&self) -> usize {
        self.size.0 as usize
    }

    fn height(&self) -> usize {
        self.size.1 as usize
    }
}
