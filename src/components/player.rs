use bracket_bevy::prelude::{to_cp437, BLACK, YELLOW};

use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub render: Render,
}

impl Player {
    pub fn spawn(mut cmds: Commands, query: Query<Entity, Added<Player>>) {
        for e in query.iter() {
            cmds.entity(e).insert(PlayerBundle {
                render: Render {
                    color: ColorPair::new(YELLOW, BLACK),
                    glyph: to_cp437('@'),
                },
            });
            info!("Spawned new player.");
        }
    }
}
