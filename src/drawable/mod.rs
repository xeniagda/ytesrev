//! Contains utilities for where and how to draw things

extern crate sdl2;

use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Where to draw a specific object.
#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Position {
    /// Place the top left corner at a specific position, ignoring size. Only used in the most
    /// primitive situations
    TopLeftCorner(Point),
    /// Center the object at this position
    Center(Point),
    /// Draw the object in a specific rectangle
    Rect(Rect),
}

/// Settings to specify how an object is supposed to be drawed
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DrawSettings {
    /// Is the object being drawed in the notes window, or the main window? In the notes window,
    /// more debug information is drawed.
    pub notes_view: bool,
}

/// The default settings for the main window
pub const SETTINGS_MAIN: DrawSettings = DrawSettings { notes_view: false };

/// The default settings for the notes window
pub const SETTINGS_NOTES: DrawSettings = DrawSettings { notes_view: true };

/// The state for a specific object on screen, used in the [`Drawable::state()`]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum State {
    /// The object is still in the process of being shown, such as fading in.
    Working,
    /// The object is complete and the next call to step will start to hide it
    Final,
    /// The object is being hidden
    Hidden,
}

impl Position {
    /// Convert a `Position` into a Rect with the specified size. The size is not gauranteed to be
    /// (width, height) such as when the `Position` is a `Position::Rect` with a smaller size
    ///
    /// For `Position::Rect` the center is preserved, but the size is changed.
    ///
    /// ```
    /// use ytesrev::{Point, Rect};
    /// use ytesrev::drawable::Position;
    ///
    /// let tlc = Position::TopLeftCorner(Point::new(10, 20));
    /// assert_eq!(tlc.into_rect_with_size(5, 8), Rect::new(10, 20, 5, 8));
    ///
    /// let center = Position::Center(Point::new(20, 30));
    /// assert_eq!(center.into_rect_with_size(4, 10), Rect::new(18, 25, 4, 10));
    ///
    /// let rect = Position::Rect(Rect::new(10, 40, 20, 20));
    /// assert_eq!(rect.into_rect_with_size(6, 4), Rect::new(17, 48, 6, 4));
    ///
    /// let rect_too_short = Position::Rect(Rect::new(10, 20, 30, 8));
    /// assert_eq!(rect_too_short.into_rect_with_size(20, 69), Rect::new(15, 20, 20, 8));
    ///
    /// ```
    pub fn into_rect_with_size(self, width: u32, height: u32) -> Rect {
        match self {
            Position::TopLeftCorner(point) => Rect::new(point.x, point.y, width, height),
            Position::Center(point) => Rect::new(
                point.x - width as i32 / 2,
                point.y - height as i32 / 2,
                width,
                height,
            ),
            Position::Rect(rect) => {
                let center_x = rect.x() + rect.width() as i32 / 2;
                let center_y = rect.y() + rect.height() as i32 / 2;

                let x = (center_x - width  as i32 / 2).max(rect.x);
                let y = (center_y - height as i32 / 2).max(rect.y);

                let width =
                    if x + width as i32 > rect.x + rect.width() as i32 {
                        rect.width()
                    } else {
                        width
                    };

                let height =
                    if y + height as i32 > rect.y + rect.height() as i32 {
                        rect.height()
                    } else {
                        height
                    };

                Rect::new(x, y, width, height)
            }
        }
    }

    /// Convert a Position into a Rect, like [`Position::into_rect_with_size`], but this doesn't
    /// check if the [`Position::Rect`] is too small.
    /// ```
    /// use ytesrev::{Point, Rect};
    /// use ytesrev::drawable::Position;
    ///
    /// let tlc = Position::TopLeftCorner(Point::new(20, 50));
    /// assert_eq!(tlc.into_rect_with_size(2, 7), Rect::new(20, 50, 2, 7));
    ///
    /// let center = Position::Center(Point::new(50, 10));
    /// assert_eq!(center.into_rect_with_size(2, 8), Rect::new(49, 6, 2, 8));
    ///
    /// let rect = Position::Rect(Rect::new(10, 40, 20, 20));
    /// assert_eq!(rect.into_rect_with_size(6, 4), Rect::new(17, 48, 6, 4));
    ///
    /// ```
    pub fn into_rect_with_size_unbounded(self, width: u32, height: u32) -> Rect {
        match self {
            Position::TopLeftCorner(point) => {
                Rect::new(point.x, point.y, width, height)
            }
            Position::Center(point) => {
                Rect::new(point.x - width as i32 / 2, point.y - height as i32 / 2, width, height)
            }
            Position::Rect(rect) => {
                let center_x = rect.x() + rect.width() as i32 / 2;
                let center_y = rect.y() + rect.height() as i32 / 2;

                let x = (center_x - width  as i32 / 2).max(rect.x);
                let y = (center_y - height as i32 / 2).max(rect.y);

                Rect::new(x, y, width, height)
            }
        }
    }
}

/// An object that can be drawn
pub trait Drawable: Send {
    /// What this object contains
    fn content(&self) -> Vec<&dyn Drawable>;
    /// What this object contains, mutably
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable>;

    /// Register all content. This is mostly just used by [`LatexObj`]s, that need to be
    /// registered before loaded.
    ///
    /// [`LatexObj`]: ../latex/latex_obj/struct.LatexObj.rs
    fn register(&mut self) {
        for content in &mut self.content_mut() {
            content.register();
        }
    }

    /// Load all content
    fn load(&mut self) {
        for content in &mut self.content_mut() {
            content.load();
        }
    }

    /// When the user presses space, the state of the presentation is advanced. This
    /// method is what is called.
    fn step(&mut self);

    /// What state the object is in
    fn state(&self) -> State;

    /// Tick the object
    fn update(&mut self, dt: f64) {
        for content in &mut self.content_mut() {
            content.update(dt);
        }
    }

    /// Draw everything
    fn draw(&mut self, _canvas: &mut Canvas<Window>, _position: &Position, _settings: DrawSettings);
}

/// An object that has a determined size, like an image, but not a solid that can fit any
/// shape it's called with
pub trait KnownSize: Drawable {
    /// The width of the object

    fn width(&self)  -> usize;
    /// The height of the object
    fn height(&self) -> usize;
}

