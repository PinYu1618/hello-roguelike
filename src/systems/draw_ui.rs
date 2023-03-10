use crate::prelude::*;

pub fn draw_ui(ctx: Res<BracketContext>) {
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.target(UI_CONSOLE);
    draw_batch.print(Point::new(1, 1), "Hello.");
    ctx.submit_batch(10000, draw_batch);
}
