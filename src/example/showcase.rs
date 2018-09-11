#![allow(missing_docs)]

extern crate ytesrev;

use std::fs::File;

use ytesrev::ditherer::color_dither_fn;
use ytesrev::latex::render::add_prelude;
use ytesrev::prelude::*;

fn main() {
    add_prelude("\\usepackage{skull}");

    let slist = SceneList::new(vec![
        Box::new(make_first_scene()),
        Box::new(make_second_scene()),
        Box::new(make_third_scene()),
        Box::new(make_fourth_scene()),
        Box::new(make_fifth_scene()),
    ]);

    let mut wmng = WindowManager::init_window(slist, default_settings("Showcase"));

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
            Box::new(Ditherer::new(LatexObj::text("Thing 3 - a bit down"))),
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
    DrawableWrapper(Split::new_ratio(
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
    DrawableWrapper(Split::new_ratio(
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
                Box::new(Ditherer::new(LatexObj::text("Cool image $ \\Rightarrow $"))),
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
                Anchor::new(AnchorDirection::East, Ditherer::new(LatexObj::text("East"))),
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
                Anchor::new(AnchorDirection::West, Ditherer::new(LatexObj::text("West"))),
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

use ytesrev::drawable::{DrawSettings, Position, State};
use ytesrev::sdl2::pixels::Color;
use ytesrev::sdl2::render::Canvas;
use ytesrev::sdl2::video::Window;

struct Line(bool);

impl Drawable for Line {
    fn content(&self) -> Vec<&dyn Drawable> {
        Vec::new()
    }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        Vec::new()
    }
    fn step(&mut self) {
        self.0 = false;
    }
    fn state(&self) -> State {
        if self.0 {
            State::Final
        } else {
            State::Hidden
        }
    }
    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &Position, _: DrawSettings) {
        let r = match position {
            Position::Rect(r) => *r,
            _ => position.into_rect_with_size(100, 100),
        };

        let (p1, p2) = (r.top_left(), r.bottom_right());

        canvas.set_draw_color(Color::RGB(0, 255, 0));

        if self.0 {
            utils::line_aa(
                canvas,
                (p1.x() as f64, p1.y() as f64),
                (p2.x() as f64, p2.y() as f64),
            );
        }
    }
}

fn make_fifth_scene() -> impl Scene {
    DrawableWrapper(Split::new_ratio(
        0.2,
        Orientation::Vertical,
        UpdateOrder::SecondFirst,
        Ditherer::new(LatexObj::text("Antialiased lines!")),
        Line(true),
    ))
}
