use crate::prelude::*;

pub fn cls(ctx: Res<BracketContext>) {
    ctx.set_active_console(0);
    ctx.cls();
    ctx.set_active_console(1);
    ctx.cls();
    ctx.set_active_console(2);
    ctx.cls();
}
