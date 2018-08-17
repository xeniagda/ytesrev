//! The thing to be rendered every time

extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;

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
pub trait Scene {
    /// Do a tick
    fn update(&mut self, _dt: f64);
    /// Called when an event occured
    fn event(&mut self, _event: YEvent);
    /// What to do
    fn action(&self) -> Action;

    /// Convert to a drawable
    fn as_drawable(&self) -> &dyn Drawable;
    /// Convert to a mutable drawable
    fn as_mut_drawable(&mut self) -> &mut dyn Drawable;
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
    fn event(&mut self, event: YEvent) {
        match event {
            YEvent::Step => {
                self.0.step();
            }
            _ => {}
        }
    }

    fn action(&self) -> Action {
        if self.0.state() == State::Hidden {
            Action::Next
        } else {
            Action::Continue
        }
    }

    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
    fn as_mut_drawable(&mut self) -> &mut dyn Drawable {
        self
    }
}

impl<T: Drawable> Drawable for DrawableWrapper<T> {
    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &Position, settings: DrawSettings) {
        self.0.draw(canvas, position, settings);
    }

    fn step(&mut self) {
        self.0.step();
    }

    fn state(&self) -> State {
        self.0.state()
    }

    fn content(&self) -> Vec<&dyn Drawable> {
        vec![&self.0]
    }

    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![&mut self.0]
    }
}
