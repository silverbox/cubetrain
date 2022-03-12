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
use cubeset::CubeSet;
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
        animaion_speed: 10,
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
    Rendered,
    RandomRotate,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AnimationSpeedChanged(input_val) => {
            model.animaion_speed = input_val.parse().unwrap();;
        },
        Msg::RandomRotate => {
            model.counter = model.animaion_speed;
            model.cubeset.rotate_test();
        },
        Msg::Rendered => {
            draw(model);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            orders.after_next_render(|_| Msg::Rendered).skip();
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
        "This is a counter: ",
        C!["counter"],
        label!["動作速度"],
        input![
            attrs! {
                At::Value => model.animaion_speed,
                At::Type => "number",
            },
            input_ev(Ev::Input, Msg::AnimationSpeedChanged),
        ],
        button![
            model.counter,
            style![
                St::BackgroundColor => CubeColor::Lime.as_css_str(),
            ],
            ev(Ev::Click, |_| Msg::RandomRotate),
        ],
        canvas![
            el_ref(&model.canvas),
            attrs![
                At::Width => px(800),
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
