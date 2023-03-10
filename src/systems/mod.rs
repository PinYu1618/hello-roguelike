mod cls;
mod draw_entity;
mod draw_map;
mod draw_ui;

pub use self::cls::cls;
pub use self::draw_entity::draw_entity;
pub use self::draw_map::draw_map;
pub use self::draw_ui::draw_ui;

use crate::prelude::*;

#[derive(SystemLabel)]
pub struct ClsSystem;

#[derive(SystemLabel)]
pub struct DrawSystemSet;
