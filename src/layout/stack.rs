extern crate sdl2;

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use drawable::{Drawable, Position, State};
use image::KnownSize;
use super::Orientation;

pub enum ElementPositioning {
    TopLeftCornered,
    Centered,
}

pub trait Stackable: Drawable + KnownSize {
    fn as_drawable(    &    self) -> &    dyn Drawable;
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable;
}

impl <T: Drawable + KnownSize> Stackable for T {
    fn as_drawable(&self) -> &dyn Drawable { self }
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable { self }
}

pub struct Stack {
    margin: u32,
    orientation: Orientation,
    positioning: ElementPositioning,
    update_seq: bool,
    content: Vec<Box<dyn Stackable>>,
}

impl Stack {
    pub fn new(
        margin: u32,
        orientation: Orientation,
        positioning: ElementPositioning,
        update_seq: bool,
        content: Vec<Box<dyn Stackable>>,
    ) -> Stack {
        Stack {
            margin,
            orientation,
            positioning,
            update_seq,
            content,
        }
    }
}

impl <'a> Drawable for Stack {
    fn content(&self) -> Vec<&dyn Drawable> {
        self.content.iter().map(|x| x.as_drawable()).collect()
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        self.content.iter_mut().map(|x| x.as_drawable_mut()).collect()
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position) {
        let rect = pos.into_rect_with_size(self.width() as u32, self.height() as u32);
        let corner = rect.top_left();
        if super::DRAW_BOXES {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_rect(rect).expect("Can't draw");
        }

        let (width, height) = (self.width(), self.height());

        match self.orientation {
            Orientation::Vertical => {
                let mut y = corner.y;
                for obj in &mut self.content {
                    let corner =
                        match self.positioning {
                            ElementPositioning::TopLeftCornered => {
                                Point::new(corner.x, y)
                            }
                            ElementPositioning::Centered => {
                                let px = corner.x + width as i32 / 2 - obj.width() as i32 / 2;
                                Point::new(px, y)
                            }
                        };
                    let pos = Position::TopLeftCorner(corner);

                    if super::DRAW_BOXES {
                        canvas.set_draw_color(Color::RGB(255, 0, 0));
                        canvas.draw_rect(
                            pos.into_rect_with_size(obj.width() as u32, obj.height() as u32)
                        ).expect("Can't draw");
                    }

                    obj.draw(canvas, &pos);
                    y += obj.height() as i32 + self.margin as i32;
                }
            }
            Orientation::Horisontal => {
                let mut x = corner.x;
                for obj in &mut self.content {
                    let corner =
                        match self.positioning {
                            ElementPositioning::TopLeftCornered => {
                                Point::new(x, corner.y)
                            }
                            ElementPositioning::Centered => {
                                let py = corner.y + height as i32 / 2 - obj.height() as i32 / 2;
                                Point::new(x, py)
                            }
                        };
                    let pos = Position::TopLeftCorner(corner);

                    if super::DRAW_BOXES {
                        canvas.set_draw_color(Color::RGB(255, 0, 0));
                        canvas.draw_rect(
                            pos.into_rect_with_size(obj.width() as u32, obj.height() as u32)
                        ).expect("Can't draw");
                    }

                    obj.draw(canvas, &pos);
                    x += obj.width() as i32 + self.margin as i32;
                }
            }
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

impl KnownSize for Stack {
    fn width(&self)  -> usize {
        match self.orientation {
            Orientation::Horisontal => {
                let content_size = self.content.iter().map(|x| x.width()).sum::<usize>();
                let margins = self.margin as usize * (self.content.len() - 1);
                content_size + margins
            }
            Orientation::Vertical => {
                self.content.iter().map(|x| x.width()).max().unwrap_or(0)
            }
        }
    }

    fn height(&self) -> usize {
        match self.orientation {
            Orientation::Vertical => {
                let content_size = self.content.iter().map(|x| x.height()).sum::<usize>();
                let margins = self.margin as usize * (self.content.len() - 1);
                content_size + margins
            }
            Orientation::Horisontal => {
                self.content.iter().map(|x| x.height()).max().unwrap_or(0)
            }
        }
    }
}
