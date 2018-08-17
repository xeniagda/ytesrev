//! A solid square of color

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{DrawSettings, Drawable, Position, State};

/// A solid, fillid with a color
pub struct Solid {
    /// The color
    pub color: Color,
    shown: bool,
}

impl Solid {
    /// Create a Solid from an sdl2 color
    pub fn new_sdl2(color: Color) -> Solid {
        Solid { color, shown: true }
    }

    /// Create a Solid from a rgba values
    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Solid {
        Solid {
            color: Color::RGBA(r, g, b, a),
            shown: true,
        }
    }
}

impl Drawable for Solid {
    fn content(&self) -> Vec<&dyn Drawable> {
        vec![]
    }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![]
    }

    fn step(&mut self) {
        self.shown = false;
    }

    fn state(&self) -> State {
        if self.shown {
            State::Final
        } else {
            State::Hidden
        }
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &Position, _settings: DrawSettings) {
        match position {
            Position::Rect(r) => {
                canvas.set_draw_color(self.color);
                canvas.fill_rect(*r).expect("can't draw");
            }
            _ => {}
        }
    }
}
