use std::cell::Cell;
use std::u64;

use sdl2::render::{Canvas, BlendMode};
use sdl2::rect::{Point, Rect};
use sdl2::video::Window;

use super::rand::{thread_rng, Rng};

use image::ImageContainer;
use loadable::Loadable;
use scene::Drawable;



const DITHER_SPEED: f64 = 500.;

pub struct Ditherer<T: ImageContainer> {
    pub inner: T,
    pub dither: Option<Vec<Vec<u64>>>,
    cached: Cell<Vec<u8>>,
    pub t: f64,
}


impl <T: ImageContainer> Ditherer<T> {
    pub fn new(inner: T) -> Ditherer<T> {
        let dither = None;

        Ditherer {
            inner: inner,
            dither: dither,
            cached: Cell::new(Vec::new()),
            t: -3.
        }
    }
}

impl <T: ImageContainer> Loadable for Ditherer<T> {
    fn register(&mut self) {
        self.inner.register();
    }

    fn load(&mut self) {
        self.inner.load();

        let mut dither_y = vec![vec![0u64; self.inner.width()]; self.inner.height()];
        let mut dither_x = vec![vec![0u64; self.inner.width()]; self.inner.height()];

        // Find gradient delta up/down
        for x in 0..self.inner.width() {
            for y in 1..self.inner.height() - 1 {
                let over  = self.inner.get_data()[((y - 1) * self.inner.width() + x) * 4 + 3];
                let under = self.inner.get_data()[((y + 1) * self.inner.width() + x) * 4 + 3];

                let delta_alpha = (over as i64 - under as i64).abs() as u64;
                dither_y[y][x] = delta_alpha;
            }
        }

        // Find gradient delta left/right
        for x in 1..self.inner.width()-1 {
            for y in 0..self.inner.height() {
                let left  = self.inner.get_data()[(y * self.inner.width() + x - 1) * 4 + 3];
                let right = self.inner.get_data()[(y * self.inner.width() + x + 1) * 4 + 3];

                let delta_alpha = (left as i64 - right as i64).abs() as u64;
                dither_x[y][x] = delta_alpha;
            }
        }

        // Select only local maximum

        // Up/down
        for x in 0..self.inner.width() {
            let mut last = 0;
            for y in 1..self.inner.height() - 1 {
                if dither_y[y][x] <= last || dither_y[y][x] <= dither_y[y+1][x] {
                    last = dither_y[y][x];
                    dither_y[y][x] = 0;
                } else {
                    dither_y[y][x] = 1 as u64;
                    last = dither_y[y][x];
                }
            }
        }

        // Left/right
        for y in 0..self.inner.height() {
            let mut last = 0;
            for x in 1..self.inner.width()-1 {
                if dither_x[y][x] <= last || dither_x[y][x] <= dither_x[y][x+1] {
                    last = dither_x[y][x];
                    dither_x[y][x] = 0;
                } else {
                    dither_x[y][x] = 1 as u64;
                    last = dither_x[y][x];
                }
            }
        }

        // Combine
        let mut dither = vec![vec![0u64; self.inner.width()]; self.inner.height()];
        for x in 0..self.inner.width() {
            for y in 0..self.inner.height() {
                dither[y][x] = (dither_y[y][x] + dither_x[y][x]) * x as u64;
            }
        }

        let mut rng = thread_rng();

        // Spread the selection
        for i in 0..100 {
            let mut dither_next = dither.clone();


            for y in 0..self.inner.height() {
                for x in 0..self.inner.width() {
                    //if dither[y][x] > 0 {
                        //continue;
                    //}

                    let alpha = self.inner.get_data()[(y * self.inner.width() + x) * 4 + 3];
                    if alpha < 10 {
                        continue;
                    }

                    let mut around = vec![];
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let (ry, rx) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
                            if let Some(pxl) = dither.get(ry).and_then(|line| line.get(rx)) {
                                if *pxl > 0 {
                                    around.push(*pxl);
                                }
                            }
                        }
                    }
                    let val = *rng.choose(around.as_slice()).unwrap_or(&dither[y][x]) + rng.gen_range(40, 80);

                    if dither[y][x] == 0 || dither[y][x] > val {
                        dither_next[y][x] = val;
                    }
                }
            }

            dither = dither_next;
        }

        self.dither = Some(dither);
        self.cached = Cell::new(self.inner.get_data().clone().into_iter().map(|_| 0).collect::<Vec<u8>>());
    }
}

impl <T: ImageContainer> Drawable for Ditherer<T> {
    fn update(&mut self, dt: f64) {
        self.t += dt;
    }

    fn draw(&self, canvas: &mut Canvas<Window>, point: &Point) {
        if let Some(ref dither) = self.dither {
            let mut cached = self.cached.take();

            for y in 0..self.inner.height() {
                for x in 0..self.inner.width() {

                    let diff = (self.t * DITHER_SPEED) - dither[y][x] as f64;
                    let mult = (diff / 100.).min(1.).max(0.);
                    let idx = (y * self.inner.width() + x) * 4;

                    let data = self.inner.get_data();
                    cached[idx + 0] = (mult * data[idx + 0] as f64) as u8;
                    cached[idx + 1] = (mult * data[idx + 1] as f64) as u8;
                    cached[idx + 2] = (mult * data[idx + 2] as f64) as u8;
                    cached[idx + 3] = (mult * data[idx + 3] as f64) as u8;
                }
            }
            let creator = canvas.texture_creator();
            let mut texture = creator
                .create_texture_target(None, self.inner.width() as u32, self.inner.height() as u32)
                .expect("Can't make texture");

            texture.set_blend_mode(BlendMode::Blend);

            texture
                .update(None, cached.as_slice(), 4 * self.inner.width())
                .expect("Can't update");

            self.cached.set(cached);

            canvas
                .copy(
                    &texture,
                    None,
                    Rect::new(point.x, point.y, self.inner.width() as u32, self.inner.height() as u32),
                )
                .expect("Can't copy");
        } else {
            self.inner.draw(canvas, point);
        }
    }
}
