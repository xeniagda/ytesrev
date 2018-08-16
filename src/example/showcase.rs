extern crate ytesrev;

use std::fs::File;

use ytesrev::prelude::*;
use ytesrev::ditherer::color_dither_fn;

fn main() {
    let mut wmng = WindowManager::init_window(
        Box::new(make_first_scene()),
        vec![
            Box::new(make_second_scene()),
            Box::new(make_third_scene()),
            Box::new(make_fourth_scene()),
        ],
    );

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
                Box::new(WithSize::new((0, 40), Empty)),
                Box::new(Ditherer::dithered_out(LatexObj::text("Thing 3 - a bit down"))),
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
                        .with_dither_fn(color_dither_fn)
                        .with_direction(DitherDirection::Outwards),
                    ),
                ]
            )
        )
    )
}

fn make_fourth_scene() -> impl Scene {
    let background =
            Layered::new(
                false,
                vec![
                    Box::new(
                        Margin::new_vert_hor(
                            40, 40,
                            Anchor::new(
                                AnchorDirection::North, Ditherer::dithered_out(LatexObj::text("North"))
                            )
                        )
                    ),
                    Box::new(
                        Margin::new_vert_hor(
                            40, 40,
                            Anchor::new(
                                AnchorDirection::East, Ditherer::dithered_out(LatexObj::text("East"))
                            )
                        )
                    ),
                    Box::new(
                        Margin::new_vert_hor(
                            40, 40,
                            Anchor::new(
                                AnchorDirection::South, Ditherer::dithered_out(LatexObj::text("South"))
                            )
                        )
                    ),
                    Box::new(
                        Margin::new_vert_hor(
                            40, 40,
                            Anchor::new(
                                AnchorDirection::West, Ditherer::dithered_out(LatexObj::text("West"))
                            )
                        )
                    ),
                    Box::new(
                        Margin::new_vert_hor(
                            40, 40,
                            Anchor::new(
                                AnchorDirection::NorthEast, Ditherer::dithered_out(LatexObj::text("NorthEast"))
                            )
                        )
                    ),
                    Box::new(
                        Margin::new_vert_hor(
                            40, 40,
                            Anchor::new(
                                AnchorDirection::SouthEast, Ditherer::dithered_out(LatexObj::text("SouthEast"))
                            )
                        )
                    ),
                    Box::new(
                        Margin::new_vert_hor(
                            40, 40,
                            Anchor::new(
                                AnchorDirection::SouthWest, Ditherer::dithered_out(LatexObj::text("SouthWest"))
                            )
                        )
                    ),
                    Box::new(
                        Margin::new_vert_hor(
                            40, 40,
                            Anchor::new(
                                AnchorDirection::NorthWest, Ditherer::dithered_out(LatexObj::text("NorthWest"))
                            )
                        )
                    ),
                ]
            );

    DrawableWrapper(
        background
    )
}
