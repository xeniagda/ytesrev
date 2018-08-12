#[cfg(feature = "debug_boxes")]
const DRAW_BOXES: bool = true;
#[cfg(not(feature = "debug_boxes"))]
const DRAW_BOXES: bool = false;

pub mod split;
pub mod stack;
pub mod layered;

#[allow(unused)]
pub enum Orientation {
    Vertical, Horisontal,
}


