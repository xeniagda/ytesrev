extern crate ytesrev;

use std::fs::File;

use ytesrev::window::WindowManager;
use ytesrev::scene::*;
use ytesrev::latex::latex_obj::*;
use ytesrev::ditherer::*;
use ytesrev::layout::*;
use ytesrev::layout::split::*;
use ytesrev::layout::stack::*;
use ytesrev::layout::layered::*;
use ytesrev::image::*;
use ytesrev::solid::*;

fn main() {
    let mut first_scene = make_first_scene();
    let mut second_scene = make_second_scene();
    let mut third_scene = make_third_scene();

    let mut wmng = WindowManager::init_window(&mut first_scene, vec![&mut second_scene, &mut third_scene]);

    wmng.start();
}


fn make_first_scene() -> impl Scene {
    DrawableWrapper(
        Stack::new(
            10,
            Orientation::Vertical,
            ElementPositioning::Centered,
            true,
            vec![
                Box::new(Ditherer::dithered_out(LatexObj::text("Thing 1"))),
                Box::new(Ditherer::dithered_out(LatexObj::text("Thing 2"))),
                Box::new(Ditherer::dithered_out(LatexObj::text("Thing 3"))),
                Box::new(
                    Stack::new(
                        100,
                        Orientation::Horisontal,
                        ElementPositioning::TopLeftCornered,
                        true,
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
            Layered::new(
                false,
                vec![
                    Box::new(Solid::new_rgba(255, 0, 0, 255)),
                    Box::new(Ditherer::dithered_out(LatexObj::math("sin^2\\theta + cos^2\\theta = 1"))),
                ]
            ),
        )
    )
}

fn make_third_scene() -> impl Scene {
    DrawableWrapper(
        SplitPrec::new(
            0.2,
            Orientation::Vertical,
            UpdateOrder::SecondFirst,
            Ditherer::dithered_in(LatexObj::text("\\huge Third page")),
            Stack::new(
                0,
                Orientation::Horisontal,
                ElementPositioning::TopLeftCornered,
                false,
                vec![
                    Box::new(
                        Ditherer::dithered_out(LatexObj::text("Cool image $ \\Rightarrow $"))
                    ),
                    Box::new(
                        Ditherer::dithered_out(PngImage::load_from_path(File::open("image.png").unwrap()).unwrap())
                        .with_dither_fn(|ref img, pos| {
                            let r = img.get_data()[(pos.1 * img.width() + pos.0) * 4    ] as f64;
                            let g = img.get_data()[(pos.1 * img.width() + pos.0) * 4 + 1] as f64;
                            let b = img.get_data()[(pos.1 * img.width() + pos.0) * 4 + 2] as f64;
                            let avg = (r + b + g) / 3.;
                            let dev = (r - avg) * (r - avg) + (g - avg) * (g - avg) + (b - avg) * (b - avg);
                            dev as u64
                        })
                        .with_direction(DitherDirection::Outwards),
                    ),
                ]
            )
        )
    )
}
