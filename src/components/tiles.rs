use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: u16,
}
