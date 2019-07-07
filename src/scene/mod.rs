//! The thing to be rendered every time

extern crate rayon;
extern crate sdl2;

use std::sync::mpsc::channel;
use std::thread::spawn;

use rayon::scope;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use drawable::{DrawSettings, Drawable, Position, State};
use window::YEvent;

/// An action that allows the [`Scene`] to communicate with the [`WindowManager`]
///
/// [`WindowManager`]: ../window/struct.WindowManager.html
#[allow(unused)]
#[derive(PartialEq)]
pub enum Action {
    /// Don't do anything
    Continue,
    /// The current scene is done, move on to the next one
    Done,
}

/// A scene is like a [`Drawable`], but without the modularity, it's in the top level and
/// no wrappers for it exists. Unless you're doing something advanced, a
/// [`DrawableWrapper`] around your [`Drawable`]s should do fine.
///
/// [`Drawable`]: ../drawable/struct.Drawable.html
pub trait Scene: Send {
    /// Do a tick
    fn update(&mut self, _dt: f64);
    /// Draw the content of this scene to a `Canvas`.
    fn draw(&self, canvas: &mut Canvas<Window>, settings: DrawSettings);
    /// Called when an event occured
    fn event(&mut self, _event: YEvent);
    /// What to do
    fn action(&self) -> Action;
    /// Register everything. The scene equivalent of [`Drawable::register`]
    ///
    /// [`Drawable::register`]: ../drawable/struct.Drawable.html#method.register
    fn register(&mut self);
    /// Load everything. The scene equivalent of [`Drawable::load`]
    ///
    /// [`Drawable::load`]: ../drawable/struct.Drawable.html#method.register
    fn load(&mut self);
}

/// A wrapper to make a [`Drawable`] into a [`Scene`]. This is probably all you will need
/// in terms of scenes, there's really not much else you can do with one.
///
/// [`Drawable`]: ../drawable/struct.Drawable.html
pub struct DrawableWrapper<T: Drawable>(pub T);

impl<T: Drawable> Scene for DrawableWrapper<T> {
    fn update(&mut self, dt: f64) {
        self.0.update(dt);
    }

    fn draw(&self, canvas: &mut Canvas<Window>, settings: DrawSettings) {
        let (w, h) = canvas.window().size();
        self.0
            .draw(canvas, &Position::Rect(Rect::new(0, 0, w, h)), settings);
    }

    fn event(&mut self, event: YEvent) {
        match event {
            YEvent::Step => {
                self.0.step();
            }
            YEvent::Other(e) => {
                self.0.event(e);
            }
            YEvent::StepSlide => {}
        }
    }

    fn action(&self) -> Action {
        if self.0.state() == State::Hidden {
            Action::Done
        } else {
            Action::Continue
        }
    }

    fn register(&mut self) {
        self.0.register()
    }
    fn load(&mut self) {
        self.0.load()
    }
}

/// A list of scenes that are showed in order. When the current scene's action is [`Action::Done`]
/// the next scene is loaded.
pub struct SceneList {
    /// The list of scenes
    pub scenes: Vec<Box<dyn Scene>>,

    /// The index of the current scene being showed. Garuanteed to always be in bounds
    current_scene: usize,
}

impl SceneList {
    /// Create a new SceneList
    pub fn new(scenes: Vec<Box<dyn Scene>>) -> SceneList {
        SceneList {
            scenes,
            current_scene: 0,
        }
    }

    /// Gets what scene is being shown
    pub fn get_current_scene(&self) -> usize {
        self.current_scene
    }
}

impl Scene for SceneList {
    fn update(&mut self, dt: f64) {
        self.scenes[self.current_scene].update(dt);

        if self.scenes[self.current_scene].action() == Action::Done {
            self.current_scene += 1;
        }
    }
    fn draw(&self, canvas: &mut Canvas<Window>, settings: DrawSettings) {
        self.scenes[self.current_scene].draw(canvas, settings);
    }
    fn event(&mut self, event: YEvent) {
        match event {
            YEvent::StepSlide => self.current_scene += 1,
            _ => {
                self.scenes[self.current_scene].event(event);
            }
        }
    }
    fn action(&self) -> Action {
        if self.current_scene >= self.scenes.len() {
            Action::Done
        } else {
            Action::Continue
        }
    }
    fn register(&mut self) {
        for scene in &mut self.scenes {
            scene.register();
        }
    }

    fn load(&mut self) {
        let nscenes = self.scenes.len();

        let (tx, rx) = channel::<usize>();

        spawn(move || {
            let mut statuses = vec![0; nscenes]; // 0 = not loaded, 1 = loading, 2 = loaded
            print_state(&statuses);
            let mut count = 0;
            while let Ok(idx) = rx.recv() {
                count += 1;
                statuses[idx] += 1;
                eprint!("\x1b[{}A", nscenes); // Clear
                print_state(&statuses);
                if count >= 2 * nscenes {
                    break;
                }
            }
        });

        let s = &mut self.scenes;

        scope(move |sc| {
            for (i, scene) in s.iter_mut().enumerate() {
                let send = tx.clone();

                sc.spawn(move |_| {
                    send.send(i).unwrap();
                    scene.load();
                    send.send(i).unwrap();
                });
            }
        });
    }
}

fn print_state(statuses: &[u8]) {
    for (i, status) in statuses.iter().enumerate() {
        eprint!("\x1b[KScene {}: ", i + 1);
        match status {
            0 => eprintln!("..."),
            1 => eprintln!("Loading"),
            2 => eprintln!("Done"),
            _ => eprintln!("o no! {}", status),
        }
    }
}
