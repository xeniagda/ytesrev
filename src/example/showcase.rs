#![allow(missing_docs)]

extern crate ytesrev;

use std::fs::File;

use ytesrev::prelude::*;
use ytesrev::latex::render::add_prelude;
use ytesrev::ditherer::color_dither_fn;

fn main() {
    add_prelude("\\usepackage{skull}");

    let mut wmng = WindowManager::init_main_notes(
        vec![
            Box::new(make_first_scene()),
            Box::new(make_second_scene()),
            Box::new(make_third_scene()),
            Box::new(make_fourth_scene()),
        ],
        "Example".to_string(),
    );

    wmng.start();
}

fn make_first_scene() -> impl Scene {
    DrawableWrapper(Stack::new(
        10,
        Orientation::Vertical,
        ElementPositioning::Centered,
        true,
        vec![
            Box::new(Ditherer::new(LatexObj::text("Thing 1"))),
            Box::new(Ditherer::new(LatexObj::text("Thing 2"))),
            Box::new(WithSize::new((0, 40), Empty)),
            Box::new(Ditherer::new(LatexObj::text(
                "Thing 3 - a bit down",
            ))),
            Box::new(Stack::new(
                100,
                Orientation::Horizontal,
                ElementPositioning::TopLeftCornered,
                true,
                vec![
                    Box::new(Ditherer::new(LatexObj::text("Stack"))),
                    Box::new(Ditherer::new(LatexObj::text("\\emph{in} a stack"))),
                ],
            )),
        ],
    ))
}

fn make_second_scene() -> impl Scene {
    DrawableWrapper(SplitPrec::new(
        0.2,
        Orientation::Vertical,
        UpdateOrder::SecondFirst,
        Ditherer::dithered_in(LatexObj::text("\\huge Second page")),
        Layered::new(
            false,
            vec![
                Box::new(Solid::new_rgba(255, 0, 0, 255)),
                Box::new(Ditherer::new(LatexObj::math(
                    "sin^2\\skull + cos^2\\skull = 1",
                ))),
            ],
        ),
    ))
}

fn make_third_scene() -> impl Scene {
    DrawableWrapper(SplitPrec::new(
        0.2,
        Orientation::Vertical,
        UpdateOrder::SecondFirst,
        Ditherer::dithered_in(LatexObj::text("\\huge Third page")),
        Stack::new(
            0,
            Orientation::Horizontal,
            ElementPositioning::TopLeftCornered,
            false,
            vec![
                Box::new(Ditherer::new(LatexObj::text(
                    "Cool image $ \\Rightarrow $",
                ))),
                Box::new(
                    Ditherer::new(
                        PngImage::load_from_path(File::open("image.png").unwrap()).unwrap(),
                    ).with_dither_fn(color_dither_fn)
                    .with_direction(DitherDirection::Outwards),
                ),
            ],
        ),
    ))
}

fn make_fourth_scene() -> impl Scene {
    let background = Layered::new(
        false,
        vec![
            Box::new(Margin::new_vert_hor(
                40,
                40,
                Anchor::new(
                    AnchorDirection::North,
                    Ditherer::new(LatexObj::text("North")),
                ),
            )),
            Box::new(Margin::new_vert_hor(
                40,
                40,
                Anchor::new(
                    AnchorDirection::East,
                    Ditherer::new(LatexObj::text("East")),
                ),
            )),
            Box::new(Margin::new_vert_hor(
                40,
                40,
                Anchor::new(
                    AnchorDirection::South,
                    Ditherer::new(LatexObj::text("South")),
                ),
            )),
            Box::new(Margin::new_vert_hor(
                40,
                40,
                Anchor::new(
                    AnchorDirection::West,
                    Ditherer::new(LatexObj::text("West")),
                ),
            )),
            Box::new(Margin::new_vert_hor(
                40,
                40,
                Anchor::new(
                    AnchorDirection::NorthEast,
                    Ditherer::new(LatexObj::text("NorthEast")),
                ),
            )),
            Box::new(Margin::new_vert_hor(
                40,
                40,
                Anchor::new(
                    AnchorDirection::SouthEast,
                    Ditherer::new(LatexObj::text("SouthEast")),
                ),
            )),
            Box::new(Margin::new_vert_hor(
                40,
                40,
                Anchor::new(
                    AnchorDirection::SouthWest,
                    Ditherer::new(LatexObj::text("SouthWest")),
                ),
            )),
            Box::new(Margin::new_vert_hor(
                40,
                40,
                Anchor::new(
                    AnchorDirection::NorthWest,
                    Ditherer::new(LatexObj::text("NorthWest")),
                ),
            )),
        ],
    );

    DrawableWrapper(background)
}
