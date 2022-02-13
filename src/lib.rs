// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

mod cube;
mod cubic_calc;

use cube::CubeColor;
use cube::Cube;

use cubic_calc::NormPoint;
use cubic_calc::CameraVec;
use cubic_calc::ViewPoint2D;
use cubic_calc::CameraAxisPoint;
use cubic_calc::ViewFrustum;
use cubic_calc::perspective_projection;
use cubic_calc::viewing_transform;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    Model { 
        counter: 0,
        camera_pos: CameraVec { x: 0.0, y: 0.0, z: -300.0 },
        camera_x_axis: CameraVec { x: -0.706, y:  0.0  , z:  0.706 },
        camera_y_axis: CameraVec { x: -0.405, y:  0.810, z: -0.405 }, // vec X * vec Z
        camera_z_axis: CameraVec { x: -0.577, y: -0.577, z: -0.577 },
        // camera_x_axis: CameraVec { x: 0.0, y: 0.0, z: 1.0 },
        // camera_y_axis: CameraVec { x: 1.0, y: 0.0, z: 0.0 },
        // camera_z_axis: CameraVec { x: 0.0, y: 1.0, z: 0.0 },
        cube: Cube::default(),
        canvas: ElRef::<HtmlCanvasElement>::default(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
    camera_pos: CameraVec,
    camera_x_axis: CameraVec,
    camera_y_axis: CameraVec,
    camera_z_axis: CameraVec,
    cube: Cube,
    canvas: ElRef<HtmlCanvasElement>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Rendered,
    RandomRotate,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RandomRotate => model.counter += 1,
        Msg::Rendered => {
            draw(model);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
    }
}

fn draw(model: &mut Model) {
    let cube = &mut model.cube;
    let canvas = &model.canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    // clear canvas
    ctx.begin_path();
    ctx.clear_rect(0., 0., 800., 800.);

    let width = 200. ;
    let height = 100. ;

    ctx.rect(0., 0., width, height);
    ctx.set_fill_style(&JsValue::from_str(cube.color_dahe.as_css_str()));
    ctx.fill();

    ctx.move_to(0., 0.);
    ctx.line_to(width, height);
    ctx.stroke();

    cube.rotate_test();
    let perspective_point_wk_a = get_abs_perspective_point("a", &model);
    let perspective_point_wk_b = get_abs_perspective_point("b", &model);
    let perspective_point_wk_c = get_abs_perspective_point("c", &model);
    let perspective_point_wk_d = get_abs_perspective_point("d", &model);
    let perspective_point_wk_e = get_abs_perspective_point("e", &model);
    let perspective_point_wk_f = get_abs_perspective_point("f", &model);
    let perspective_point_wk_g = get_abs_perspective_point("g", &model);
    let perspective_point_wk_h = get_abs_perspective_point("h", &model);

    let vf = ViewFrustum { left: 200.0, right: -200.0, top: 200.0, bottom: -200.0, near: 100.0, far: 200.0 };
    let view_point_a = viewing_transform(&perspective_point_wk_a, &vf);
    let view_point_b = viewing_transform(&perspective_point_wk_b, &vf);
    let view_point_c = viewing_transform(&perspective_point_wk_c, &vf);
    let view_point_d = viewing_transform(&perspective_point_wk_d, &vf);
    let view_point_e = viewing_transform(&perspective_point_wk_e, &vf);
    let view_point_f = viewing_transform(&perspective_point_wk_f, &vf);
    let view_point_g = viewing_transform(&perspective_point_wk_g, &vf);
    let view_point_h = viewing_transform(&perspective_point_wk_h, &vf);

    let offset_x = 100.0;
    let offset_y = 300.0;
    ctx.move_to((perspective_point_wk_a.x + offset_x) as f64, (perspective_point_wk_a.y + offset_y) as f64);
    ctx.line_to((perspective_point_wk_b.x + offset_x) as f64, (perspective_point_wk_b.y + offset_y) as f64);
    ctx.stroke();
    ctx.line_to((perspective_point_wk_c.x + offset_x) as f64, (perspective_point_wk_c.y + offset_y) as f64);
    ctx.stroke();
    ctx.line_to((perspective_point_wk_d.x + offset_x) as f64, (perspective_point_wk_d.y + offset_y) as f64);
    ctx.stroke();
    ctx.line_to((perspective_point_wk_a.x + offset_x) as f64, (perspective_point_wk_a.y + offset_y) as f64);
    ctx.stroke();

    ctx.move_to((perspective_point_wk_e.x + offset_x) as f64, (perspective_point_wk_e.y + offset_y) as f64);
    ctx.line_to((perspective_point_wk_f.x + offset_x) as f64, (perspective_point_wk_f.y + offset_y) as f64);
    ctx.stroke();
    ctx.line_to((perspective_point_wk_g.x + offset_x) as f64, (perspective_point_wk_g.y + offset_y) as f64);
    ctx.stroke();
    ctx.line_to((perspective_point_wk_h.x + offset_x) as f64, (perspective_point_wk_h.y + offset_y) as f64);
    ctx.stroke();
    ctx.line_to((perspective_point_wk_e.x + offset_x) as f64, (perspective_point_wk_e.y + offset_y) as f64);
    ctx.stroke();

    ctx.move_to((perspective_point_wk_a.x + offset_x) as f64, (perspective_point_wk_a.y + offset_y) as f64);
    ctx.line_to((perspective_point_wk_e.x + offset_x) as f64, (perspective_point_wk_e.y + offset_y) as f64);
    ctx.stroke();

    ctx.move_to((perspective_point_wk_b.x + offset_x) as f64, (perspective_point_wk_b.y + offset_y) as f64);
    ctx.line_to((perspective_point_wk_f.x + offset_x) as f64, (perspective_point_wk_f.y + offset_y) as f64);
    ctx.stroke();

    ctx.move_to((perspective_point_wk_c.x + offset_x) as f64, (perspective_point_wk_c.y + offset_y) as f64);
    ctx.line_to((perspective_point_wk_g.x + offset_x) as f64, (perspective_point_wk_g.y + offset_y) as f64);
    ctx.stroke();

    ctx.move_to((perspective_point_wk_d.x + offset_x) as f64, (perspective_point_wk_d.y + offset_y) as f64);
    ctx.line_to((perspective_point_wk_h.x + offset_x) as f64, (perspective_point_wk_h.y + offset_y) as f64);
    ctx.stroke();

    let debugtxt = format!("perspective_point_wk_a x={}, y={}, z={}", perspective_point_wk_a.x, perspective_point_wk_a.y, perspective_point_wk_a.z);
    ctx.set_fill_style(&JsValue::from_str("red"));
    ctx.fill_text(&debugtxt, 10.0, 20.0);
}

fn get_abs_perspective_point(point_name: &str, model: &Model) -> CameraAxisPoint {
    let cube = &model.cube;
    perspective_projection(&cube.get_abs_point(point_name), &model.camera_pos,
        &model.camera_x_axis, &model.camera_y_axis, &model.camera_z_axis)
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![
            model.counter,
            style![
                St::BackgroundColor => CubeColor::Red.as_css_str(),
            ],
            ev(Ev::Click, |_| Msg::RandomRotate),
        ],
        canvas![
            el_ref(&model.canvas),
            attrs![
                At::Width => px(800),
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
