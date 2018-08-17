//! Positioning objects on the screen

pub mod layered;
pub mod split;
pub mod stack;

/// In what direction something is facing
pub enum Orientation {
    /// Like this:
    /// ```
    ///    ^
    ///    |
    ///    |
    ///    |
    /// ```
    Vertical,
    /// Like this:
    /// ```
    /// --->
    /// ```
    Horizontal,
}
