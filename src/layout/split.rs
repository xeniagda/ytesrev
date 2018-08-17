//! Split a region into two parts

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{DrawSettings, Drawable, Position, State};

use super::Orientation;

/// The order to update the content in
#[allow(missing_docs)]
pub enum UpdateOrder {
    Simultaneous,
    FirstSecond,
    SecondFirst,
}

/// A split at a centain percent
pub struct SplitPrec<T: Drawable, U: Drawable> {
    /// The split raito (must be between 0 and 1)
    pub prec: f64,
    /// The orientation to split
    pub orientation: Orientation,
    /// The stepping order
    pub order: UpdateOrder,
    /// The first content
    pub first: T,
    /// The second content
    pub second: U,
}

impl<T: Drawable, U: Drawable> SplitPrec<T, U> {
    /// Create a new SplitPrec
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

impl<T: Drawable, U: Drawable> Drawable for SplitPrec<T, U> {
    fn content(&self) -> Vec<&dyn Drawable> {
        vec![&self.first, &self.second]
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![&mut self.first, &mut self.second]
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position, settings: DrawSettings) {
        match pos {
            Position::TopLeftCorner(_) | Position::Center(_) => {
                eprintln!("Trying to draw a Splitpane not using a Position::Rect. Please don't");
            }
            Position::Rect(rect) => {
                let (first_rect, second_rect) = match self.orientation {
                    Orientation::Vertical => {
                        let first_height = (rect.height() as f64 * self.prec) as u32;
                        let first_rect = Rect::new(rect.x, rect.y, rect.width(), first_height);
                        let second_rect = Rect::new(
                            rect.x,
                            rect.y + first_height as i32,
                            rect.width(),
                            rect.height() - first_height,
                        );
                        (first_rect, second_rect)
                    }
                    Orientation::Horizontal => {
                        let first_width = (rect.width() as f64 * self.prec) as u32;
                        let first_rect = Rect::new(rect.x, rect.y, first_width, rect.height());
                        let second_rect = Rect::new(
                            rect.x + first_width as i32,
                            rect.y,
                            rect.width() - first_width,
                            rect.height(),
                        );
                        (first_rect, second_rect)
                    }
                };

                self.first
                    .draw(canvas, &Position::Rect(first_rect), settings);
                self.second
                    .draw(canvas, &Position::Rect(second_rect), settings);

                if settings.notes_view {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas.draw_rect(first_rect).expect("Can't draw");
                    canvas.draw_rect(second_rect).expect("Can't draw");
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                    canvas.draw_rect(*rect).expect("Can't draw");
                }
            }
        }
    }

    fn step(&mut self) {
        match self.order {
            UpdateOrder::Simultaneous => {
                if self.first.state() == State::Working {
                    self.first.step();
                }
                if self.second.state() == State::Working {
                    self.second.step();
                }
                if self.first.state() >= State::Final && self.second.state() >= State::Final {
                    self.first.step();
                    self.second.step();
                }
            }
            UpdateOrder::FirstSecond => {
                if self.first.state() == State::Working {
                    self.first.step();
                } else if self.second.state() == State::Working {
                    self.second.step();
                } else {
                    self.first.step();
                    self.second.step();
                }
            }
            UpdateOrder::SecondFirst => {
                if self.second.state() == State::Working {
                    self.second.step();
                } else if self.first.state() == State::Working {
                    self.first.step();
                } else {
                    self.first.step();
                    self.second.step();
                }
            }
        }
    }

    fn state(&self) -> State {
        Ord::min(self.first.state(), self.second.state())
    }
}
