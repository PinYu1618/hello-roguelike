pub mod components;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_bevy::prelude::{BracketContext, ColorPair, Point, RandomNumbers};
    pub use iyes_loopless::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const MAP_CONSOLE: usize = 0;
    pub const ENTITY_CONSOLE: usize = 1;
    pub const UI_CONSOLE: usize = 2;
    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::resources::*;
    pub use crate::states::*;
    pub use crate::systems::*;
}

use self::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(TurnState::Paused)
            .add_plugin(plugins::SpawnPlugin)
            .add_system_set(ConditionSet::new().with_system(draw_entity).into())
            .add_startup_system(setup);
    }
}

fn setup(mut cmds: Commands) {
    let mut rng = RandomNumbers::new();
    let schema = Schema::new(&mut rng);
    cmds.spawn((Player, Position(Point::new(40, 25))));
    cmds.insert_resource(schema.map);
}
