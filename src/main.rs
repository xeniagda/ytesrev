#![feature(duration_as_u128, nll)]
extern crate sdl2;
extern crate png;

mod window;
mod image;
mod scene;
use window::WindowManager;
use scene::{DrawableWrapper, Scene};

use std::fs::File;

fn main() {
    let img = image::PngImage
        ::load_from_path(File::open("test.png-1.png").expect("Can't open"))
        .expect("Can't read png");

    let scene: &dyn Scene = &DrawableWrapper(img);

    let mut wmng = WindowManager::init_window(scene);
    wmng.start();
}
