// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

mod cube;
mod cubeset;
mod cubic_calc;
mod draw_manager;

use cube::CubeColor;
use cube::ROTATE_STEP;
use cubeset::CubeSet;
use cubeset::RotateAxis;
use cubeset::RotateLayer;
use cubeset::RotateTarget;
use cubic_calc::PI;
use cubic_calc::CameraVec;
use cubic_calc::ViewFrustum;
use cubic_calc::CameraModel;
use draw_manager::draw_cubeset;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    Model { 
        counter: 0,
        animaion_speed: 50,
        animation_rotate_target: RotateTarget::default(),
        animation_countdown: 0,
        cubeset: CubeSet::default(),
        camera: CameraModel::default(),
        canvas: ElRef::<HtmlCanvasElement>::default(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
    animaion_speed: i32,
    animation_rotate_target: RotateTarget,
    animation_countdown: i32,
    cubeset: CubeSet,
    camera: CameraModel,
    canvas: ElRef<HtmlCanvasElement>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    AnimationSpeedChanged(String),
    Rotate(RotateTarget),
    Rendered,
    ResetPosition,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AnimationSpeedChanged(input_val) => {
            model.animaion_speed = input_val.parse().unwrap();;
        },
        Msg::Rotate(rotate_target) => {
            model.animation_countdown = (900 / model.animaion_speed) as i32;
            model.animation_rotate_target = RotateTarget {
                axis: rotate_target.axis,
                layer: rotate_target.layer,
                rad: PI / (2 * model.animation_countdown) as f32
            };
        },
        Msg::ResetPosition => {
            model.counter = model.animaion_speed;
            model.cubeset = CubeSet::default();
        },
        Msg::Rendered => {
            draw(model);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            if model.animation_countdown > 0 {
                model.animation_countdown -= 1;
                model.cubeset.rotate_layer(&model.animation_rotate_target);
                orders.after_next_render(|_| Msg::Rendered);
            } else {
                orders.after_next_render(|_| Msg::Rendered).skip();
            }
        }
    }
}

fn draw(model: &Model) {
    let canvas_element = model.canvas.get().expect("get canvas element");

    draw_cubeset(&canvas_element, &model.cubeset, &model.camera);
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        style! {St::Display => "flex"},
        div![
            attrs![
                At::Width => px(300),
                At::Height => px(500),
            ],
            div![
                label!["動作速度"],
                input![
                    attrs! {
                        At::Width => px(50),
                        At::Value => model.animaion_speed,
                        At::Type => "number",
                    },
                    input_ev(Ev::Input, Msg::AnimationSpeedChanged),
                ],
            ],
            div![
                label!["X軸回転"],
                div![
                    label!["前"],
                    button![
                        "x(+)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::X,
                            layer: RotateLayer::Positive,
                            rad: ROTATE_STEP
                        })),
                    ],
                    label!["中"],
                    button![
                        "x(0)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::X,
                            layer: RotateLayer::Neutral,
                            rad: ROTATE_STEP
                        })),
                    ],
                    label!["奥"],
                    button![
                        "x(-)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::X,
                            layer: RotateLayer::Negative,
                            rad: ROTATE_STEP
                        })),
                    ],
                ]
            ],
            div![
                label!["Y軸回転"],
                div![
                    label!["上"],
                    button![
                        "y(+)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::Y,
                            layer: RotateLayer::Positive,
                            rad: ROTATE_STEP
                        })),
                    ],
                    label!["中"],
                    button![
                        "y(0)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::Y,
                            layer: RotateLayer::Neutral,
                            rad: ROTATE_STEP
                        })),
                    ],
                    label!["下"],
                    button![
                        "y(-)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::Y,
                            layer: RotateLayer::Negative,
                            rad: ROTATE_STEP
                        })),
                    ],
                ]
            ],
            div![
                label!["Z軸回転"],
                div![
                    label!["前"],
                    button![
                        "z(+)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::Z,
                            layer: RotateLayer::Positive,
                            rad: ROTATE_STEP
                        })),
                    ],
                    label!["中"],
                    button![
                        "z(0)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::Z,
                            layer: RotateLayer::Neutral,
                            rad: ROTATE_STEP
                        })),
                    ],
                    label!["奥"],
                    button![
                        "z(-)",
                        ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                            axis: RotateAxis::Z,
                            layer: RotateLayer::Negative,
                            rad: ROTATE_STEP
                        })),
                    ],
                ]
            ],
            button![
                "Reset",
                style![
                    St::BackgroundColor => CubeColor::Lime.as_css_str(),
                ],
                ev(Ev::Click, |_| Msg::ResetPosition),
            ],
        ],
        canvas![
            el_ref(&model.canvas),
            attrs![
                At::Width => px(500),
                At::Height => px(500),
            ],
            style![
                St::Border => "1px solid black",
            ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
