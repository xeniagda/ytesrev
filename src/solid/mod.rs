use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{Drawable, State, Position};


pub struct Solid {
    pub color: Color
}

impl Solid {
    pub fn new_sdl2(color: Color) -> Solid {
        Solid {
            color
        }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Solid {
        Solid {
            color: Color::RGBA(r, g, b, a)
        }
    }
}

impl Drawable for Solid {
    fn content(&self) -> Vec<&dyn Drawable> { vec![] }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![] }

    fn step(&mut self) {
    }

    fn state(&self) -> State {
        State::Final
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &Position) {
        match position {
            Position::Rect(r) => {
                canvas.set_draw_color(self.color);
                canvas.fill_rect(*r).expect("can't draw");
            }
            _ => {}
        }
    }
}
