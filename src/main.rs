#![feature(duration_as_u128, nll, specialization)]

#[macro_use]
extern crate lazy_static;

extern crate sdl2;
extern crate png;

mod window;
mod image;
mod scene;
mod latex;
#[macro_use]
mod loadable;

use window::WindowManager;
use scene::{Scene, Drawable};
use latex::latex_obj::LatexObj;

use loadable::Loadable;

use sdl2::rect::Point;


fn main() {
    let scene = &mut MyScene::new();

    let mut wmng = WindowManager::init_window(scene, vec![]);

    wmng.start();
}

struct MyScene {
    dist: LatexObj,
    col: LatexObj,
    t: f64,
}

impl_loadable!{MyScene, dist, col}

impl MyScene {
    fn new() -> MyScene {
        let dist = LatexObj::new(r#"z^2 = x^2 + y^2"#);
        let col = LatexObj::new(r#"\frac{\textcolor{green}x}{\sqrt{x^2 + y^2}}"#);

        MyScene {
            dist: dist,
            col: col,
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
