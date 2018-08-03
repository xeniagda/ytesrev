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
#[macro_use]
mod loadable;
#[macro_use]
mod state;

use window::{WindowManager, YEvent};
use scene::{Scene, Drawable};
use latex::latex_obj::LatexObj;

use loadable::Loadable;
use scene::Position;
use image::ImageContainer;

use sdl2::rect::Point;


fn main() {
    let scene = &mut MyScene::new();

    let mut wmng = WindowManager::init_window(scene, vec![]);

    wmng.start();
}

create_state! {
    MyState {
        Start,
        SubtitleDitherIn,
        SubtitleDitherOut
    }
}

struct MyScene {
    title: ditherer::Ditherer<LatexObj>,
    subtitle: ditherer::Ditherer<LatexObj>,
    col: LatexObj,
    t: f64,
    state: MyState,
}

impl_loadable!{MyScene, title, subtitle, col}

impl MyScene {
    fn new() -> MyScene {
        let mut title = ditherer::Ditherer::new(LatexObj::text(r#"\large Title text"#));
        title.start_dither();

        let subtitle = ditherer::Ditherer::new(LatexObj::text(r#"\small Subtitle"#));
        let col = LatexObj::math(r#"\frac{\textcolor{green}x}{\sqrt{x^2 + y^2}}"#);

        MyScene {
            title: title,
            subtitle: subtitle,
            col: col,
            t: 0.,
            state: MyState::Start,
        }
    }
}


impl Scene for MyScene {
    fn update(&mut self, dt: f64) -> scene::Action {
        self.t += dt;
        self.title.update(dt);
        self.subtitle.update(dt);
        self.col.update(dt);

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
                        MyState::SubtitleDitherOut => {
                            self.subtitle.fade_out();
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
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let (width, height) = canvas.window().size();

        let point = Point::new(
                (((self.t * 0.8).sin() / 4. + 0.5) * width as f64) as i32,
                (((self.t * 1.3).sin() / 4. + 0.5) * height as f64) as i32);


        self.title.draw(canvas, &Position::Center(Point::new(width as i32 / 2, height as i32 / 4)));
        self.subtitle.draw(
            canvas,
            &Position::Center(Point::new(width as i32 / 2, height as i32 / 4 + self.title.height() as i32))
        );

        self.col.draw(canvas, &Position::Center(point));
    }
}
