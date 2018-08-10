extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{Drawable, Position};

use super::Orientation;

#[allow(unused)]
pub enum UpdateOrder {
    Simultaneous,
    FirstSecond,
    SecondFirst,
}

pub struct SplitPrec<T: Drawable, U: Drawable> {
    pub prec: f64,
    pub orientation: Orientation,
    pub order: UpdateOrder,
    pub first: T,
    pub second: U,
}

impl <T: Drawable, U: Drawable> SplitPrec<T, U> {
    pub fn new(
        prec: f64,
        orientation: Orientation,
        order: UpdateOrder,
        first: T,
        second: U,
    ) -> SplitPrec<T, U> {
        SplitPrec {
            prec,
            orientation,
            order,
            first,
            second,
        }
    }
}

impl <T: Drawable, U: Drawable> Drawable for SplitPrec<T, U> {
    fn content(&self) -> Vec<&dyn Drawable> {
        vec![&self.first, &self.second]
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![&mut self.first, &mut self.second]
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position) {
        match pos {
            Position::TopLeftCorner(_) | Position::Center(_) => {
                eprintln!("Trying to draw a YSplitpane not using a Position::Rect. Please don't");
            }
            Position::Rect(rect) => {
                let (first_rect, second_rect) = match self.orientation {
                    Orientation::Vertical => {
                        let first_height = (rect.height() as f64 * self.prec) as u32;
                        let first_rect = Rect::new(
                            rect.x,
                            rect.y,
                            rect.width(),
                            first_height
                        );
                        let second_rect = Rect::new(
                            rect.x,
                            rect.y + first_height as i32,
                            rect.width(),
                            rect.height() - first_height,
                        );
                        (first_rect, second_rect)
                    }
                    Orientation::Horisontal => {
                        let first_width = (rect.width() as f64 * self.prec) as u32;
                        let first_rect = Rect::new(
                            rect.x,
                            rect.y,
                            first_width,
                            rect.height(),
                        );
                        let second_rect = Rect::new(
                            rect.x + first_width as i32,
                            rect.y,
                            rect.width() - first_width,
                            rect.height(),
                        );
                        (first_rect, second_rect)
                    }
                };

                self.first.draw(canvas, &Position::Rect(first_rect));
                self.second.draw(canvas, &Position::Rect(second_rect));

                if super::DRAW_BOXES {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas.draw_rect(first_rect).expect("Can't draw");
                    canvas.draw_rect(second_rect).expect("Can't draw");
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                    canvas.draw_rect(*rect).expect("Can't draw");
                }
            }
        }
    }

    fn step(&mut self) -> bool {
        match self.order {
            UpdateOrder::Simultaneous => {
                let first_res = self.first.step();
                let second_res = self.second.step();

                first_res && second_res
            }
            UpdateOrder::FirstSecond => {
                if self.first.step() {
                    true
                } else {
                    self.second.step()
                }
            }
            UpdateOrder::SecondFirst => {
                if self.second.step() {
                    true
                } else {
                    self.first.step()
                }
            }
        }
    }
}
