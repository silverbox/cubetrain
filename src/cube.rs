#[allow(dead_code)]
#[allow(clippy::wildcard_imports)]

// use super::cubic_calc::Point;
use super::cubic_calc::*;

pub const ROTATE_STEP: f32 = PI / 16.0;
pub const CUBE_SIZE: f32 = 100.0;

#[derive(PartialEq)]
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
      size: CUBE_SIZE / 2.0,
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
      color_abcd: CubeColor::Black,
      color_abef: CubeColor::Black,
      color_bcfg: CubeColor::Black,
      color_cdgh: CubeColor::Black,
      color_dahe: CubeColor::Black,
      color_efgh: CubeColor::Black,
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
  pub fn is_visible_surface(&self, surface: &CubeSurface, camera: &CameraModel) -> bool {
    // if self.get_surface_color(surface) == &CubeColor::Black {
    //   return false;
    // }

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

    (normal_vec.x * center_vec.x) + (normal_vec.y * center_vec.y) + (normal_vec.z * center_vec.z) < 0.0
  }

  fn adjust_rad(rad: f32, rad_step: f32) -> f32 {
    let wk_rad1 = rad % (PI * 2.0);
    let wk_multiple = ((wk_rad1 / rad_step) + 0.5) as i32;
    (wk_multiple as f32) * rad_step
  }

  // Cubeの中心位置はそのまま。回転だけする
  pub fn rotate(&mut self, rad_x: f32, rad_y: f32, rad_z: f32, rad_step: f32) {
    self.x_axis_rotate_rad = Cube::adjust_rad(self.x_axis_rotate_rad + rad_x, rad_step);
    self.y_axis_rotate_rad = Cube::adjust_rad(self.y_axis_rotate_rad + rad_y, rad_step);
    self.z_axis_rotate_rad = Cube::adjust_rad(self.z_axis_rotate_rad + rad_z, rad_step);
  }

  fn get_surface_point_list(&self, surface: &CubeSurface) -> [&NormPoint; 4] {
    // 法線ベクトルを求めるために見せたい面の点を半時計回りにセットする
    match surface {
      CubeSurface::ABCD => [&self.pa, &self.pb, &self.pc, &self.pd] ,
      CubeSurface::ABEF => [&self.pa, &self.pe, &self.pf, &self.pb] ,
      CubeSurface::BCFG => [&self.pb, &self.pf, &self.pg, &self.pc] ,
      CubeSurface::CDGH => [&self.pc, &self.pg, &self.ph, &self.pd] ,
      CubeSurface::DAEH => [&self.pd, &self.ph, &self.pe, &self.pa] ,
      CubeSurface::EFGH => [&self.pe, &self.ph, &self.pg, &self.pf] ,
      // _ => [&self.pe, &self.pf, &self.pg, &self.ph]
    }
  }

  fn get_surface_color(&self, surface: &CubeSurface) -> &CubeColor {
    // 法線ベクトルを求めるために見せたい面の点を半時計回りにセットする
    match surface {
      CubeSurface::ABCD => &self.color_abcd,
      CubeSurface::ABEF => &self.color_abef,
      CubeSurface::BCFG => &self.color_bcfg,
      CubeSurface::CDGH => &self.color_cdgh,
      CubeSurface::DAEH => &self.color_dahe,
      CubeSurface::EFGH => &self.color_efgh,
      // _ => &CubeColor::Black
    }
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

  fn gettestcube() -> Cube {
    Cube {
      color_abcd: CubeColor::White,
      color_abef: CubeColor::Orange,
      color_bcfg: CubeColor::Lime,
      color_cdgh: CubeColor::Red,
      color_dahe: CubeColor::Blue,
      color_efgh: CubeColor::Yellow,
      ..Default::default()
    }
  }

  #[test]
  fn rotate_test() {
    let mut testcube = gettestcube();
    let point_a1 = testcube.get_abs_point("a");
    assert_eq!(point_a1.x, -CUBE_SIZE / 2.0);
    assert_eq!(point_a1.y,  CUBE_SIZE / 2.0);

    testcube.rotate_test();
    assert_eq!(testcube.x_axis_rotate_rad, ROTATE_STEP);

    let point_a2 = testcube.get_abs_point("a");
    let obj_norm_point2 = testcube.get_norm_point(&point_a2);
    assert_eq!(point_a2.x as i32, -(CUBE_SIZE / 2.0) as i32);

    testcube.rotate_test();
    testcube.rotate_test();
    testcube.rotate_test();
    let point_a3 = testcube.get_abs_point("a");
    assert_eq!(point_a3.x as i32, -(CUBE_SIZE / 2.0) as i32);
    assert_eq!(point_a3.y as i32,    0);
    assert_eq!(point_a3.z as i32,  (CUBE_SIZE * 0.707) as i32); // sqrt(2) / 2
  }

  #[test]
  fn visible_surface_test() {
    let testcube = gettestcube();
    let camera = &CameraModel::default();

    assert_eq!(testcube.is_visible_surface(&CubeSurface::ABCD, &camera), true);
    assert_eq!(testcube.is_visible_surface(&CubeSurface::ABEF, &camera), true);
    assert_eq!(testcube.is_visible_surface(&CubeSurface::BCFG, &camera), true);
    assert_eq!(testcube.is_visible_surface(&CubeSurface::CDGH, &camera), false);
    assert_eq!(testcube.is_visible_surface(&CubeSurface::DAEH, &camera), false);
    assert_eq!(testcube.is_visible_surface(&CubeSurface::EFGH, &camera), false);
  }
}

