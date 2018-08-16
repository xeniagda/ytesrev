use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{Drawable, Position, State, DrawSettings};
use image::KnownSize;

pub struct Margin<T: Drawable + KnownSize> {
    pub margin: (u32, u32, u32, u32), // Top, right, bottom, left
    pub inner: T,
}

impl <T: Drawable + KnownSize> Margin<T> {
    pub fn new(margin: (u32, u32, u32, u32), inner: T) -> Margin<T> {
        Margin {
            margin,
            inner,
        }
    }
    pub fn new_vert_hor(vertical: u32, horizontal: u32, inner: T) -> Margin<T> {
        Margin {
            margin: (vertical, horizontal, vertical, horizontal),
            inner,
        }
    }
}

impl <T: Drawable + KnownSize> Drawable for Margin<T> {

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
        match pos {
            Position::Rect(r) => {
                let r2 =
                    Rect::new(
                        r.x() + self.margin.1 as i32,
                        r.y() + self.margin.0 as i32,
                        r.width()  - self.margin.1 - self.margin.3,
                        r.height() - self.margin.0 - self.margin.2,
                    );

                if settings.notes_view { // TODO: Fewer expects
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                    canvas.draw_rect(*r).expect("Can't draw");
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas.draw_rect(r2).expect("Can't draw");

                    canvas.set_draw_color(Color::RGB(0, 0, 255));
                    canvas.draw_line(r.top_left(), r2.top_left()).expect("Can't draw");
                    canvas.draw_line(r.top_right(), r2.top_right()).expect("Can't draw");
                    canvas.draw_line(r.bottom_left(), r2.bottom_left()).expect("Can't draw");
                    canvas.draw_line(r.bottom_right(), r2.bottom_right()).expect("Can't draw");
                }

                self.inner.draw(canvas, &Position::Rect(r2), settings);
            }
            _ => {
                self.inner.draw(canvas, pos, settings);
            }
        }
    }
}

impl <T: Drawable + KnownSize> KnownSize for Margin<T> {
    fn width(&self) -> usize {
        self.inner.width() + (self.margin.1 + self.margin.3) as usize
    }

    fn height(&self) -> usize {
        self.inner.height() + (self.margin.0 + self.margin.2) as usize
    }
}
