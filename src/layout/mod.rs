#[cfg(debug_assertions)]
const DRAW_BOXES: bool = true;
#[cfg(not(debug_assertions))]
const DRAW_BOXES: bool = false;

pub mod split;
pub mod stack;

#[allow(unused)]
pub enum Orientation {
    Vertical, Horisontal,
}


