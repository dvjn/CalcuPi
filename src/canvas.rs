#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Color<'a> {
    light: &'a str,
    dark: &'a str,
}

impl<'a> Color<'a> {
    const fn get(&self, prefers_dark_mode: bool) -> &str {
        if prefers_dark_mode {
            self.dark
        } else {
            self.light
        }
    }
}

pub const CANVAS_SIZE: f64 = 1000.0;
pub const CANVAS_MIDDLE: f64 = CANVAS_SIZE / 2.0;
pub const POINT_COLOR: Color = Color {
    dark: "#eeeeee",
    light: "#333333",
};
pub const CIRCLE_POINT_COLOR: Color = Color {
    dark: "#001acd",
    light: "#304ffe",
};
pub const FADE_COLOR: Color = Color {
    dark: "rgba(0, 0, 0, 0.0075)",
    light: "rgba(255, 255, 255, 0.0025)",
};

fn get_current_ctx_2d(canvas: &ElRef<HtmlCanvasElement>) -> CanvasRenderingContext2d {
    canvas_context_2d(&canvas.get().expect("get canvas element"))
}

pub fn draw_point(
    canvas: &ElRef<HtmlCanvasElement>,
    x: f64,
    y: f64,
    in_circle: bool,
    prefers_dark_mode: bool,
) {
    let ctx = get_current_ctx_2d(canvas);

    ctx.begin_path();
    ctx.arc(
        x.mul_add(CANVAS_SIZE, CANVAS_MIDDLE),
        y.mul_add(CANVAS_SIZE, CANVAS_MIDDLE),
        CANVAS_SIZE / 250.0,
        0.0,
        std::f64::consts::PI * 2.0,
    )
    .unwrap();
    ctx.set_fill_style(&JsValue::from_str(if in_circle {
        CIRCLE_POINT_COLOR.get(prefers_dark_mode)
    } else {
        POINT_COLOR.get(prefers_dark_mode)
    }));
    ctx.fill();
}

pub fn fade(canvas: &ElRef<HtmlCanvasElement>, prefers_dark_mode: bool) {
    let ctx = get_current_ctx_2d(canvas);

    ctx.set_fill_style(&JsValue::from_str(FADE_COLOR.get(prefers_dark_mode)));
    ctx.fill_rect(0.0, 0.0, CANVAS_SIZE, CANVAS_SIZE);
}

pub fn clear(canvas: &ElRef<HtmlCanvasElement>) {
    let ctx = get_current_ctx_2d(canvas);

    ctx.clear_rect(0.0, 0.0, CANVAS_SIZE, CANVAS_SIZE);
}
