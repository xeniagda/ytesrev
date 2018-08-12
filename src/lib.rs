#![feature(duration_as_u128, nll, specialization)]

#[macro_use]
extern crate lazy_static;
extern crate sdl2;
extern crate png;
extern crate rand;

pub mod window;
pub mod image;
pub mod scene;
pub mod latex;
pub mod ditherer;
pub mod layout;
pub mod drawable;
pub mod solid;
