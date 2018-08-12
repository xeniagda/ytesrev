extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{Drawable, Position, State};

/// I'm not sure why this is needed, but when just storing the dyn Drawable, the compiler complains about
/// Layered::content_mut
///
/// Please ignore this
pub trait Layerable: Drawable {
    fn as_drawable(&self) -> &dyn Drawable;
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable;
}

impl <T: Drawable> Layerable for T {
    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable {
        self
    }
}

pub struct Layered {
    update_seq: bool,
    content: Vec<Box<dyn Layerable>>,
}

impl Layered {
    pub fn new(
        update_seq: bool,
        content: Vec<Box<dyn Layerable>>,
    ) -> Layered {
        Layered {
            content,
            update_seq,
        }
    }
}

impl Drawable for Layered {
    fn content(&self) -> Vec<&dyn Drawable> {
        self.content.iter().map(|x| x.as_drawable()).collect()
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        self.content.iter_mut().map(|x| x.as_drawable_mut()).collect()
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position) {
        for obj in &mut self.content {
            obj.draw(canvas, pos);
        }
    }

    fn step(&mut self) {
        let mut any_stepped = false;
        for item in &mut self.content {
            if item.state() == State::Working {
                item.step();
                any_stepped = true;
                if self.update_seq {
                    return;
                }
            }
        }

        if !any_stepped {
            for item in &mut self.content {
                if item.state() == State::Final {
                    item.step();
                }
            }
        }
    }

    fn state(&self) -> State {
        self.content.iter().map(|x| x.state()).min().unwrap_or(State::Hidden)
    }
}

