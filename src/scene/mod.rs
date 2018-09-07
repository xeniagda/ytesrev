//! The thing to be rendered every time

extern crate sdl2;
extern crate rayon;


use std::mem;

use rayon::prelude::*;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use drawable::{DrawSettings, Drawable, Position, State};
use window::YEvent;

/// An action that allows the [`Scene`] to communicate with the [`WindowManager`]
///
/// [`WindowManager`]: ../window/struct.WindowManager.html
#[allow(unused)]
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
    fn draw(&mut self, canvas: &mut Canvas<Window>, settings: DrawSettings);
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

    fn draw(&mut self, canvas: &mut Canvas<Window>, settings: DrawSettings) {
        let (w, h) = canvas.window().size();
        self.0.draw(canvas, &Position::Rect(Rect::new(0, 0, w, h)), settings);
    }

    fn event(&mut self, event: YEvent) {
        match event {
            YEvent::Step => {
                self.0.step();
            }
            YEvent::Other(e) => {
                self.0.event(e);
            }
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
    current_scene: usize
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
    }
    fn draw(&mut self, canvas: &mut Canvas<Window>, settings: DrawSettings) {
        self.scenes[self.current_scene].draw(canvas, settings);
    }
    fn event(&mut self, event: YEvent) {
        self.scenes[self.current_scene].event(event);
    }
    fn action(&self) -> Action {
        self.scenes[self.current_scene].action()
    }
    fn register(&mut self) {
        for scene in &mut self.scenes {
            scene.register();
        }
    }
    fn load(&mut self) {
        let scenes = mem::replace(&mut self.scenes, Vec::new());

        self.scenes = scenes
            .into_par_iter()
            .enumerate()
            .map(|(i, mut scene)| {
                eprintln!("Loading scene {}...", i + 1);
                scene.load();
                scene
            }).collect::<Vec<Box<dyn Scene>>>();
    }
}
