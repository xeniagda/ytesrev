extern crate sdl2;

use sdl2::rect::{Point, Rect};
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
    fn as_sizeable(    &    self) -> &    dyn KnownSize;
    fn as_sizeable_mut(&mut self) -> &mut dyn KnownSize;
    fn as_drawable(    &    self) -> &    dyn Drawable;
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable;
}

impl <T: Drawable + KnownSize> Stackable for T {
    fn as_sizeable(&self) -> &dyn KnownSize  { self }
    fn as_sizeable_mut(&mut self) -> &mut dyn KnownSize  { self }
    fn as_drawable(&self) -> &dyn Drawable { self }
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable { self }
}

pub struct Stack {
    margin: u32,
    orientation: Orientation,
    positioning: ElementPositioning,
    content: Vec<Box<dyn Stackable>>,
}

impl Stack {
    pub fn new(
        margin: u32,
        orientation: Orientation,
        positioning: ElementPositioning,
        content: Vec<Box<dyn Stackable>>,
    ) -> Stack {
        Stack {
            margin,
            orientation,
            positioning,
            content,
        }
    }
}

impl <'a> Drawable for Stack {
    fn content(&self) -> Vec<&dyn Drawable> {
        self.content.iter().map(|x| x.as_ref().as_drawable()).collect()
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        self.content.iter_mut().map(|x| x.as_mut().as_drawable_mut()).collect()
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position) {
        let corner = pos.into_rect_with_size(self.width() as u32, self.height() as u32).top_left();

        let (width, height) = (self.width(), self.height());

        match self.orientation {
            Orientation::Vertical => {
                let mut y = corner.y;
                for obj in &mut self.content {
                    if super::DRAW_BOXES {
                        canvas.set_draw_color(Color::RGB(255, 0, 0));
                        canvas.draw_rect(
                            Rect::new(corner.x, y, obj.as_sizeable().width() as u32, obj.as_sizeable().height() as u32)
                        ).expect("Can't draw");
                    }

                    match self.positioning {
                        ElementPositioning::TopLeftCornered => {
                            obj.as_drawable_mut().draw(canvas, &Position::TopLeftCorner(Point::new(corner.x, y)));
                        }
                        ElementPositioning::Centered => {
                            let px = corner.x + width  as i32 / 2;
                            obj.as_drawable_mut().draw(canvas, &Position::Center(Point::new(px, y)));
                        }
                    }
                    y += obj.as_sizeable().height() as i32 + self.margin as i32;
                }
            }
            Orientation::Horisontal => {
                let mut x = corner.x;
                for obj in &mut self.content {
                    if super::DRAW_BOXES {
                        canvas.set_draw_color(Color::RGB(255, 0, 0));
                        canvas.draw_rect(
                            Rect::new(x, corner.y, obj.as_sizeable().width() as u32, obj.as_sizeable().height() as u32)
                        ).expect("Can't draw");
                    }

                    match self.positioning {
                        ElementPositioning::TopLeftCornered => {
                            obj.as_drawable_mut().draw(canvas, &Position::TopLeftCorner(Point::new(x, corner.y)));
                        }
                        ElementPositioning::Centered => {
                            let py = corner.y + height as i32 / 2;
                            obj.as_drawable_mut().draw(canvas, &Position::Center(Point::new(x, py)));
                        }
                    }
                    x += obj.as_sizeable().width() as i32 + self.margin as i32;
                }
            }
        }
    }

    fn step(&mut self) {
        for item in &mut self.content {
            if item.state() == State::Working {
                item.step();
                return;
            }
        }
        for item in &mut self.content {
            if item.state() == State::Final {
                item.step();
            }
        }
    }

    fn state(&self) -> State {
        self.content.iter().map(|x| x.as_drawable().state()).min().unwrap_or(State::Hidden)
    }
}

impl KnownSize for Stack {
    fn width(&self)  -> usize {
        match self.orientation {
            Orientation::Horisontal => {
                let content_size = self.content.iter().map(|x| x.as_sizeable().width()).sum::<usize>();
                let margins = self.margin as usize * (self.content.len() - 1);
                content_size + margins
            }
            Orientation::Vertical => {
                self.content.iter().fold(0, |old, obj| obj.width().max(old))
            }
        }
    }
    fn height(&self) -> usize {
        match self.orientation {
            Orientation::Vertical => {
                let content_size = self.content.iter().map(|x| x.as_sizeable().height()).sum::<usize>();
                let margins = self.margin as usize * (self.content.len() - 1);
                content_size + margins
            }
            Orientation::Horisontal => {
                self.content.iter().fold(0, |old, obj| obj.height().max(old))
            }
        }
    }
}
