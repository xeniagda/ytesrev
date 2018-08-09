use std::cell::Cell;
use std::u64;

use sdl2::render::{Canvas, BlendMode};
use sdl2::rect::Rect;
use sdl2::video::Window;

use super::rand::{thread_rng, Rng};

use image::ImageContainer;
use drawable::{Drawable, Position};


const DITHER_SPEED: f64 = 300.;
const DITHER_ALPHA_SPEED: f64 = 140.;

#[derive(PartialEq, Copy, Clone)]
enum DitherState {
    Nothing,
    DitherIn,
    DitherOut,
}

pub struct Ditherer<T: ImageContainer> {
    pub inner: T,
    pub dither: Option<Vec<Vec<u64>>>,
    max_time: u64,
    cached: Cell<Vec<u8>>,
    pub dither_in_time: f64,
    pub dither_out_time: f64,
    dithering: DitherState,
}

impl <T: ImageContainer> ImageContainer for Ditherer<T> {
    fn get_data(&self) -> &Vec<u8> { self.inner.get_data() }
    fn get_data_mut(&mut self) -> &mut Vec<u8> { self.inner.get_data_mut() }
    fn into_data(self) -> Vec<u8> { self.inner.into_data() }
    fn width(&self) -> usize { self.inner.width() }
    fn height(&self) -> usize { self.inner.height() }
}

impl <T: ImageContainer> Ditherer<T> {
    pub fn new(inner: T) -> Ditherer<T> {
        let dither = None;

        Ditherer {
            inner: inner,
            dither: dither,
            max_time: 0,
            cached: Cell::new(Vec::new()),
            dither_in_time: 0.,
            dither_out_time: 0.,
            dithering: DitherState::Nothing,
        }
    }

    pub fn dither_in(&mut self) {
        self.dithering = DitherState::DitherIn;
    }

    pub fn dither_out(&mut self) {
        self.dithering = DitherState::DitherOut;
    }
}

impl <T: ImageContainer> Drawable for Ditherer<T> {

    fn     content(&    self) -> Vec<&    dyn Drawable> { vec![&    self.inner] }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![&mut self.inner] }

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
        for _ in 0..100 {
            let mut dither_next = dither.clone();


            for y in 0..self.inner.height() {
                for x in 0..self.inner.width() {

                    let alpha = self.inner.get_data()[(y * self.inner.width() + x) * 4 + 3];
                    if alpha == 0 {
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
                        self.max_time = self.max_time.max(val);
                    }
                }
            }

            dither = dither_next;
        }

        self.dither = Some(dither);
        self.cached = Cell::new(self.inner.get_data().clone().into_iter().map(|_| 0).collect::<Vec<u8>>());
    }

    fn update(&mut self, dt: f64) {
        match self.dithering {
            DitherState::DitherIn  => {
                self.dither_in_time += dt;
            }
            DitherState::DitherOut => {
                self.dither_in_time += dt;
                self.dither_out_time += dt;
            }
            DitherState::Nothing   => {}
        }
    }

    fn step(&mut self) -> bool {
        match self.dithering {
            DitherState::Nothing => {
                self.dither_in();
                true
            }
            DitherState::DitherIn => {
                if !self.inner.step() {
                    self.dither_out();
                }
                true
            }
            DitherState::DitherOut => {
                false
            }
        }
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position) {
        if let Some(ref dither) = self.dither {
            let mut cached = self.cached.take();

            for y in 0..self.inner.height() {
                for x in 0..self.inner.width() {

                    let mut mult = 1.;

                    let diff_out = dither[y][x] as f64 - (self.dither_out_time * DITHER_SPEED);
                    mult *= (diff_out / DITHER_ALPHA_SPEED + 1.).min(1.).max(0.);

                    let diff_in = (self.dither_in_time * DITHER_SPEED) - dither[y][x] as f64;
                    mult *= (diff_in  / DITHER_ALPHA_SPEED).min(1.).max(0.);

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

            let rect =
                match pos {
                    Position::TopLeftCorner(point) => {
                        Rect::new(point.x, point.y, self.inner.width() as u32, self.inner.height() as u32)
                    }
                    Position::Center(point) => {
                        Rect::new(
                            point.x - self.inner.width()  as i32 / 2,
                            point.y - self.inner.height() as i32 / 2,
                            self.inner.width() as u32,
                            self.inner.height() as u32
                        )
                    }
                    Position::Rect(rect) => {
                        Rect::new(
                            rect.x,
                            rect.y,
                            self.inner.width() as u32,
                            self.inner.height() as u32
                        )
                    }
                };


            canvas
                .copy(
                    &texture,
                    None,
                    rect,
                )
                .expect("Can't copy");
        } else {
            self.inner.draw(canvas, pos);
        }
    }
}
