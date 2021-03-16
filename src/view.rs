use crate::{
    app::{Model, Msg},
    consts::CANVAS_SIZE,
    helpers::calculate_pi,
};
use num_format::{Locale, ToFormattedString};
#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

macro_rules! icon_button {
    ($icon:ident, $text:expr, $on_click:expr, $($rest:expr),*) => {{
        button![seed_icons::mi::default::$icon::i_c(vec![]), span![$text], ev(Ev::Click, $on_click), $($rest)*]
    }};
    ($icon:ident, $text:expr, $on_click:expr) => {
        icon_button!($icon, $text, $on_click,)
    };
}

macro_rules! simulation_speed_button {
    ($speed:literal, $simulation_speed:ident) => {
        icon_button![
            close,
            $speed,
            |_| Msg::SetSimulationSpeed($speed),
            C![
                "small",
                if $simulation_speed == $speed {
                    "primary"
                } else {
                    ""
                }
            ]
        ]
    };
}

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        main![
            view_header(),
            view_controls(
                model.simulation_timer_handle.is_some(),
                model.simulation_speed
            ),
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

fn view_controls(is_playing: bool, simulation_speed: usize) -> Node<Msg> {
    div![
        attrs!(At::Id => "controls"),
        h5!["Run Simulation"],
        div![
            C!["horizontal-group"],
            if is_playing {
                icon_button![pause, "Pause", |_| Msg::StopSimulation, C!["primary"]]
            } else {
                icon_button![play_arrow, "Play", |_| Msg::StartSimulation, C!["primary"]]
            },
            icon_button![replay, "Reset", |_| Msg::Reset, C!["secondary"]],
        ],
        h5!["Simulation Speed"],
        div![
            C!["horizontal-group"],
            simulation_speed_button!(1, simulation_speed),
            simulation_speed_button!(10, simulation_speed),
            simulation_speed_button!(100, simulation_speed),
        ],
        div![
            C!["horizontal-group"],
            simulation_speed_button!(1000, simulation_speed),
            simulation_speed_button!(10000, simulation_speed),
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
