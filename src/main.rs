use bracket_bevy::BTermBuilder;
use hello_bracket_bevy::{prelude::*, GamePlugin};

fn main() {
    let bterm = BTermBuilder::empty()
        .with_random_number_generator(true)
        //.with_font("dungeonfont.png", 16, 16, (32.0, 32.0))
        .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
        .with_simple_console(0, DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_background(true)
        .with_simple_console(0, DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_background(false)
        .with_simple_console(0, SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2)
        .with_background(false);

    App::new()
        .add_plugin(bterm)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
