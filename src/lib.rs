pub mod components;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_bevy::prelude::{
        to_cp437, BracketContext, ColorPair, Point, RandomNumbers, Rect as BRect, BLACK, WHITE,
    };
    pub use iyes_loopless::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const MAP_WIDTH: i32 = SCREEN_WIDTH;
    pub const MAP_HEIGHT: i32 = 43;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 1; //2
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 1; //2
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

pub struct HelloRglkPlugin;

impl Plugin for HelloRglkPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(AppState::Paused)
            .add_plugin(plugins::SpawnPlugin)
            .add_plugin(plugins::DevPlugin)
            .add_plugin(systems::SystemsPlugin)
            .add_startup_system(setup)
            .add_system(print_state_on_change)
            .add_system(print_camera_on_change);
    }
}

fn setup(mut cmds: Commands) {
    let mut rng = RandomNumbers::new();
    let schema = Schema::new(&mut rng);
    //cmds.spawn((Player, Position(schema.player_start)));
    cmds.spawn((Player, Position(schema.player_start)));
    cmds.insert_resource(schema.map);
    //cmds.insert_resource(BCamera::new(schema.player_start));
    info!("Setup finished.");
    cmds.insert_resource(NextState(AppState::AwaitInput));
}

//fn camera_follow(camera_q: Query<&mut Transform, With<Camera>>, player_q: Query<&Position, With<Player>>) {}

fn print_state_on_change(turn_state: Res<CurrentState<AppState>>) {
    if turn_state.is_changed() {
        info!("{:?}", turn_state);
    }
}

fn print_camera_on_change(camera_q: Query<&Transform, (With<Camera>, Changed<Transform>)>) {
    for camera_transform in camera_q.iter() {
        info!("{:?}", camera_transform);
    }
}
