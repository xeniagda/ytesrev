//! The thing to be rendered every time

extern crate sdl2;

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
    /// Continue to the next slide
    Next,
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
#[allow(unused)]
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
            Action::Next
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

