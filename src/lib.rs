// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;
extern crate rand;

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

const subcamera_alt: CameraModel = CameraModel {
    pos: CameraVec { x: -200.0, y: -200.0, z: -200.0 },
    x_axis: CameraVec { x: 0.706, y:  0.0  , z: -0.706 },
    y_axis: CameraVec { x: 0.405, y: -0.810, z:  0.405 }, // vec X * vec Z
    z_axis: CameraVec { x: 0.577, y:  0.577, z:  0.577 },
    view_frustum: ViewFrustum { left: 200.0, right: -200.0, top: -200.0, bottom: 200.0, near: 100.0, far: 200.0 }
  };

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    Model { 
        counter: 0,
        scramble_step: 25,
        animaion_speed: 50,
        animation_rotate_target: RotateTarget::default(),
        animation_countdown: 0,
        cubeset: CubeSet::default(),
        camera: CameraModel::default(),
        canvas: ElRef::<HtmlCanvasElement>::default(),
        subcamera: subcamera_alt,
        subcanvas: ElRef::<HtmlCanvasElement>::default(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
    scramble_step: i32,
    animaion_speed: i32,
    animation_rotate_target: RotateTarget,
    animation_countdown: i32,
    cubeset: CubeSet,
    camera: CameraModel,
    canvas: ElRef<HtmlCanvasElement>,
    subcamera: CameraModel,
    subcanvas: ElRef<HtmlCanvasElement>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    AnimationSpeedChanged(String),
    ScrambleStepChanged(String),
    Rotate(RotateTarget),
    Rendered,
    ResetPosition,
    ScramblePosition,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AnimationSpeedChanged(input_val) => {
            model.animaion_speed = input_val.parse().unwrap();;
        },
        Msg::ScrambleStepChanged(input_val) => {
            model.scramble_step = input_val.parse().unwrap();;
        },
        Msg::Rotate(rotate_target) => {
            if model.animation_countdown > 0 {
                return;
            }
            model.animation_countdown = (900 / model.animaion_speed) as i32;
            model.animation_rotate_target = RotateTarget {
                axis: rotate_target.axis,
                layer: rotate_target.layer,
                rad: rotate_target.rad / (model.animation_countdown as f32)
            };
        },
        Msg::ResetPosition => {
            // model.counter = model.animaion_speed;
            model.cubeset = CubeSet::default();
        },
        Msg::ScramblePosition => {
            model.cubeset = CubeSet::default();
            for i in 0..model.scramble_step {
                model.cubeset.rotate_layer(&RotateTarget::random(PI / 2.0));
            }
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
    let subcanvas_element = model.subcanvas.get().expect("get sub canvas element");

    draw_cubeset(&canvas_element, &model.cubeset, &model.camera);
    draw_cubeset(&subcanvas_element, &model.cubeset, &model.subcamera);
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
                    style! {St::Width => px(50)},
                    attrs! {
                        At::Value => model.animaion_speed,
                        At::Type => "number",
                    },
                    input_ev(Ev::Input, Msg::AnimationSpeedChanged),
                ],
            ],
            table![
                C!["operationtable"],
                tr![
                    th!["回転軸"], // ↓ / 対象 →
                    th!["全体(正)"],
                    th!["全体(逆)"],
                    th!["前/上(正)"],
                    th!["前/上(逆)"],
                    th!["中(正)"],
                    th!["中(逆)"],
                    th!["奥/下(正)"],
                    th!["奥/下(逆)"],
                ],
                tr![
                    td!["X軸"],
                    td![
                        button![
                            "x",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::X,
                                layer: RotateLayer::All,
                                rad: -PI / 2.0
                            })),
                        ],    
                    ],
                    td![
                        button![
                            "x'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::X,
                                layer: RotateLayer::All,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "R",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::X,
                                layer: RotateLayer::Positive,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "R'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::X,
                                layer: RotateLayer::Positive,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "M",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::X,
                                layer: RotateLayer::Neutral,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "M",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::X,
                                layer: RotateLayer::Neutral,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "L",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::X,
                                layer: RotateLayer::Negative,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "L'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::X,
                                layer: RotateLayer::Negative,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                ],
                tr![
                    td!["Y軸"],
                    td![
                        button![
                            "y",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Y,
                                layer: RotateLayer::All,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "y'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Y,
                                layer: RotateLayer::All,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "U",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Y,
                                layer: RotateLayer::Positive,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "U'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Y,
                                layer: RotateLayer::Positive,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "E",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Y,
                                layer: RotateLayer::Neutral,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "E'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Y,
                                layer: RotateLayer::Neutral,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "D",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Y,
                                layer: RotateLayer::Negative,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "D'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Y,
                                layer: RotateLayer::Negative,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                ],
                tr![
                    td!["Z軸"],
                    td![
                        button![
                            "z",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Z,
                                layer: RotateLayer::All,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "z'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Z,
                                layer: RotateLayer::All,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "F",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Z,
                                layer: RotateLayer::Positive,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "F'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Z,
                                layer: RotateLayer::Positive,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "S",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Z,
                                layer: RotateLayer::Neutral,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "S'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Z,
                                layer: RotateLayer::Neutral,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "B",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Z,
                                layer: RotateLayer::Negative,
                                rad: PI / 2.0
                            })),
                        ],
                    ],
                    td![
                        button![
                            "B'",
                            ev(Ev::Click, |_| Msg::Rotate(RotateTarget {
                                axis: RotateAxis::Z,
                                layer: RotateLayer::Negative,
                                rad: -PI / 2.0
                            })),
                        ],
                    ],
                ],
            ],
            button![
                "Reset",
                style![
                    St::BackgroundColor => CubeColor::Lime.as_css_str(),
                ],
                ev(Ev::Click, |_| Msg::ResetPosition),
            ],
            div![
                label!["手数"],
                input![
                    style! {St::Width => px(50)},
                    attrs! {
                        At::Value => model.scramble_step,
                        At::Type => "number",
                    },
                    input_ev(Ev::Input, Msg::ScrambleStepChanged),
                ],
                button![
                    "Scramble",
                    style![
                        St::BackgroundColor => CubeColor::Lime.as_css_str(),
                    ],
                    ev(Ev::Click, |_| Msg::ScramblePosition),
                ],
            ],
        ],
        canvas![
            el_ref(&model.canvas),
            attrs![
                At::Width => px(400),
                At::Height => px(400),
            ],
            style![
                St::Border => "1px solid black",
            ],
        ],
        canvas![
            el_ref(&model.subcanvas),
            attrs![
                At::Width => px(400),
                At::Height => px(400),
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
