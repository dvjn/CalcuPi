use crate::{
    canvas::{clear, draw_point, fade},
    helpers::is_in_circle,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use seed::prelude::{streams, web_sys::HtmlCanvasElement, ElRef, Orders, StreamHandle, Url};

pub struct Model {
    pub total_points: usize,
    pub points_in_circle: usize,
    pub random_generators: (SmallRng, SmallRng),
    pub simulation_timer_handle: Option<StreamHandle>,
    pub canvas: ElRef<HtmlCanvasElement>,
}

#[derive(Copy, Clone)]
pub enum Msg {
    StartSimulation,
    StopSimulation,
    AddRandomPoint,
    AddRandomPoints(usize),
    Reset,
}

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        total_points: 0,
        points_in_circle: 0,
        random_generators: (SmallRng::from_entropy(), SmallRng::from_entropy()),
        simulation_timer_handle: None,
        canvas: ElRef::default(),
    }
}

fn add_random_point(model: &mut Model) {
    let (x, y) = (
        model.random_generators.0.gen::<f64>() - 0.5,
        model.random_generators.1.gen::<f64>() - 0.5,
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
        Msg::StartSimulation => {
            model.simulation_timer_handle = Some(
                orders.stream_with_handle(streams::interval(50, || Msg::AddRandomPoints(100))),
            );
        }
        Msg::StopSimulation => {
            model.simulation_timer_handle = None;
        }
        Msg::AddRandomPoint => {
            add_random_point(model);
        }
        Msg::AddRandomPoints(n) => {
            (0..n).for_each(|_| add_random_point(model));
        }
        Msg::Reset => {
            model.total_points = 0;
            model.points_in_circle = 0;
            model.random_generators = (SmallRng::from_entropy(), SmallRng::from_entropy());
            model.simulation_timer_handle = None;
            clear(&model.canvas);
        }
    }
}
