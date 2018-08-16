#![feature(duration_as_u128, nll, specialization)]

#[macro_use]
extern crate lazy_static;
extern crate png;
extern crate rand;
extern crate sdl2;

pub mod anchor;
pub mod ditherer;
pub mod drawable;
pub mod empty;
pub mod image;
pub mod latex;
pub mod layout;
pub mod margin;
pub mod scene;
pub mod solid;
pub mod window;
pub mod withsize;

pub mod prelude {
    pub use anchor::{Anchor, AnchorDirection};
    pub use ditherer::{DitherDirection, Ditherer};
    pub use drawable::Drawable;
    pub use empty::Empty;
    pub use image::PngImage;
    pub use latex::latex_obj::LatexObj;
    pub use layout::layered::Layered;
    pub use layout::split::{SplitPrec, UpdateOrder};
    pub use layout::stack::{ElementPositioning, Stack};
    pub use layout::Orientation;
    pub use margin::Margin;
    pub use scene::{DrawableWrapper, Scene};
    pub use solid::Solid;
    pub use window::WindowManager;
    pub use withsize::WithSize;
}
