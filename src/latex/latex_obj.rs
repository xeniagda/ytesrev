extern crate sdl2;

use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect};
use image::{PngImage, ImageContainer};
use drawable::{Drawable, Position};
use super::render::{register_equation, read_image, LatexIdx};


pub struct LatexObj {
    pub inner: Option<PngImage>,
    pub id: Option<LatexIdx>,
    pub expr: &'static str,
    pub is_text: bool,
}

impl ImageContainer for LatexObj {
    fn get_data(&self) -> &Vec<u8> {
        if let Some(ref inner) = self.inner {
            inner.get_data()
        } else {
            panic!("Use of imagecontainer on unloaded LatexObj");
        }
    }
    fn get_data_mut(&mut self) -> &mut Vec<u8> {
        if let Some(ref mut inner) = self.inner {
            inner.get_data_mut()
        } else {
            panic!("Use of imagecontainer on unloaded LatexObj");
        }
    }
    fn into_data(self) -> Vec<u8> {
        if let Some(inner) = self.inner {
            inner.into_data()
        } else {
            panic!("Use of imagecontainer on unloaded LatexObj");
        }
    }
    fn width(&self)    -> usize {
        if let Some(ref inner) = self.inner {
            inner.width()
        } else {
            panic!("Use of imagecontainer on unloaded LatexObj");
        }
    }
    fn height(&self)   -> usize {
        if let Some(ref inner) = self.inner {
            inner.height()
        } else {
            panic!("Use of imagecontainer on unloaded LatexObj");
        }
    }
}

impl LatexObj {
    pub fn math(expr: &'static str) -> LatexObj {
        LatexObj {
            inner: None,
            id: None,
            expr: expr,
            is_text: false,
        }
    }

    pub fn text(expr: &'static str) -> LatexObj {
        LatexObj {
            inner: None,
            id: None,
            expr: expr,
            is_text: true,
        }
    }
}

impl Drawable for LatexObj {
    fn content(&self) -> Vec<&dyn Drawable> {
        if let Some(ref inner) = self.inner {
            vec![ inner ]
        } else {
            vec![ ]
        }
    }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        if let Some(ref mut inner) = self.inner {
            vec![ inner ]
        } else {
            vec![ ]
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, position: &Position) {
        if let Some(ref img) = self.inner {
            img.draw(canvas, position);
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 255));
            match position {
                Position::TopLeftCorner(point) => {
                    canvas.fill_rect(Rect::new(point.x, point.y, 100, 100)).expect("Can't draw");
                }
                Position::Center(point) => {
                    canvas.fill_rect(Rect::new(point.x - 50, point.y - 50, 100, 100)).expect("Can't draw");
                }
                Position::Rect(rect) => {
                    canvas.fill_rect(*rect).expect("Can't draw");
                }
            }
        }
    }

    fn register(&mut self) {
        self.id = Some(register_equation(self.expr, self.is_text));
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
