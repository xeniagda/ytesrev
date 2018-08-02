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
    title: ditherer::Ditherer<LatexObj>,
    col: LatexObj,
    t: f64,
}

impl_loadable!{MyScene, title, col}

impl MyScene {
    fn new() -> MyScene {
        let title = ditherer::Ditherer::new(LatexObj::new(r#"\text{Dithering sample text}"#));
        let col = LatexObj::new(r#"\frac{\textcolor{green}x}{\sqrt{x^2 + y^2}}"#);

        MyScene {
            title: title,
            col: col,
            t: 0.,
        }
    }
}


impl Scene for MyScene {
    fn update(&mut self, dt: f64) -> scene::Action {
        self.t += dt;
        self.title.update(dt);
        self.col.update(dt);

        scene::Action::Continue
    }
    fn event(&mut self, _event: sdl2::event::Event) -> scene::Action {
        scene::Action::Continue
    }
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let point = Point::new(
                (((self.t * 0.8).sin() / 4. + 0.5) * canvas.window().size().0 as f64) as i32,
                (((self.t * 1.3).sin() / 4. + 0.5) * canvas.window().size().1 as f64) as i32);


        self.title.draw(canvas, &Point::new(100, 100));

        self.col.draw(canvas, &point);
    }
}
