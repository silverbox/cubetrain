#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;

use super::cubeset::CubeSet;

use super::cube::CubeSurface;
use super::cube::Cube;

use super::cubic_calc::NormPoint;
use super::cubic_calc::CameraModel;
use super::cubic_calc::ViewPoint2D;
use super::cubic_calc::perspective_projection;
use super::cubic_calc::viewing_transform;

pub const CANVAS_W: f32 = 400.0;
pub const CANVAS_H: f32 = 400.0;
const OFFSET_X: f32 = CANVAS_W / 2.0;
const OFFSET_Y: f32 = CANVAS_H / 2.0 + 20.0;

const X_AXIS_LINE_S: NormPoint = NormPoint {x:  170.0, y: -150.0, z: -150.0, w: 1.0};
const X_AXIS_LINE_E: NormPoint = NormPoint {x:  300.0, y: -150.0, z: -150.0, w: 1.0};
const Y_AXIS_LINE_S: NormPoint = NormPoint {x: -150.0, y:  170.0, z: -150.0, w: 1.0};
const Y_AXIS_LINE_E: NormPoint = NormPoint {x: -150.0, y:  300.0, z: -150.0, w: 1.0};
const Z_AXIS_LINE_S: NormPoint = NormPoint {x: -150.0, y: -150.0, z:  170.0, w: 1.0};
const Z_AXIS_LINE_E: NormPoint = NormPoint {x: -150.0, y: -150.0, z:  300.0, w: 1.0};
const AXIS_OFFSET_X: f32 = 20.0;
const AXIS_OFFSET_Y: f32 = 10.0;

pub fn draw_cubeset(canvas_element: &HtmlCanvasElement, cubeset: &CubeSet, camera: &CameraModel, on_animation: bool) {
    let ctx = seed::canvas_context_2d(canvas_element);

    // clear canvas
    ctx.begin_path();
    ctx.clear_rect(0., 0., CANVAS_W as f64, CANVAS_H as f64);
    if !on_animation && cubeset.is_finished() {
        ctx.rect(0., 0., CANVAS_W as f64, CANVAS_H as f64);
        ctx.set_fill_style(&JsValue::from_str("cyan"));
        ctx.fill();
    }

    let mut wkvec:Vec<&Cube> = cubeset.get_all_cube().into_iter().map(|item| item).collect();
    wkvec.sort_by(|a, b| {
        let cvp_a = get_center_view_point(a, camera);
        let cvp_b = get_center_view_point(b, camera);
        cvp_a.z.partial_cmp(&cvp_b.z).unwrap()
    });
    for cube in wkvec.into_iter() {
        draw_cube(&ctx, cube, camera);
    }

    draw_axis(&ctx, camera);
}

fn draw_axis(ctx: &CanvasRenderingContext2d, camera: &CameraModel) {
    ctx.set_fill_style(&JsValue::from_str("black"));
    ctx.set_stroke_style(&JsValue::from_str("gray"));
    ctx.set_font("bold 16px serif");

    let view_point_xs = get_view_p(&X_AXIS_LINE_S, camera);
    let view_point_xe = get_view_p(&X_AXIS_LINE_E, camera);
    ctx.begin_path();
    ctx.move_to((view_point_xs.x + OFFSET_X) as f64, (OFFSET_Y - view_point_xs.y) as f64);
    ctx.line_to((view_point_xe.x + OFFSET_X) as f64, (OFFSET_Y - view_point_xe.y) as f64);
    ctx.close_path();
    ctx.stroke();
    ctx.fill_text("X軸", (view_point_xe.x + OFFSET_X - AXIS_OFFSET_X) as f64, (OFFSET_Y - view_point_xe.y + AXIS_OFFSET_Y) as f64).expect("Couldn't draw text");

    let view_point_ys = get_view_p(&Y_AXIS_LINE_S, camera);
    let view_point_ye = get_view_p(&Y_AXIS_LINE_E, camera);
    ctx.begin_path();
    ctx.move_to((view_point_ys.x + OFFSET_X) as f64, (OFFSET_Y - view_point_ys.y) as f64);
    ctx.line_to((view_point_ye.x + OFFSET_X) as f64, (OFFSET_Y - view_point_ye.y) as f64);
    ctx.close_path();
    ctx.stroke();
    ctx.fill_text("Y軸", (view_point_ye.x + OFFSET_X - AXIS_OFFSET_X) as f64, (OFFSET_Y - view_point_ye.y + AXIS_OFFSET_Y) as f64).expect("Couldn't draw text");

    let view_point_zs = get_view_p(&Z_AXIS_LINE_S, camera);
    let view_point_ze = get_view_p(&Z_AXIS_LINE_E, camera);
    ctx.begin_path();
    ctx.move_to((view_point_zs.x + OFFSET_X) as f64, (OFFSET_Y - view_point_zs.y) as f64);
    ctx.line_to((view_point_ze.x + OFFSET_X) as f64, (OFFSET_Y - view_point_ze.y) as f64);
    ctx.close_path();
    ctx.stroke();
    ctx.fill_text("Z軸", (view_point_ze.x + OFFSET_X - AXIS_OFFSET_X) as f64, (OFFSET_Y - view_point_ze.y + AXIS_OFFSET_Y) as f64).expect("Couldn't draw text");
}

fn draw_cube(ctx: &CanvasRenderingContext2d, cube: &Cube, camera: &CameraModel) {

    // 立方体描画
    let view_point_a = get_view_point("a", cube, camera);
    let view_point_b = get_view_point("b", cube, camera);
    let view_point_c = get_view_point("c", cube, camera);
    let view_point_d = get_view_point("d", cube, camera);
    let view_point_e = get_view_point("e", cube, camera);
    let view_point_f = get_view_point("f", cube, camera);
    let view_point_g = get_view_point("g", cube, camera);
    let view_point_h = get_view_point("h", cube, camera);

    let is_visible_abcd = cube.is_visible_surface(&CubeSurface::ABCD, camera);
    let is_visible_abef = cube.is_visible_surface(&CubeSurface::ABEF, camera);
    let is_visible_bcfg = cube.is_visible_surface(&CubeSurface::BCFG, camera);
    let is_visible_cdgh = cube.is_visible_surface(&CubeSurface::CDGH, camera);
    let is_visible_daeh = cube.is_visible_surface(&CubeSurface::DAEH, camera);
    let is_visible_efgh = cube.is_visible_surface(&CubeSurface::EFGH, camera);
    if is_visible_abcd {
        ctx.begin_path();
        ctx.move_to((view_point_a.x + OFFSET_X) as f64, (OFFSET_Y - view_point_a.y) as f64);
        ctx.line_to((view_point_b.x + OFFSET_X) as f64, (OFFSET_Y - view_point_b.y) as f64);
        ctx.line_to((view_point_c.x + OFFSET_X) as f64, (OFFSET_Y - view_point_c.y) as f64);
        ctx.line_to((view_point_d.x + OFFSET_X) as f64, (OFFSET_Y - view_point_d.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_abcd.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }
    if is_visible_abef {
        ctx.begin_path();
        ctx.move_to((view_point_a.x + OFFSET_X) as f64, (OFFSET_Y - view_point_a.y) as f64);
        ctx.line_to((view_point_b.x + OFFSET_X) as f64, (OFFSET_Y - view_point_b.y) as f64);
        ctx.line_to((view_point_f.x + OFFSET_X) as f64, (OFFSET_Y - view_point_f.y) as f64);
        ctx.line_to((view_point_e.x + OFFSET_X) as f64, (OFFSET_Y - view_point_e.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_abef.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }
    if is_visible_bcfg {
        ctx.begin_path();
        ctx.move_to((view_point_b.x + OFFSET_X) as f64, (OFFSET_Y - view_point_b.y) as f64);
        ctx.line_to((view_point_c.x + OFFSET_X) as f64, (OFFSET_Y - view_point_c.y) as f64);
        ctx.line_to((view_point_g.x + OFFSET_X) as f64, (OFFSET_Y - view_point_g.y) as f64);
        ctx.line_to((view_point_f.x + OFFSET_X) as f64, (OFFSET_Y - view_point_f.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_bcfg.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }
    if is_visible_cdgh {
        ctx.begin_path();
        ctx.move_to((view_point_c.x + OFFSET_X) as f64, (OFFSET_Y - view_point_c.y) as f64);
        ctx.line_to((view_point_d.x + OFFSET_X) as f64, (OFFSET_Y - view_point_d.y) as f64);
        ctx.line_to((view_point_h.x + OFFSET_X) as f64, (OFFSET_Y - view_point_h.y) as f64);
        ctx.line_to((view_point_g.x + OFFSET_X) as f64, (OFFSET_Y - view_point_g.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_cdgh.as_css_str()));
        ctx.stroke();
        ctx.fill();
    }
    if is_visible_daeh {
        ctx.begin_path();
        ctx.move_to((view_point_d.x + OFFSET_X) as f64, (OFFSET_Y - view_point_d.y) as f64);
        ctx.line_to((view_point_a.x + OFFSET_X) as f64, (OFFSET_Y - view_point_a.y) as f64);
        ctx.line_to((view_point_e.x + OFFSET_X) as f64, (OFFSET_Y - view_point_e.y) as f64);
        ctx.line_to((view_point_h.x + OFFSET_X) as f64, (OFFSET_Y - view_point_h.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_dahe.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }
    if is_visible_efgh {
        ctx.begin_path();
        ctx.move_to((view_point_e.x + OFFSET_X) as f64, (OFFSET_Y - view_point_e.y) as f64);
        ctx.line_to((view_point_f.x + OFFSET_X) as f64, (OFFSET_Y - view_point_f.y) as f64);
        ctx.line_to((view_point_g.x + OFFSET_X) as f64, (OFFSET_Y - view_point_g.y) as f64);
        ctx.line_to((view_point_h.x + OFFSET_X) as f64, (OFFSET_Y - view_point_h.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_efgh.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }

}

fn get_center_view_point(cube: &Cube, camera: &CameraModel) -> ViewPoint2D {
    let perspective_point_wk = perspective_projection(&cube.center_point, camera);
    viewing_transform(&perspective_point_wk, &camera.view_frustum)
}

fn get_view_point(point_name: &str, cube: &Cube, camera: &CameraModel) -> ViewPoint2D {
    let perspective_point_wk = perspective_projection(&cube.get_abs_point(point_name), camera);
    viewing_transform(&perspective_point_wk, &camera.view_frustum)
}

fn get_view_p(point: &NormPoint, camera: &CameraModel) -> ViewPoint2D {
    let perspective_point_wk = perspective_projection(point, camera);
    viewing_transform(&perspective_point_wk, &camera.view_frustum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_test() {
        let cube = &Cube::default();
        let camera = &CameraModel::default();

        let view_point_a = get_view_point("a", cube, camera);
        let view_point_b = get_view_point("b", cube, camera);
        let view_point_c = get_view_point("c", cube, camera);
        let view_point_d = get_view_point("d", cube, camera);
        let view_point_e = get_view_point("e", cube, camera);
        let view_point_f = get_view_point("f", cube, camera);
        let view_point_g = get_view_point("g", cube, camera);
        let view_point_h = get_view_point("h", cube, camera);

        let abs_val1 = (CUBE_SIZE * 0.35) as i32;
        let abs_val2 = (CUBE_SIZE * 0.2) as i32;
        let abs_val3 = (CUBE_SIZE * 0.4) as i32;
        assert_eq!(view_point_a.x as i32, -abs_val1);
        assert_eq!(view_point_a.y as i32,  abs_val2);
        assert_eq!(view_point_b.x as i32,   0);
        assert_eq!(view_point_b.y as i32,   0);
        assert_eq!(view_point_c.x as i32,  abs_val1);
        assert_eq!(view_point_c.y as i32,  abs_val2);
        assert_eq!(view_point_d.x as i32,   0);
        assert_eq!(view_point_d.y as i32,  abs_val3);

        assert_eq!(view_point_e.x as i32, -abs_val1);
        assert_eq!(view_point_e.y as i32, -abs_val2);
        assert_eq!(view_point_f.x as i32,   0);
        assert_eq!(view_point_f.y as i32, -abs_val3);
        assert_eq!(view_point_g.x as i32,  abs_val1);
        assert_eq!(view_point_g.y as i32, -abs_val2);
    }
}
