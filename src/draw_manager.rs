#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;

use super::cubeset::CubeSet;

use super::cube::CubeColor;
use super::cube::CubeSurface;
use super::cube::Cube;
use super::cube::CUBE_SIZE;


use super::cubic_calc::NormPoint;
use super::cubic_calc::CameraAxisPoint;
use super::cubic_calc::CameraVec;
use super::cubic_calc::CameraModel;
use super::cubic_calc::ViewFrustum;
use super::cubic_calc::ViewPoint2D;
use super::cubic_calc::perspective_projection;
use super::cubic_calc::viewing_transform;

pub fn draw_cubeset(canvas_element: &HtmlCanvasElement, cubeset: &CubeSet, camera: &CameraModel) {
    let ctx = seed::canvas_context_2d(canvas_element);

    // clear canvas
    ctx.begin_path();
    ctx.clear_rect(0., 0., 800., 800.);

    let mut wkvec:Vec<&Cube> = cubeset.get_all_cube().into_iter().map(|item| item).collect();
    wkvec.sort_by(|a, b| {
        let cvp_a = get_center_view_point(a, camera);
        let cvp_b = get_center_view_point(b, camera);
        cvp_a.z.partial_cmp(&cvp_b.z).unwrap()
    });
    for cube in wkvec.into_iter() {
        draw_cube(&ctx, cube, camera);
    }

    // let debugtxt = format!("visible abcd={}, abef={}, bcfg={}, cdgh={}, daeh={}, efgh={}",
    //     is_visible_abcd, is_visible_abef, is_visible_bcfg, is_visible_cdgh, is_visible_daeh, is_visible_efgh);
    // ctx.set_fill_style(&JsValue::from_str("red"));
    // ctx.fill_text(&debugtxt, 10.0, 20.0);
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

    let offset_x = 200.0;
    let offset_y = 200.0;
    let is_visible_abcd = cube.is_visible_surface(&CubeSurface::ABCD, camera);
    let is_visible_abef = cube.is_visible_surface(&CubeSurface::ABEF, camera);
    let is_visible_bcfg = cube.is_visible_surface(&CubeSurface::BCFG, camera);
    let is_visible_cdgh = cube.is_visible_surface(&CubeSurface::CDGH, camera);
    let is_visible_daeh = cube.is_visible_surface(&CubeSurface::DAEH, camera);
    let is_visible_efgh = cube.is_visible_surface(&CubeSurface::EFGH, camera);
    if is_visible_abcd {
        ctx.begin_path();
        ctx.move_to((view_point_a.x + offset_x) as f64, (offset_y - view_point_a.y) as f64);
        ctx.line_to((view_point_b.x + offset_x) as f64, (offset_y - view_point_b.y) as f64);
        ctx.line_to((view_point_c.x + offset_x) as f64, (offset_y - view_point_c.y) as f64);
        ctx.line_to((view_point_d.x + offset_x) as f64, (offset_y - view_point_d.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_abcd.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }
    if is_visible_abef {
        ctx.begin_path();
        ctx.move_to((view_point_a.x + offset_x) as f64, (offset_y - view_point_a.y) as f64);
        ctx.line_to((view_point_b.x + offset_x) as f64, (offset_y - view_point_b.y) as f64);
        ctx.line_to((view_point_f.x + offset_x) as f64, (offset_y - view_point_f.y) as f64);
        ctx.line_to((view_point_e.x + offset_x) as f64, (offset_y - view_point_e.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_abef.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }
    if is_visible_bcfg {
        ctx.begin_path();
        ctx.move_to((view_point_b.x + offset_x) as f64, (offset_y - view_point_b.y) as f64);
        ctx.line_to((view_point_c.x + offset_x) as f64, (offset_y - view_point_c.y) as f64);
        ctx.line_to((view_point_g.x + offset_x) as f64, (offset_y - view_point_g.y) as f64);
        ctx.line_to((view_point_f.x + offset_x) as f64, (offset_y - view_point_f.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_bcfg.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }
    if is_visible_cdgh {
        ctx.begin_path();
        ctx.move_to((view_point_c.x + offset_x) as f64, (offset_y - view_point_c.y) as f64);
        ctx.line_to((view_point_d.x + offset_x) as f64, (offset_y - view_point_d.y) as f64);
        ctx.line_to((view_point_h.x + offset_x) as f64, (offset_y - view_point_h.y) as f64);
        ctx.line_to((view_point_g.x + offset_x) as f64, (offset_y - view_point_g.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_cdgh.as_css_str()));
        ctx.stroke();
        ctx.fill();
    }
    if is_visible_daeh {
        ctx.begin_path();
        ctx.move_to((view_point_d.x + offset_x) as f64, (offset_y - view_point_d.y) as f64);
        ctx.line_to((view_point_a.x + offset_x) as f64, (offset_y - view_point_a.y) as f64);
        ctx.line_to((view_point_e.x + offset_x) as f64, (offset_y - view_point_e.y) as f64);
        ctx.line_to((view_point_h.x + offset_x) as f64, (offset_y - view_point_h.y) as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from_str(cube.color_dahe.as_css_str()));
        ctx.fill();
        ctx.stroke();
    }
    if is_visible_efgh {
        ctx.begin_path();
        ctx.move_to((view_point_e.x + offset_x) as f64, (offset_y - view_point_e.y) as f64);
        ctx.line_to((view_point_f.x + offset_x) as f64, (offset_y - view_point_f.y) as f64);
        ctx.line_to((view_point_g.x + offset_x) as f64, (offset_y - view_point_g.y) as f64);
        ctx.line_to((view_point_h.x + offset_x) as f64, (offset_y - view_point_h.y) as f64);
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
