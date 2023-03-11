use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Position(pub Point);

impl Position {
    pub fn x(&self) -> i32 {
        self.0.x
    }

    pub fn y(&self) -> i32 {
        self.0.y
    }
}
