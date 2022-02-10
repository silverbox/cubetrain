#[allow(dead_code)]
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

pub struct Point {
  x: f32,
  y: f32,
  z: f32
}

pub struct Cube {
  pub pa: Point, // 上左手前
  pub pb: Point, // 上右手前
  pub pc: Point, // 上右奥
  pub pd: Point, // 上左奥
  pub pe: Point, // 下左手前
  pub pf: Point, // 下右手前
  pub pg: Point, // 下右奥
  pub ph: Point, // 下左奥
  //
  pub color_abcd: CubeColor, // 上面
  pub color_abef: CubeColor, // 側面１
  pub color_bcfg: CubeColor, // 側面２
  pub color_cdgh: CubeColor, // 側面３
  pub color_dahe: CubeColor, // 側面４
  pub color_efgh: CubeColor, // 下面
  //
  pub x_axis_rotate_angle: f32,
  pub y_axis_rotate_angle: f32,
  pub z_axis_rotate_angle: f32,
}

impl Default for Cube {
  fn default() -> Self {
    Self {
      pa: Point {x: -50.0, y: -50.0, z:  50.0},
      pb: Point {x:  50.0, y: -50.0, z:  50.0},
      pc: Point {x:  50.0, y:  50.0, z:  50.0},
      pd: Point {x: -50.0, y:  50.0, z:  50.0},
      pe: Point {x: -50.0, y: -50.0, z: -50.0},
      pf: Point {x:  50.0, y: -50.0, z: -50.0},
      pg: Point {x:  50.0, y:  50.0, z: -50.0},
      ph: Point {x: -50.0, y:  50.0, z: -50.0},
      //
      color_abcd: CubeColor::White,
      color_abef: CubeColor::Red,
      color_bcfg: CubeColor::Blue,
      color_cdgh: CubeColor::Orange,
      color_dahe: CubeColor::Lime,
      color_efgh: CubeColor::Yellow,
      //
      x_axis_rotate_angle: 0.0,
      y_axis_rotate_angle: 0.0,
      z_axis_rotate_angle: 0.0,
    }
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
//  x軸：右側面方向
//  z軸：上方向
//  y軸：奥方向
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