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

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cls.label(ClsSystem)).add_system_set(
            ConditionSet::new()
                .label(DrawSystemSet)
                .after(ClsSystem)
                .with_system(draw_map)
                .with_system(draw_entity)
                .with_system(draw_ui)
                .into(),
        );
    }
}
