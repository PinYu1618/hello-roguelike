use bracket_bevy::BTermBuilder;
use hello_roguelike::{prelude::*, HelloRglkPlugin};

fn main() {
    let bterm = BTermBuilder::empty()
        .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
        .with_simple_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_background(true)
        .with_simple_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_background(false)
        .with_simple_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_background(false);

    App::new()
        .add_plugin(bterm)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                canvas: Some("canvas".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(HelloRglkPlugin)
        .run();
}
