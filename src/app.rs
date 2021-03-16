use crate::{
    canvas::{clear, draw_point, fade},
    helpers::is_in_circle,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use seed::prelude::{streams, web_sys::HtmlCanvasElement, ElRef, Orders, StreamHandle, Url};

pub struct Model {
    pub total_points: usize,
    pub points_in_circle: usize,
    pub generators: (SmallRng, SmallRng),
    pub auto_add_timer_handle: Option<StreamHandle>,
    pub canvas: ElRef<HtmlCanvasElement>,
}

#[derive(Copy, Clone)]
pub enum Msg {
    StartAutoAdd,
    StopAutoAdd,
    AddPoint,
    AddTenPoints,
    AddHundredPoints,
    AddThousandPoints,
    Reset,
}

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
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
    let in_circle = is_in_circle(x, y);

    model.total_points += 1;
    if in_circle {
        model.points_in_circle += 1;
    }

    draw_point(&model.canvas, x, y, in_circle);

    if model.total_points % 100 == 0 {
        fade(&model.canvas);
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
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
