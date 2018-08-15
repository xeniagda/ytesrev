use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{Drawable, Position, State, DrawSettings};
use image::KnownSize;


pub enum AnchorDirection {
    North,
    East,
    South,
    West,

    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

pub struct Anchor<T: Drawable + KnownSize> {
    pub inner: T,
    pub direction: AnchorDirection,
}

impl <T: Drawable + KnownSize> Anchor<T> {
    pub fn new(direction: AnchorDirection, inner: T) -> Anchor<T> {
        Anchor {
            direction,
            inner,
        }
    }
}

impl <T: Drawable + KnownSize> Drawable for Anchor<T> {
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
        let rect =
            match pos {
                Position::Rect(r) => r,
                _ => {
                    self.inner.draw(canvas, pos, settings);
                    return;
                }
            };

        let (iwidth, iheight) = (self.width() as i32, self.height() as i32);
        let (rwidth, rheight) = (rect.width() as i32, rect.height() as i32);


        let corner =
            match self.direction {
                AnchorDirection::North => {
                    Point::new(rect.x + (rwidth - iwidth) / 2, rect.y)
                }
                AnchorDirection::East => {
                    Point::new(rect.x + rwidth - iwidth, rect.y + (rheight - iheight) / 2)
                }
                AnchorDirection::South => {
                    Point::new(rect.x + (rwidth - iwidth) / 2, rect.y + rheight - iheight)
                }
                AnchorDirection::West => {
                    Point::new(rect.x, rect.y + (rheight - iheight) / 2)
                }
                AnchorDirection::NorthWest => {
                    Point::new(rect.x, rect.y)
                }
                AnchorDirection::NorthEast => {
                    Point::new(rect.x + rwidth - iwidth, rect.y)
                }
                AnchorDirection::SouthWest => {
                    Point::new(rect.x, rect.y + rheight - iheight)
                }
                AnchorDirection::SouthEast => {
                    Point::new(rect.x + rwidth - iwidth, rect.y + rheight - iheight)
                }
            };
        self.inner
            .draw(canvas, &Position::Rect(Rect::new(corner.x, corner.y, iwidth as u32, iheight as u32)), settings);
    }
}

impl <T: Drawable + KnownSize> KnownSize for Anchor<T> {
    fn width(&self) -> usize {
        self.inner.width()
    }

    fn height(&self) -> usize {
        self.inner.height()
    }
}
