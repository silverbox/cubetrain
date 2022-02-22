#[allow(dead_code)]
#[allow(clippy::wildcard_imports)]

// use super::cubic_calc::Point;
use super::cubic_calc::*;

const ROTATE_STEP: f32 = PI / 16.0;

pub enum CubeColor {
  White,
  Yellow,
  Blue,   // 青
  Red, // 赤
  Orange, // オレンジ
  Lime,    // 黄緑
  Black
}

impl CubeColor {
  pub const fn as_int(&self) -> i32 {
      match self {
          Self::White => 0xffffff,
          Self::Yellow => 0xffff00,
          Self::Blue => 0x0000ff,
          Self::Red => 0xff4500,
          Self::Orange => 0xff4500,
          Self::Lime => 0x00ff00,
          Self::Black => 0x000000
      }
  }
  pub const fn as_css_str(&self) -> &str {
    match self {
        Self::White => "white",
        Self::Yellow => "yellow",
        Self::Blue => "navy",
        Self::Red => "maroon",
        Self::Orange => "orangered",
        Self::Lime => "lime",
        Self::Black => "black"
    }
  }
}

pub enum CubeSurface {
  ABCD,
  ABEF,
  BCFG,
  CDGH,
  DAEH,
  EFGH,
}

// 右手座標系
// x軸：右側面方向
// y軸：上方向
// z軸：手前方向
pub struct Cube {
  // １辺のサイズ
  pub size: f32,
  // 絶対空間値
  pub center_point: NormPoint,
  // 相対座標係数（初期値）
  // center_point、とそれぞれの正規化デバイス座標を元に、それぞれの絶対座標を計算する。
  pub pa: NormPoint, // 上左手前
  pub pb: NormPoint, // 上右手前
  pub pc: NormPoint, // 上右奥
  pub pd: NormPoint, // 上左奥
  pub pe: NormPoint, // 下左手前
  pub pf: NormPoint, // 下右手前
  pub pg: NormPoint, // 下右奥
  pub ph: NormPoint, // 下左奥
  //
  pub color_abcd: CubeColor, // 上面
  pub color_abef: CubeColor, // 側面１
  pub color_bcfg: CubeColor, // 側面２
  pub color_cdgh: CubeColor, // 側面３
  pub color_dahe: CubeColor, // 側面４
  pub color_efgh: CubeColor, // 下面
  //
  // 各点初期値と各軸中心の回転角度で、それぞれの正規化デバイス座標を算出
  pub x_axis_rotate_rad: f32,
  pub y_axis_rotate_rad: f32,
  pub z_axis_rotate_rad: f32,
}

impl Default for Cube {
  fn default() -> Self {
    Self {
      size: 100.0,
      center_point: NormPoint {x: 0.0, y:  0.0, z: 0.0, w: 1.0},
      //
      pa: NormPoint {x: -1.0, y:  1.0, z:  1.0, w: 1.0},
      pb: NormPoint {x:  1.0, y:  1.0, z:  1.0, w: 1.0},
      pc: NormPoint {x:  1.0, y:  1.0, z: -1.0, w: 1.0},
      pd: NormPoint {x: -1.0, y:  1.0, z: -1.0, w: 1.0},
      pe: NormPoint {x: -1.0, y: -1.0, z:  1.0, w: 1.0},
      pf: NormPoint {x:  1.0, y: -1.0, z:  1.0, w: 1.0},
      pg: NormPoint {x:  1.0, y: -1.0, z: -1.0, w: 1.0},
      ph: NormPoint {x: -1.0, y: -1.0, z: -1.0, w: 1.0},
      //
      color_abcd: CubeColor::White,
      color_abef: CubeColor::Red,
      color_bcfg: CubeColor::Blue,
      color_cdgh: CubeColor::Orange,
      color_dahe: CubeColor::Lime,
      color_efgh: CubeColor::Yellow,
      //
      x_axis_rotate_rad: 0.0,
      y_axis_rotate_rad: 0.0,
      z_axis_rotate_rad: 0.0,
    }
  }
}

impl Cube {

  pub fn get_abs_point(&self, point_name: &str) -> NormPoint {
    let obj_norm_point = self.get_norm_point(&self.get_target_point(point_name));
    let scaled_point = scale(&obj_norm_point, self.size, self.size, self.size);
    shift(&scaled_point, self.center_point.x, self.center_point.y, self.center_point.z)
  }

  // 回転した面の視点からの可視を判断。正方形の３つの点を使った三角形で処理する
  pub fn is_visible_surface(&self, surface: CubeSurface, camera: &CameraModel) -> bool {
    // , camera_pos: &CameraVec, camera_x_axis: &CameraVec, camera_y_axis: &CameraVec, camera_z_axis: &CameraVec
    let point_list = self.get_surface_point_list(surface);

    // 回転済み絶対座標
    let abs_p1 = self.get_norm_point(point_list[0]);
    let abs_p2 = self.get_norm_point(point_list[1]);
    let abs_p3 = self.get_norm_point(point_list[2]);
    // カメラ視点座標
    let rot_p1 = perspective_projection(&abs_p1, camera);
    let rot_p2 = perspective_projection(&abs_p2, camera);
    let rot_p3 = perspective_projection(&abs_p3, camera);

    // 法線ベクトル
    let normal_vec = CameraVec {
      x: (rot_p2.y - rot_p1.y) * (rot_p3.z - rot_p2.z) - (rot_p2.z - rot_p1.z) * (rot_p3.y - rot_p2.y),
      y: (rot_p2.z - rot_p1.z) * (rot_p3.x - rot_p2.x) - (rot_p2.x - rot_p1.x) * (rot_p3.z - rot_p2.z),
      z: (rot_p2.x - rot_p1.x) * (rot_p3.y - rot_p2.y) - (rot_p2.y - rot_p1.y) * (rot_p3.x - rot_p2.x)
    };

    // カメラ視点から中心点へのベクトル
    let center_vec = CameraVec {
      x: (rot_p1.x + rot_p2.x + rot_p3.x) / 3.0,
      y: (rot_p1.y + rot_p2.y + rot_p3.y) / 3.0,
      z: (rot_p1.z + rot_p2.z + rot_p3.z) / 3.0
    };

    (normal_vec.x * center_vec.x) + (normal_vec.y * center_vec.y) + (normal_vec.z * center_vec.z) > 0.0
  }

  fn adjust_rad(rad: f32, rad_step: f32) -> f32 {
    let wk_rad1 = rad % (PI * 2.0);
    let wk_multiple = ((wk_rad1 / rad_step) + 0.5) as i32;
    (wk_multiple as f32) * rad_step
  }

  fn get_surface_point_list(&self, surface: CubeSurface) -> [&NormPoint; 4] {
    // 法線ベクトルを求めるために見せたい面の点を半時計回りにセットする
    match surface {
      CubeSurface::ABCD => [&self.pa, &self.pb, &self.pc, &self.pd] ,
      CubeSurface::ABEF => [&self.pa, &self.pe, &self.pf, &self.pb] ,
      CubeSurface::BCFG => [&self.pb, &self.pf, &self.pg, &self.pc] ,
      CubeSurface::CDGH => [&self.pc, &self.pg, &self.ph, &self.pd] ,
      CubeSurface::DAEH => [&self.pd, &self.ph, &self.pe, &self.pa] ,
      CubeSurface::EFGH => [&self.pe, &self.ph, &self.pg, &self.pf] ,
      _ => [&self.pe, &self.pf, &self.pg, &self.ph]
    }
  }

  // Cubeの中心位置はそのまま。回転だけする
  fn rotate(&mut self, rad_x: f32, rad_y: f32, rad_z: f32, rad_step: f32) {
    self.x_axis_rotate_rad = Cube::adjust_rad(self.x_axis_rotate_rad + rad_x, rad_step);
    self.y_axis_rotate_rad = Cube::adjust_rad(self.y_axis_rotate_rad + rad_y, rad_step);
    self.z_axis_rotate_rad = Cube::adjust_rad(self.z_axis_rotate_rad + rad_z, rad_step);
  }

  // 引数の点を現在の回転した座標にして返す
  fn get_norm_point(&self, norm_point: &NormPoint) -> NormPoint {
    let obj_normwk_point_1 = x_rotate(norm_point, self.x_axis_rotate_rad);
    let obj_normwk_point_2 = y_rotate(&obj_normwk_point_1, self.y_axis_rotate_rad);
    z_rotate(&obj_normwk_point_2, self.z_axis_rotate_rad)
  }

  fn get_target_point(&self, point_name: &str) -> &NormPoint {
    match point_name {
      "a" => &self.pa ,
      "b" => &self.pb ,
      "c" => &self.pc ,
      "d" => &self.pd ,
      "e" => &self.pe ,
      "f" => &self.pf ,
      "g" => &self.pg ,
      "h" => &self.ph ,
      _ => &self.pa
    }
  }

  pub fn rotate_test(&mut self) {
    self.rotate(ROTATE_STEP, 0.0, 0.0, ROTATE_STEP);
  }
}

// ------ ------
//     Test
// ------ ------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rotate_test() {
    let testcube = &mut Cube::default();

    let point_a1 = testcube.get_abs_point("a");
    assert_eq!(point_a1.x, -100.0);
    assert_eq!(point_a1.y,  100.0);

    testcube.rotate_test();
    assert_eq!(testcube.x_axis_rotate_rad, ROTATE_STEP);

    let point_a2 = testcube.get_abs_point("a");
    let obj_norm_point2 = testcube.get_norm_point(&point_a2);
    assert_eq!(point_a2.x as i32, -100);

    testcube.rotate_test();
    testcube.rotate_test();
    testcube.rotate_test();
    let point_a3 = testcube.get_abs_point("a");
    assert_eq!(point_a3.z as i32,  141);
  }

  #[test]
  fn visible_surface_test() {
    let testcube = &mut Cube::default();
    let camera_pos = CameraVec { x: 200.0, y: 200.0, z: 200.0 };
    let camera_x_axis = CameraVec { x: -0.706, y:  0.0  , z:  0.706 };
    let camera_y_axis = CameraVec { x: -0.405, y: -0.810, z: -0.405 };
    let camera_z_axis = CameraVec { x: -0.577, y: -0.577, z: -0.577 };

    assert_eq!(testcube.is_visible_surface(CubeSurface::ABCD, &camera_pos, &camera_x_axis, &camera_y_axis, &camera_z_axis), true);
    assert_eq!(testcube.is_visible_surface(CubeSurface::ABEF, &camera_pos, &camera_x_axis, &camera_y_axis, &camera_z_axis), true);
    assert_eq!(testcube.is_visible_surface(CubeSurface::BCFG, &camera_pos, &camera_x_axis, &camera_y_axis, &camera_z_axis), true);
    assert_eq!(testcube.is_visible_surface(CubeSurface::CDGH, &camera_pos, &camera_x_axis, &camera_y_axis, &camera_z_axis), false);
    assert_eq!(testcube.is_visible_surface(CubeSurface::DAEH, &camera_pos, &camera_x_axis, &camera_y_axis, &camera_z_axis), false);
    assert_eq!(testcube.is_visible_surface(CubeSurface::EFGH, &camera_pos, &camera_x_axis, &camera_y_axis, &camera_z_axis), false);
  }
}


// struct CubeSet {
//   white_center: Cube,
//   white_red_edge: Cube,
//   white_blue_edge: Cube,
//   white_lime_edge: Cube,
//   white_orange_edge: Cube,
//   white_red_blue_corner: Cube,
//   white_blue_orange_corner: Cube,
//   white_orange_lime_corner: Cube,
//   white_lime_red_corner: Cube,

//   yellow_center: Cube,
//   yellow_red_edge: Cube,
//   yellow_lime_edge: Cube,
//   yellow_orange_edge: Cube,
//   yellow_blue_edge: Cube,
//   yellow_blue_red_corner: Cube,
//   yellow_red_lime_corner: Cube,
//   yellow_lime_orange_corner: Cube,
//   yellow_orange_blue_corner: Cube,

//   blue_center: Cube,
//   red_center: Cube,
//   orange_center: Cube,
//   lime_center: Cube,

//   blue_red_edge: Cube,
//   red_lime_edge: Cube,
//   lime_orange_edge: Cube,
//   orange_blue_edge: Cube
// }

// /**
//  白上面、赤正面、青右側面。キューブの辺の長さを100として、キューブ集合体の中心座標が(0,0,0)

//  */
// impl Default for CubeSet {
//   fn default() -> Self {
//       Self {
//         white_center: Cube {
//           pa: Point {x: -50.0, y: -50.0, z: 150.0},
//           pb: Point {x:  50.0, y: -50.0, z: 150.0},
//           pc: Point {x:  50.0, y:  50.0, z: 150.0},
//           pd: Point {x: -50.0, y:  50.0, z: 150.0},
//           pe: Point {x: -50.0, y: -50.0, z: 150.0},
//           pf: Point {x:  50.0, y: -50.0, z: 150.0},
//           pg: Point {x:  50.0, y:  50.0, z: 150.0},
//           ph: Point {x: -50.0, y:  50.0, z: 150.0},
//           //
//           color_abcd: CubeColor::White,
//           color_abef: CubeColor::Black,
//           color_bcfg: CubeColor::Black,
//           color_cdgh: CubeColor::Black,
//           color_dahe: CubeColor::Black,
//           color_efgh: CubeColor::Black,
//         },
//         white_red_edge: Cube {
//           pa: Point {x: -50.0, y: -150.0, z: 150.0},
//           pb: Point {x:  50.0, y: -150.0, z: 150.0},
//           pc: Point {x:  50.0, y:  -50.0, z: 150.0},
//           pd: Point {x: -50.0, y:  -50.0, z: 150.0},
//           pe: Point {x: -50.0, y: -150.0, z: 150.0},
//           pf: Point {x:  50.0, y: -150.0, z: 150.0},
//           pg: Point {x:  50.0, y:  -50.0, z: 150.0},
//           ph: Point {x: -50.0, y:  -50.0, z: 150.0},
//           //
//           color_abcd: CubeColor::White,
//           color_abef: CubeColor::Red,
//           color_bcfg: CubeColor::Black,
//           color_cdgh: CubeColor::Black,
//           color_dahe: CubeColor::Black,
//           color_efgh: CubeColor::Black,
//         },
//         white_blue_edge: Cube {
//           pa: Point {x:  50.0, y: -50.0, z: 150.0},
//           pb: Point {x: 150.0, y: -50.0, z: 150.0},
//           pc: Point {x: 150.0, y:  50.0, z: 150.0},
//           pd: Point {x:  50.0, y:  50.0, z: 150.0},
//           pe: Point {x:  50.0, y: -50.0, z: 150.0},
//           pf: Point {x: 150.0, y: -50.0, z: 150.0},
//           pg: Point {x: 150.0, y:  50.0, z: 150.0},
//           ph: Point {x:  50.0, y:  50.0, z: 150.0},
//           //
//           color_abcd: CubeColor::White,
//           color_abef: CubeColor::Black,
//           color_bcfg: CubeColor::Blue,
//           color_cdgh: CubeColor::Black,
//           color_dahe: CubeColor::Black,
//           color_efgh: CubeColor::Black,
//         },
//         white_lime_edge: Cube,
//         white_orange_edge: Cube,
//         white_red_blue_corner: Cube,
//         white_blue_orange_corner: Cube,
//         white_orange_lime_corner: Cube,
//         white_lime_red_corner: Cube,
      
//         yellow_center: Cube,
//         yellow_red_edge: Cube,
//         yellow_lime_edge: Cube,
//         yellow_orange_edge: Cube,
//         yellow_blue_edge: Cube,
//         yellow_blue_red_corner: Cube,
//         yellow_red_lime_corner: Cube,
//         yellow_lime_orange_corner: Cube,
//         yellow_orange_blue_corner: Cube,
      
//         blue_center: Cube,
//         red_center: Cube,
//         orange_center: Cube,
//         lime_center: Cube,
      
//         blue_red_edge: Cube,
//         red_lime_edge: Cube,
//         lime_orange_edge: Cube,
//         orange_blue_edge: Cube
//       }
//   }
// }