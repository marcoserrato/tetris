pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    L,
    J,
    T,
    Square,
    Flat,
    Z,
    S
}

#[derive(Clone, Copy, Debug)]
pub struct WantsToMove {
    pub piece : Entity,
    pub destination: Point
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub shape : Shape
}

impl Piece {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ActivePiece;
