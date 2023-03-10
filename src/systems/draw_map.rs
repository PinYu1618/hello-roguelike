use crate::prelude::*;

pub fn draw_map(ctx: Res<BracketContext>, map: Res<Map> /*, #[resource] camera: &Camera*/) {
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.target(MAP_CONSOLE);
    for y in 0..=50 {
        for x in 0..80 {
            let pt = Point::new(x, y);
            let offset = Point::new(0, 0);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }
    ctx.submit_batch(0, draw_batch);
}
