use crate::prelude::*;

pub fn draw_entity(
    ctx: Res<BracketContext>,
    /*bcamera: Res<BCamera>,*/
    query: Query<(&Position, &Render)>,
) {
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.target(ENTITY_CONSOLE);
    let offset = Point::new(0, 0);

    query.iter().for_each(|(pos, render)| {
        draw_batch.set(pos.0 - offset, render.color, render.glyph);
    });
    ctx.submit_batch(5000, draw_batch);
}
