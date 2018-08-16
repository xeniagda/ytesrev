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
pub mod empty;
pub mod withsize;
pub mod anchor;
pub mod margin;

pub mod prelude {
    pub use window::WindowManager;
    pub use drawable::Drawable;
    pub use scene::{DrawableWrapper, Scene};
    pub use latex::latex_obj::LatexObj;
    pub use ditherer::{Ditherer, DitherDirection};
    pub use layout::Orientation;
    pub use layout::split::{SplitPrec, UpdateOrder};
    pub use layout::stack::{Stack, ElementPositioning};
    pub use layout::layered::Layered;
    pub use image::PngImage;
    pub use solid::Solid;
    pub use empty::Empty;
    pub use withsize::WithSize;
    pub use anchor::{Anchor, AnchorDirection};
    pub use margin::Margin;
}
