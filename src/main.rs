#![feature(duration_as_u128, nll, specialization)]

#[macro_use]
extern crate lazy_static;

extern crate sdl2;
extern crate png;
extern crate rand;

mod window;
mod image;
mod scene;
mod latex;
mod ditherer;
mod layout;

mod drawable;

use window::WindowManager;
use scene::{Scene, DrawableWrapper};
use latex::latex_obj::LatexObj;
use ditherer::Ditherer;
use layout::{SplitPrec, Orientation, UpdateOrder};

fn main() {
    let mut first_scene = make_first_scene();

    let mut wmng = WindowManager::init_window(&mut first_scene, vec![]);

    wmng.start();
}

fn make_first_scene() -> impl Scene {
    DrawableWrapper(
        SplitPrec::new(
            0.3,
            Orientation::UpDown,
            UpdateOrder::SecondFirst,
            Ditherer::dithered_in(LatexObj::text("\\large Title")),
            Ditherer::dithered_out(LatexObj::math("E = mc^2")),
        )
    )
}
