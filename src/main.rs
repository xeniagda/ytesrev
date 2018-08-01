#![feature(duration_as_u128, nll)]

#[macro_use]
extern crate lazy_static;

extern crate sdl2;
extern crate png;

mod window;
mod image;
mod scene;
mod latex;
use window::WindowManager;
use scene::{Scene, Drawable};

use sdl2::rect::Point;


fn main() {
    let scene = &mut MyScene::new();

    let mut wmng = WindowManager::init_window(scene);

    wmng.start();
}

struct MyScene {
    dist: image::PngImage,
    col: image::PngImage,
    t: f64,
}

impl MyScene {
    fn new() -> MyScene {
        let dist_i = latex::register_equation(r#"z^2 = x^2 + y^2"#);
        let col_i = latex::register_equation(r#"\frac{\textcolor{green}x}{\sqrt{x^2 + y^2}}"#);

        latex::render_all_eqations();

        MyScene {
            dist: latex::read_image(dist_i).unwrap(),
            col: latex::read_image(col_i).unwrap(),
            t: 0.,
        }
    }
}

impl Scene for MyScene {
    fn update(&mut self, dt: f64) -> scene::Action {
        self.t += dt;
        scene::Action::Continue
    }
    fn event(&mut self, _event: sdl2::event::Event) -> scene::Action {
        scene::Action::Continue
    }
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let point = Point::new(
                (((self.t * 0.8).sin() / 2. + 0.5) * canvas.window().size().0 as f64) as i32,
                (((self.t * 1.3).sin() / 2. + 0.5) * canvas.window().size().1 as f64) as i32);
        if self.t % 2. > 1. {
            self.dist.draw(canvas, &point);
        } else {
            self.col.draw(canvas, &point);
        }
    }
}
