use crate::consts::{CANVAS_MIDDLE, CANVAS_SIZE, CIRCLE_POINT_COLOR, FADE_COLOR, POINT_COLOR};
#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn get_current_ctx_2d(canvas: &ElRef<HtmlCanvasElement>) -> CanvasRenderingContext2d {
    canvas_context_2d(&canvas.get().expect("get canvas element"))
}

pub fn draw_point(canvas: &ElRef<HtmlCanvasElement>, x: f64, y: f64, in_circle: bool) {
    let ctx = get_current_ctx_2d(&canvas);

    ctx.begin_path();
    ctx.arc(
        x * CANVAS_SIZE + CANVAS_MIDDLE,
        y * CANVAS_SIZE + CANVAS_MIDDLE,
        CANVAS_SIZE / 250.0,
        0.0,
        std::f64::consts::PI * 2.0,
    )
    .unwrap();
    ctx.set_fill_style(&JsValue::from_str(if in_circle {
        CIRCLE_POINT_COLOR
    } else {
        POINT_COLOR
    }));
    ctx.fill();
}

pub fn fade(canvas: &ElRef<HtmlCanvasElement>) {
    let ctx = get_current_ctx_2d(&canvas);

    ctx.set_fill_style(&JsValue::from_str(FADE_COLOR));
    ctx.fill_rect(0.0, 0.0, CANVAS_SIZE, CANVAS_SIZE);
}

pub fn clear(canvas: &ElRef<HtmlCanvasElement>) {
    let ctx = get_current_ctx_2d(&canvas);

    ctx.clear_rect(0.0, 0.0, CANVAS_SIZE, CANVAS_SIZE);
}
