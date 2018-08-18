//! A dithering effect, useful for fading images and text, in and out.

use std::cell::Cell;
use std::{f64, u64};

use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;

use super::rand::{thread_rng, Rng};

use drawable::{DrawSettings, Drawable, Position, State, KnownSize};
use image::ImageContainer;

const DITHER_SPEED: f64 = 350.;
const DITHER_ALPHA_SPEED: f64 = 140.;
const MAX_TIME: f64 = 2.5;

#[derive(PartialEq, Copy, Clone)]
enum DitherState {
    Nothing,
    DitherIn,
    DitherOut,
}

/// The direction to dither
#[derive(PartialEq, Copy, Clone)]
#[allow(unused,missing_docs)]
pub enum DitherDirection {
    Leftwards,
    Rightwards,
    Downwards,
    Upwards,
    Outwards,
    None,
}

impl DitherDirection {
    /// At what time should this pixel be faded in?
    pub fn value(&self, pos: (usize, usize), size: (usize, usize)) -> usize {
        match self {
            DitherDirection::Rightwards => pos.0,
            DitherDirection::Leftwards => size.0 - pos.0 + 1,
            DitherDirection::Downwards => pos.1,
            DitherDirection::Upwards => size.1 - pos.1 + 1,
            DitherDirection::Outwards => {
                let dx = pos.0 as f64 - size.0 as f64 / 2.;
                let dy = pos.1 as f64 - size.1 as f64 / 2.;
                (dx * dx + dy * dy).sqrt() as usize
            }
            DitherDirection::None => 1,
        }
    }
}

/// A dithering function that is useful for images with transparency in them, fading regions with
/// high alpha gradient first
pub fn alpha_dither_fn<T: ImageContainer + KnownSize>(image: &T, (x, y): (usize, usize)) -> u64 {
    let alpha = move |x: usize, y: usize| {
        let x = x.max(0).min(image.width() - 1);
        let y = y.max(0).min(image.height() - 1);
        image.get_data()[(y * image.width() + x) * 4 + 3]
    };

    let delta_alpha_y = (alpha(x, y + 1) as i64 - alpha(x, y.saturating_sub(1)) as i64).abs();
    let delta_alpha_x = (alpha(x + 1, y) as i64 - alpha(x.saturating_sub(1), y) as i64).abs();

    delta_alpha_y.max(delta_alpha_x) as u64
}

fn color_diff(c1: (u8, u8, u8, u8), c2: (u8, u8, u8, u8)) -> u64 {
    let r_diff = c1.0 as f64 - c2.0 as f64;
    let g_diff = c1.1 as f64 - c2.1 as f64;
    let b_diff = c1.2 as f64 - c2.2 as f64;
    let a_diff = c1.3 as f64 - c2.3 as f64;

    let r_dev = r_diff * r_diff;
    let g_dev = g_diff * g_diff;
    let b_dev = b_diff * b_diff;
    let a_dev = a_diff * a_diff;

    r_dev.max(g_dev).max(b_dev).max(a_dev) as u64
}

/// A dithering function that dithers based on color deviation gradient
pub fn color_dither_fn<T: ImageContainer + KnownSize>(image: &T, (x, y): (usize, usize)) -> u64 {
    let color = move |x: usize, y: usize| {
        let x = x.max(0).min(image.width() - 1);
        let y = y.max(0).min(image.height() - 1);

        let r = image.get_data()[(y * image.width() + x) * 4 + 3];
        let g = image.get_data()[(y * image.width() + x) * 4 + 2];
        let b = image.get_data()[(y * image.width() + x) * 4 + 1];
        let a = image.get_data()[(y * image.width() + x) * 4    ];
        (r, g, b, a)
    };

    let grad_x = color_diff(color(x.saturating_sub(1), y), color(x + 1, y));
    let grad_y = color_diff(color(x, y.saturating_sub(1)), color(x, y + 1));
    grad_x.max(grad_y)
}

/// The ditherer itself. The inner type `T` is the thing to be dithered
pub struct Ditherer<T: ImageContainer + 'static> {
    /// The inner object to be dithered
    pub inner: T,
    /// The time each pixel will be dithered in
    pub dither: Option<Vec<Vec<u64>>>,
    max_time: u64,
    cached: Cell<Vec<u8>>,
    /// How long the ditherer has been dithering in for
    pub dither_in_time: f64,
    /// How long the ditherer has been dithering out for
    pub dither_out_time: f64,
    /// The function to determine what regionso to dither first
    pub dither_fn: Box<Fn(&T, (usize, usize)) -> u64>,
    /// The direction to dither in
    pub direction: DitherDirection,
    dithering: DitherState,
}

impl<T: ImageContainer + KnownSize> KnownSize for Ditherer<T> {
    fn width(&self) -> usize {
        self.inner.width()
    }
    fn height(&self) -> usize {
        self.inner.height()
    }
}

impl<T: ImageContainer + KnownSize> ImageContainer for Ditherer<T> {
    fn get_data(&self) -> &Vec<u8> {
        self.inner.get_data()
    }
    fn get_data_mut(&mut self) -> &mut Vec<u8> {
        self.inner.get_data_mut()
    }
    fn into_data(self) -> Vec<u8> {
        self.inner.into_data()
    }
}

impl<T: ImageContainer> Ditherer<T> {
    /// Create a new ditherer instance, dithering using [`alpha_dither_fn`].
    pub fn new(inner: T) -> Ditherer<T> {
        let dither = None;

        Ditherer {
            inner,
            dither,
            max_time: 0,
            cached: Cell::new(Vec::new()),
            dither_in_time: 0.,
            dither_out_time: 0.,
            dither_fn: Box::new(alpha_dither_fn),
            direction: DitherDirection::Rightwards,
            dithering: DitherState::Nothing,
        }
    }

    /// Create a new ditherer that is already dithered in
    pub fn dithered_in(inner: T) -> Ditherer<T> {
        let dither = None;

        Ditherer {
            inner,
            dither,
            max_time: 0,
            cached: Cell::new(Vec::new()),
            dither_in_time: 0.,
            dither_out_time: 0.,
            dither_fn: Box::new(alpha_dither_fn),
            direction: DitherDirection::Rightwards,
            dithering: DitherState::DitherIn,
        }
    }

    /// Convert a ditherer to use a specific dither function
    pub fn with_dither_fn<F: Fn(&T, (usize, usize)) -> u64>(self, f: F) -> Ditherer<T>
    where
        F: 'static,
    {
        Ditherer {
            dither_fn: Box::new(f),
            ..self
        }
    }

    /// Convert a ditherer to use a specific direction function
    pub fn with_direction(self, dir: DitherDirection) -> Ditherer<T> {
        Ditherer {
            direction: dir,
            ..self
        }
    }

    /// Start dither in this object
    pub fn dither_in(&mut self) {
        self.dithering = DitherState::DitherIn;
    }

    /// Start dither out this object
    pub fn dither_out(&mut self) {
        self.dithering = DitherState::DitherOut;
    }

    /// Is the ditherer dithered in?
    fn is_dithered_in(&self) -> bool {
        self.dither_in_time * DITHER_SPEED > self.max_time as f64
    }

    /// Is the ditherer dithered out?
    fn is_dithered_out(&self) -> bool {
        self.dither_out_time * DITHER_SPEED > self.max_time as f64
    }
}

impl<T: ImageContainer> Drawable for Ditherer<T> {
    fn content(&self) -> Vec<&dyn Drawable> {
        vec![&self.inner]
    }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![&mut self.inner]
    }

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

        for y in 0..self.inner.height() {
            for x in 0..self.inner.width() {
                let val = self
                    .direction
                    .value((x, y), (self.inner.width(), self.inner.height()))
                    as u64;
                let val = val as u64 + rng.gen_range(0, 100);

                if x == 0 || x == self.inner.width() - 1 || y == 0 || y == self.inner.height() - 1 {
                    dither[y][x] = val;
                    continue;
                }
                // Check left/right
                if grad[y][x] > grad[y][x + 1] && grad[y][x] > grad[y][x - 1] && rng.gen() {
                    dither[y][x] = val;
                    continue;
                }
                // Check up/down
                if grad[y][x] > grad[y + 1][x] && grad[y][x] > grad[y - 1][x] && rng.gen() {
                    dither[y][x] = val;
                    continue;
                }
            }
        }

        // Spread the selection
        for _ in 0..50 {
            let mut dither_next = dither.clone();

            for y in 0..self.inner.height() {
                for x in 0..self.inner.width() {
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

        if self.dithering == DitherState::DitherIn {
            self.dither_in_time = self.max_time as f64 * DITHER_SPEED;
        }

        self.dither = Some(dither);
        self.cached = Cell::new(
            self.inner
                .get_data()
                .clone()
                .into_iter()
                .map(|_| 0)
                .collect::<Vec<u8>>(),
        );
    }

    fn update(&mut self, dt: f64) {
        match self.dithering {
            DitherState::DitherIn => {
                if !self.is_dithered_in() {
                    self.dither_in_time += dt;
                }
            }
            DitherState::DitherOut => {
                if !self.is_dithered_in() {
                    self.dither_in_time += dt;
                }
                if !self.is_dithered_out() {
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
                if self.is_dithered_out() {
                    State::Hidden
                } else {
                    State::Final
                }
            }
        }
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position, settings: DrawSettings) {
        let mut t_mult = 1.;
        if self.max_time as f64 > MAX_TIME * DITHER_SPEED {
            t_mult = self.max_time as f64 / (MAX_TIME * DITHER_SPEED);
        }

        match self.dithering {
            DitherState::Nothing if !settings.notes_view => {}
            DitherState::DitherIn if self.is_dithered_in() && !settings.notes_view => {
                self.inner.draw(canvas, pos, settings);
            }
            _ => {
                if self.is_dithered_out() && !settings.notes_view {
                    return;
                }
                if let Some(ref dither) = self.dither {
                    let mut cached = self.cached.take();

                    for y in 0..self.inner.height() {
                        for x in 0..self.inner.width() {
                            let mut mult = 1.;

                            let diff_out = dither[y][x] as f64
                                - (self.dither_out_time * DITHER_SPEED * t_mult);
                            mult *= (diff_out / DITHER_ALPHA_SPEED + 1.).min(1.).max(0.);

                            let diff_in =
                                (self.dither_in_time * DITHER_SPEED * t_mult) - dither[y][x] as f64;
                            mult *= (diff_in / DITHER_ALPHA_SPEED).min(1.).max(0.);

                            let idx = (y * self.inner.width() + x) * 4;

                            let data = self.inner.get_data();
                            if settings.notes_view {
                                let nmult_min = 0.2;
                                let nmult = mult * (1. - nmult_min) + nmult_min;

                                let (r, g, b) =
                                    if self.dithering == DitherState::Nothing {
                                        (1., nmult_min, nmult_min)
                                    } else if !self.is_dithered_in() {
                                        (1. - mult, nmult, nmult_min)
                                    } else if self.is_dithered_in() && self.dithering == DitherState::DitherIn {
                                        (0., 1., nmult_min)
                                    } else if self.dithering == DitherState::DitherOut {
                                        (0., mult, 1. - nmult)
                                    } else {
                                        (0., 0., 1.)
                                    };

                                let avg = data[idx] / 3 + data[idx + 1] / 3 + data[idx + 2] / 3;

                                cached[idx    ] = (b * (255. - avg as f64)) as u8;
                                cached[idx + 1] = (g * (255. - avg as f64)) as u8;
                                cached[idx + 2] = (r * (255. - avg as f64)) as u8;
                                cached[idx + 3] = (data[idx + 3] as f64) as u8;
                            } else {
                                cached[idx    ] = (mult * data[idx    ] as f64) as u8;
                                cached[idx + 1] = (mult * data[idx + 1] as f64) as u8;
                                cached[idx + 2] = (mult * data[idx + 2] as f64) as u8;
                                cached[idx + 3] = (mult * data[idx + 3] as f64) as u8;
                            }
                        }
                    }
                    let creator = canvas.texture_creator();
                    let mut texture = creator
                        .create_texture_target(
                            None,
                            self.inner.width() as u32,
                            self.inner.height() as u32,
                        ).expect("Can't make texture");

                    texture.set_blend_mode(BlendMode::Blend);

                    texture
                        .update(None, cached.as_slice(), 4 * self.inner.width())
                        .expect("Can't update");

                    self.cached.set(cached);

                    let rect = pos
                        .into_rect_with_size(self.inner.width() as u32, self.inner.height() as u32);

                    canvas.copy(&texture, None, rect).expect("Can't copy");
                } else {
                    self.inner.draw(canvas, pos, settings);
                }
            }
        }
    }
}
