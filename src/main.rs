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
use layout::Orientation;
use layout::split::{SplitPrec, UpdateOrder};
use layout::stack::{Stack, ElementPositioning};

fn main() {
    let mut first_scene = make_first_scene();
    let mut second_scene = make_second_scene();

    let mut wmng = WindowManager::init_window(&mut first_scene, vec![&mut second_scene]);

    wmng.start();
}


fn make_first_scene() -> impl Scene {
    DrawableWrapper(
        Stack::new(
            10,
            Orientation::Vertical,
            ElementPositioning::Centered,
            vec![
                Box::new(Ditherer::dithered_out(LatexObj::text("Thing 1"))),
                Box::new(Ditherer::dithered_out(LatexObj::text("Thing 2"))),
                Box::new(Ditherer::dithered_out(LatexObj::text("Thing 3"))),
                Box::new(
                    Stack::new(
                        100,
                        Orientation::Horisontal,
                        ElementPositioning::TopLeftCornered,
                        vec![
                            Box::new(Ditherer::dithered_out(LatexObj::text("Stack"))),
                            Box::new(Ditherer::dithered_out(LatexObj::text("\\emph{in} a stack"))),
                        ]
                    )
                )
            ]
        )
    )
}

fn make_second_scene() -> impl Scene {
    DrawableWrapper(
        SplitPrec::new(
            0.2,
            Orientation::Vertical,
            UpdateOrder::SecondFirst,
            Ditherer::dithered_in(LatexObj::text("\\huge Second page")),
            Ditherer::dithered_out(LatexObj::math("a^2 + b^2 = c^2")),
        )
    )
}
