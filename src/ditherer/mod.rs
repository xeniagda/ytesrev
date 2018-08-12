use std::cell::Cell;
use std::{u64, f64};

use sdl2::render::{Canvas, BlendMode};
use sdl2::video::Window;

use super::rand::{thread_rng, Rng};

use image::{KnownSize, ImageContainer};
use drawable::{Drawable, Position, State};


const DITHER_SPEED: f64 = 300.;
const DITHER_ALPHA_SPEED: f64 = 140.;

#[derive(PartialEq, Copy, Clone)]
enum DitherState {
    Nothing,
    DitherIn,
    DitherOut,
}

pub fn default_dither_fn<T: ImageContainer + KnownSize>(image: &T, (x, y): (usize, usize)) -> u64 {
    let alpha = move |x: usize, y: usize| {
            let x = x.max(0).min(image.width() - 1);
            let y = y.max(0).min(image.height() - 1);
            image.get_data()[(y * image.width() + x) * 4 + 3]
        };

    let delta_alpha_y = (alpha(x, y + 1) as i64 - alpha(x, y - 1) as i64).abs();
    let delta_alpha_x = (alpha(x + 1, y) as i64 - alpha(x - 1, y) as i64).abs();

    delta_alpha_y.max(delta_alpha_x) as u64
}

pub struct Ditherer<T: ImageContainer + 'static> {
    pub inner: T,
    pub dither: Option<Vec<Vec<u64>>>,
    max_time: u64,
    cached: Cell<Vec<u8>>,
    pub dither_in_time: f64,
    pub dither_out_time: f64,
    pub dither_fn: Box<Fn(&T, (usize, usize)) -> u64>,
    dithering: DitherState,
}

impl <T: ImageContainer + KnownSize> KnownSize for Ditherer<T> {
    fn width(&self) -> usize { self.inner.width() }
    fn height(&self) -> usize { self.inner.height() }
}

impl <T: ImageContainer + KnownSize> ImageContainer for Ditherer<T> {
    fn get_data(&self) -> &Vec<u8> { self.inner.get_data() }
    fn get_data_mut(&mut self) -> &mut Vec<u8> { self.inner.get_data_mut() }
    fn into_data(self) -> Vec<u8> { self.inner.into_data() }
}


impl <T: ImageContainer> Ditherer<T> {
    pub fn dithered_out(inner: T) -> Ditherer<T> {
        let dither = None;

        Ditherer {
            inner,
            dither,
            max_time: 0,
            cached: Cell::new(Vec::new()),
            dither_in_time: 0.,
            dither_out_time: 0.,
            dither_fn: Box::new(default_dither_fn),
            dithering: DitherState::Nothing,
        }
    }

    pub fn dithered_in(inner: T) -> Ditherer<T> {
        let dither = None;

        Ditherer {
            inner,
            dither,
            max_time: 0,
            cached: Cell::new(Vec::new()),
            dither_in_time: 0.,
            dither_out_time: 0.,
            dither_fn: Box::new(default_dither_fn),
            dithering: DitherState::DitherIn,
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

        let mut grad = vec![vec![0u64; self.inner.width()]; self.inner.height()];

        // Find gradient
        for x in 0..self.inner.width() {
            for y in 0..self.inner.height() {
                grad[y][x] = (*self.dither_fn)(&self.inner, (x, y));
            }
        }

        // Select only local maximum

        let mut dither = vec![vec![0u64; self.inner.width()]; self.inner.height()];

        let mut rng = thread_rng();

        for y in 1..self.inner.height()-1 {
            for x in 1..self.inner.width()-1 {
                // Check left/right
                if grad[y][x] > grad[y][x + 1] && grad[y][x] > grad[y][x - 1] && rng.gen() {
                    dither[y][x] = x as u64 + rng.gen_range(0, 100);
                }
                // Check up/down
                if grad[y][x] > grad[y + 1][x] && grad[y][x] > grad[y - 1][x] && rng.gen() {
                    dither[y][x] = x as u64 + rng.gen_range(0, 100);
                }
            }
        }

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
                    for dy in -1..=2 {
                        for dx in -1..=1 {
                            let (ry, rx) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
                            if let Some(pxl) = dither.get(ry).and_then(|line| line.get(rx)) {
                                if *pxl > 0 {
                                    around.push(*pxl);
                                }
                            }
                        }
                    }
                    if around.is_empty() || rng.gen() {
                        continue;
                    }

                    let val = *rng.choose(around.as_slice()).unwrap() + rng.gen_range(20, 40);

                    if dither[y][x] == 0 {
                        dither_next[y][x] = val;
                        self.max_time = self.max_time.max(val + DITHER_ALPHA_SPEED as u64);
                    }
                }
            }

            dither = dither_next;
        }

        // Check for dead spots (areas that should be rendered but weren't)
        for x in 0..self.inner.width() {
            for y in 0..self.inner.height() {
                let alpha = self.inner.get_data()[(y * self.inner.width() + x) * 4 + 3];
                if alpha > 0 && dither[y][x] == 0 {
                    dither[y][x] = self.max_time;
                }
            }
        }

        self.dither = Some(dither);
        self.cached = Cell::new(self.inner.get_data().clone().into_iter().map(|_| 0).collect::<Vec<u8>>());
    }

    fn update(&mut self, dt: f64) {
        match self.dithering {
            DitherState::DitherIn => {
                if self.dither_in_time * DITHER_SPEED < self.max_time as f64 {
                    self.dither_in_time += dt;
                }
            }
            DitherState::DitherOut => {
                if self.dither_in_time * DITHER_SPEED < self.max_time as f64 {
                    self.dither_in_time += dt;
                }
                if self.dither_out_time * DITHER_SPEED < self.max_time as f64 {
                    self.dither_out_time += dt;
                }
            }
            DitherState::Nothing => {}
        }
    }

    fn step(&mut self) {
        match self.dithering {
            DitherState::Nothing => {
                self.dither_in();
            }
            DitherState::DitherIn => {
                self.inner.step();
                self.dither_out();
            }
            DitherState::DitherOut => {
            }
        }
    }

    fn state(&self) -> State {
        match self.dithering {
            DitherState::Nothing => {
                State::Working
            }
            DitherState::DitherIn => {
                State::Final
            }
            DitherState::DitherOut => {
                if self.dither_out_time * DITHER_SPEED < self.max_time as f64 {
                    State::Final
                } else {
                    State::Hidden
                }
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
                    cached[idx    ] = (mult * data[idx    ] as f64) as u8;
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

            let rect = pos.into_rect_with_size(self.inner.width() as u32, self.inner.height() as u32);

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
