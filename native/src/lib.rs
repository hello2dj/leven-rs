#[macro_use]
extern crate neon;
extern crate quickcheck;

mod leven;

use leven::edit_distance;
use neon::prelude::*;

fn leven(mut ctx: FunctionContext) -> JsResult<JsNumber> {
    if ctx.len() != 2 {
      panic!("need 2 parameters");
    }
    let left = ctx.argument::<JsString>(0)?.value();
    let right = ctx.argument::<JsString>(1)?.value();
    // Ok(ctx.number(1))
    Ok(ctx.number(edit_distance(&left[..], &right[..]) as f64))
}

register_module!(mut cx, {
    cx.export_function("leven", leven)
});
