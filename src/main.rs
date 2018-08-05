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
#[macro_use]
mod state;

use std::fs::File;

use window::{WindowManager, YEvent};
use scene::Scene;
use drawable::{Drawable, Position};
use latex::latex_obj::LatexObj;
use image::PngImage;
use layout::{SplitPrec, Orientation};

use sdl2::rect::{Point, Rect};


fn main() {
    let scene = &mut MyScene::new();

    let mut wmng = WindowManager::init_window(scene, vec![]);

    wmng.start();
}

create_state! {
    MyState {
        Start,
        SubtitleDitherIn,
        Point1,
        Point2,
        Point3,
        Point4,
        FadePoints,
        Piano,
        DitherOut
    }
}

struct MyScene {
    title: ditherer::Ditherer<LatexObj>,
    subtitle: ditherer::Ditherer<LatexObj>,
    point1: ditherer::Ditherer<LatexObj>,
    point2: ditherer::Ditherer<LatexObj>,
    point3: ditherer::Ditherer<LatexObj>,
    point4: ditherer::Ditherer<LatexObj>,
    piano: ditherer::Ditherer<PngImage>,
    slut: ditherer::Ditherer<LatexObj>,
    state: MyState,
}

impl MyScene {
    fn new() -> MyScene {
        let mut title = ditherer::Ditherer::new(LatexObj::text(r#"\large Titeltext"#));
        title.start_dither();

        let subtitle = ditherer::Ditherer::new(LatexObj::text(r#"\small Undertitel här"#));

        let point1 = ditherer::Ditherer::new(LatexObj::text(r#"$ \cdot $ Punkt 1"#));
        let point2 = ditherer::Ditherer::new(LatexObj::text(r#"$ \cdot $ Punkt 2 - En till"#));
        let point3 = ditherer::Ditherer::new(LatexObj::text(r#"$ \cdot $ Punkt 3 - Inte slut än!"#));
        let point4 = ditherer::Ditherer::new(LatexObj::math(r#"\cdot e^{i\tau}=1"#));

        let piano = ditherer::Ditherer::new(PngImage::load_from_path(File::open("piano.png").unwrap()).unwrap());

        let slut = ditherer::Ditherer::new(LatexObj::text(r#"\huge The End"#));

        MyScene {
            title: title,
            subtitle: subtitle,
            point1: point1,
            point2: point2,
            point3: point3,
            point4: point4,
            piano: piano,
            slut: slut,
            state: MyState::Start,
        }
    }
}

impl Drawable for MyScene {
    fn content(&self) -> Vec<&dyn Drawable> {
        vec![
            &self.title,
            &self.subtitle,
            &self.point1,
            &self.point2,
            &self.point3,
            &self.point4,
            &self.piano,
            &self.slut
        ]
    }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![
            &mut self.title,
            &mut self.subtitle,
            &mut self.point1,
            &mut self.point2,
            &mut self.point3,
            &mut self.point4,
            &mut self.piano,
            &mut self.slut,
        ]
    }

    fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, _position: &Position) {
        let (width, height) = canvas.window().size();

        let mut title_subtitle = SplitPrec::new(0.5, Orientation::UpDown, &mut self.title, &mut self.subtitle);
        let mut title_piano = SplitPrec::new(0.5, Orientation::RightLeft, &mut title_subtitle, &mut self.piano);


        title_piano.draw(canvas, &Position::Rect(Rect::new(0, 0, width, height)));

        self.slut.draw(canvas, &Position::Center(Point::new(width as i32 / 2, height as i32 / 2)));
    }

}

impl Scene for MyScene {
    fn update(&mut self, dt: f64) -> scene::Action {
        self.as_mut_drawable().update(dt);
        scene::Action::Continue
    }

    fn event(&mut self, event: YEvent) -> scene::Action {
        match event {
            YEvent::Step { .. } => {
                if let Some(next) = self.state.next() {
                    self.state = next;

                    match self.state {
                        MyState::SubtitleDitherIn => {
                            self.subtitle.start_dither();
                        }
                        MyState::Point1 => {
                            self.point1.start_dither();
                        }
                        MyState::Point2 => {
                            self.point2.start_dither();
                        }
                        MyState::Point3 => {
                            self.point3.start_dither();
                        }
                        MyState::Point4 => {
                            self.point4.start_dither();
                        }
                        MyState::FadePoints => {
                            self.point1.fade_out();
                            self.point2.fade_out();
                            self.point3.fade_out();
                            self.point4.fade_out();
                        }
                        MyState::Piano => {
                            self.piano.start_dither();
                        }
                        MyState::DitherOut => {
                            self.piano.fade_out();
                            self.title.fade_out();
                            self.subtitle.fade_out();
                            self.slut.start_dither();
                        }
                        _ => {}
                    }
                } else {
                    return scene::Action::Next;
                }
            }
            YEvent::Next => {
                return scene::Action::Next;
            }
            _ => {}
        }
        scene::Action::Continue
    }
    fn as_drawable(&self) -> &dyn Drawable { self }
    fn as_mut_drawable(&mut self) -> &mut dyn Drawable { self }
}
