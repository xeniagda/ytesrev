extern crate sdl2;

use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect, rect::Point};
use image::PngImage;
use loadable::Loadable;
use scene::Drawable;
use super::render::{register_equation, read_image, LatexIdx};


pub struct LatexObj {
    pub inner: Option<PngImage>,
    pub id: Option<LatexIdx>,
    pub expr: &'static str,
}

impl LatexObj {
    pub fn new(expr: &'static str) -> LatexObj {
        LatexObj {
            inner: None,
            id: None,
            expr: expr,
        }
    }
}

impl Loadable for LatexObj {
    fn register(&mut self) {
        self.id = Some(register_equation(self.expr));
    }

    fn load(&mut self) {
        if let Some(id) = self.id.take() {
            match read_image(id) {
                Ok(image) => {
                    self.inner = Some(image);
                }
                Err(e) => {
                    eprintln!("Couldn't load image for expression `{}`: {:?}", self.expr, e);
                }
            }
        } else {
            eprintln!("Wrong loading order!");
        }
    }
}

impl Drawable for LatexObj {
    fn draw(&self, canvas: &mut Canvas<Window>, position: &Point) {
        if let Some(ref img) = self.inner {
            img.draw(canvas, position);
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 255));
            canvas.fill_rect(Rect::new(position.x, position.y, 100, 100)).expect("Can't draw");
        }
    }
}
