use crate::prelude::*;

mod map;
mod schema;
mod tiletype;

pub use self::map::*;
pub use self::schema::Schema;
pub use self::tiletype::TileType;

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
