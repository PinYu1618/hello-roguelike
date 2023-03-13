mod cls;
mod draw_entity;
mod draw_map;
mod draw_ui;
mod input;

pub use self::cls::cls;
pub use self::draw_entity::draw_entity;
pub use self::draw_map::draw_map;
pub use self::draw_ui::draw_ui;

use crate::prelude::*;

#[derive(SystemLabel)]
pub struct DungeonUpdate;

#[derive(SystemLabel)]
pub struct TerminalUpdate;

#[derive(SystemLabel)]
pub struct ClsSystem;

#[derive(SystemLabel)]
pub struct DrawSystemSet;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .label(DungeonUpdate)
                .run_in_state(AppState::AwaitInput)
                .with_system(input::player_input)
                .into(),
        )
        .add_system(cls.label(ClsSystem).after(DungeonUpdate))
        .add_system_set(
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
