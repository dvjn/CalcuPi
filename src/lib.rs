#![allow(clippy::wildcard_imports)]

use num_format::{Locale, ToFormattedString};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use seed::{prelude::*, *};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const CANVAS_SIZE: f64 = 1000.0;
const CANVAS_MIDDLE: f64 = CANVAS_SIZE / 2.0;
const POINT_COLOR: &str = "#333333";
const CIRCLE_POINT_COLOR: &str = "#304ffe";
const FADE_COLOR: &str = "rgba(255, 255, 255, 0.005)";

struct Model {
    total_points: usize,
    points_in_circle: usize,
    generators: (SmallRng, SmallRng),
    auto_add_timer_handle: Option<StreamHandle>,
    canvas: ElRef<HtmlCanvasElement>,
}

#[derive(Copy, Clone)]
enum Msg {
    StartAutoAdd,
    StopAutoAdd,
    AddPoint,
    AddTenPoints,
    AddHundredPoints,
    AddThousandPoints,
    Reset,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        total_points: 0,
        points_in_circle: 0,
        generators: (SmallRng::from_entropy(), SmallRng::from_entropy()),
        auto_add_timer_handle: None,
        canvas: ElRef::default(),
    }
}

fn add_random_point(model: &mut Model) {
    let (x, y) = (
        model.generators.0.gen::<f64>() - 0.5,
        model.generators.1.gen::<f64>() - 0.5,
    );
    let in_circle = x * x + y * y <= 0.25;

    model.total_points += 1;
    if in_circle {
        model.points_in_circle += 1;
    }

    draw_point(&model.canvas, x, y, in_circle);

    if model.total_points % 100 == 0 {
        fade(&model.canvas);
    }
}

fn get_current_ctx_2d(canvas: &ElRef<HtmlCanvasElement>) -> CanvasRenderingContext2d {
    canvas_context_2d(&canvas.get().expect("get canvas element"))
}

fn draw_point(canvas: &ElRef<HtmlCanvasElement>, x: f64, y: f64, in_circle: bool) {
    let ctx = get_current_ctx_2d(&canvas);

    ctx.begin_path();
    ctx.arc(
        x * CANVAS_SIZE + CANVAS_MIDDLE,
        y * CANVAS_SIZE + CANVAS_MIDDLE,
        CANVAS_SIZE / 200.0,
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

fn fade(canvas: &ElRef<HtmlCanvasElement>) {
    let ctx = get_current_ctx_2d(&canvas);

    ctx.set_fill_style(&JsValue::from_str(FADE_COLOR));
    ctx.fill_rect(0.0, 0.0, CANVAS_SIZE, CANVAS_SIZE);
}

fn clear(canvas: &ElRef<HtmlCanvasElement>) {
    let ctx = get_current_ctx_2d(&canvas);

    ctx.clear_rect(0.0, 0.0, CANVAS_SIZE, CANVAS_SIZE);
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartAutoAdd => {
            model.auto_add_timer_handle =
                Some(orders.stream_with_handle(streams::interval(50, || Msg::AddHundredPoints)));
        }
        Msg::StopAutoAdd => {
            model.auto_add_timer_handle = None;
        }
        Msg::AddPoint => {
            add_random_point(model);
        }
        Msg::AddTenPoints => {
            (0..10).for_each(|_| add_random_point(model));
        }
        Msg::AddHundredPoints => {
            (0..100).for_each(|_| add_random_point(model));
        }
        Msg::AddThousandPoints => {
            (0..1000).for_each(|_| add_random_point(model));
        }
        Msg::Reset => {
            model.total_points = 0;
            model.points_in_circle = 0;
            model.generators = (SmallRng::from_entropy(), SmallRng::from_entropy());
            model.auto_add_timer_handle = None;
            clear(&model.canvas);
        }
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        main![
            view_header(),
            view_controls(model.auto_add_timer_handle.is_some()),
            view_results(model.total_points, model.points_in_circle)
        ],
        div![
            attrs!(At::Id => "visualization"),
            canvas![
                el_ref(&model.canvas),
                attrs![
                    At::Width => px(CANVAS_SIZE),
                    At::Height => px(CANVAS_SIZE),
                ],
            ],
        ]
    ]
}

fn view_header() -> Node<Msg> {
    header![h1!["CalcuPi"], h3!["Monte Carlo Method"]]
}

macro_rules! icon_button {
    ($icon:ident, $text:expr, $on_click:expr, $($rest:expr),*) => {{
        use seed_icons::mi::default::$icon;
        button![$icon::i_c(vec![]), span![$text], ev(Ev::Click, $on_click), $($rest)*]
    }};
    ($icon:ident, $text:expr, $on_click:expr) => {
        icon_button!($icon, $text, $on_click,)
    };
}

fn view_controls(is_playing: bool) -> Node<Msg> {
    div![
        attrs!(At::Id => "controls"),
        h5!["Run Simulation"],
        div![
            C!["horizontal-group"],
            if is_playing {
                icon_button![pause, "Pause", |_| Msg::StopAutoAdd, C!["primary"]]
            } else {
                icon_button![play_arrow, "Play", |_| Msg::StartAutoAdd, C!["primary"]]
            },
            icon_button![replay, "Reset", |_| Msg::Reset, C!["secondary"]],
        ],
        h5!["Manually Add Points"],
        div![
            C!["horizontal-group"],
            icon_button![add, "1", |_| Msg::AddPoint, C!["small"]],
            icon_button![add, "10", |_| Msg::AddTenPoints, C!["small"]],
            icon_button![add, "100", |_| Msg::AddHundredPoints, C!["small"]],
            icon_button![add, "1000", |_| Msg::AddThousandPoints, C!["small"]],
        ],
    ]
}

fn view_results(total_points: usize, points_in_circle: usize) -> Node<Msg> {
    let pi = calculate_pi(total_points, points_in_circle);

    div![
        attrs!(At::Id => "results"),
        h5!["Results"],
        div![
            C!["horizontal-group"],
            div![
                C!["infobox"],
                caption!["Total Points"],
                div![C!["spacer"]],
                div![span!["S = "], total_points.to_formatted_string(&Locale::en)]
            ],
            div![
                C!["infobox"],
                caption!["Inside Circle"],
                div![C!["spacer"]],
                div![
                    span!["C = "],
                    points_in_circle.to_formatted_string(&Locale::en)
                ]
            ],
        ],
        div![
            C!["horizontal-group"],
            div![
                C!["infobox"],
                caption!["Calculated Pi"],
                match pi {
                    Some(pi) => div![
                        span!["4*C/S = "],
                        format!("{:.5}", pi,),
                        span![format!("({:+.5})", std::f64::consts::PI - pi)]
                    ],
                    None => div!["Not calculated"],
                }
            ],
        ],
    ]
}

fn calculate_pi(total_points: usize, points_in_circle: usize) -> Option<f64> {
    if total_points == 0 {
        None
    } else {
        Some((points_in_circle as f64) / (total_points as f64) * 4.0)
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
