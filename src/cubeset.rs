#[allow(dead_code)]
#[allow(clippy::wildcard_imports)]

use super::cube::Cube;
use super::cube::ROTATE_STEP;
use super::cube::CUBE_SIZE;
use super::cube::CubeColor;
use super::cubic_calc::*;

pub struct CubeSet {
  white_center: Cube,
  white_red_edge: Cube,
  white_blue_edge: Cube,
  white_lime_edge: Cube,
  white_orange_edge: Cube,
  white_red_blue_corner: Cube,
  white_blue_orange_corner: Cube,
  pub white_orange_lime_corner: Cube,
  white_lime_red_corner: Cube,

  yellow_center: Cube,
  yellow_red_edge: Cube,
  yellow_lime_edge: Cube,
  yellow_orange_edge: Cube,
  yellow_blue_edge: Cube,
  yellow_blue_red_corner: Cube,
  yellow_red_lime_corner: Cube,
  yellow_lime_orange_corner: Cube,
  yellow_orange_blue_corner: Cube,

  blue_center: Cube,
  red_center: Cube,
  orange_center: Cube,
  lime_center: Cube,

  blue_red_edge: Cube,
  red_lime_edge: Cube,
  lime_orange_edge: Cube,
  orange_blue_edge: Cube
}

/**
 白上面、オレンジ正面、ライム右側面。キューブの辺の長さを100として、キューブ集合体の中心座標が(0,0,0)
 ※白中心ピースの方向は、必ずしもこの方向にはならない（時には読める方向にすると、ライム正面、赤右側面になったりする）
 */
impl Default for CubeSet {
  fn default() -> Self {
    Self {
      white_center: Cube {
        center_point: NormPoint {x: 0.0, y: CUBE_SIZE, z: 0.0, w: 1.0},
        color_abcd: CubeColor::White,
        ..Default::default()
      },
      white_red_edge: Cube {
        center_point: NormPoint {x: 0.0, y: CUBE_SIZE, z: -CUBE_SIZE, w: 1.0},
        color_abcd: CubeColor::White,
        color_cdgh: CubeColor::Red,
        ..Default::default()
      },
      white_blue_edge: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: CUBE_SIZE, z: 0.0, w: 1.0},
        color_abcd: CubeColor::White,
        color_dahe: CubeColor::Blue,
        ..Default::default()
      },
      white_lime_edge: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: CUBE_SIZE, z: 0.0, w: 1.0},
        color_abcd: CubeColor::White,
        color_bcfg: CubeColor::Lime,
        ..Default::default()
      },
      white_orange_edge: Cube {
        center_point: NormPoint {x: 0.0, y: CUBE_SIZE, z: CUBE_SIZE, w: 1.0},
        color_abcd: CubeColor::White,
        color_abef: CubeColor::Orange,
        ..Default::default()
      },
      //
      white_red_blue_corner: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: CUBE_SIZE, z: -CUBE_SIZE, w: 1.0},
        color_abcd: CubeColor::White,
        color_cdgh: CubeColor::Red,
        color_dahe: CubeColor::Blue,
        ..Default::default()
      },
      white_blue_orange_corner: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: CUBE_SIZE, z: CUBE_SIZE, w: 1.0},
        color_abcd: CubeColor::White,
        color_dahe: CubeColor::Blue,
        color_abef: CubeColor::Orange,
        ..Default::default()
      },
      white_orange_lime_corner: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: CUBE_SIZE, z: CUBE_SIZE, w: 1.0},
        color_abcd: CubeColor::White,
        color_bcfg: CubeColor::Lime,
        color_abef: CubeColor::Orange,
        ..Default::default()
      },
      white_lime_red_corner: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: CUBE_SIZE, z: -CUBE_SIZE, w: 1.0},
        //
        color_abcd: CubeColor::White,
        color_bcfg: CubeColor::Lime,
        color_cdgh: CubeColor::Red,
        ..Default::default()
      },
      //
      yellow_center: Cube {
        center_point: NormPoint {x: 0.0, y: -CUBE_SIZE, z: 0.0, w: 1.0},
        color_efgh: CubeColor::Yellow,
        ..Default::default()
      },
      //
      yellow_red_edge: Cube {
        center_point: NormPoint {x: 0.0, y: -CUBE_SIZE, z: -CUBE_SIZE, w: 1.0},
        color_cdgh: CubeColor::Red,
        color_efgh: CubeColor::Yellow,
        ..Default::default()
      },
      yellow_lime_edge: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: -CUBE_SIZE, z: 0.0, w: 1.0},
        color_bcfg: CubeColor::Lime,
        color_efgh: CubeColor::Yellow,
        ..Default::default()
      },
      yellow_orange_edge: Cube {
        center_point: NormPoint {x: 0.0, y: -CUBE_SIZE, z: CUBE_SIZE, w: 1.0},
        color_abef: CubeColor::Orange,
        color_efgh: CubeColor::Yellow,
        ..Default::default()
      },
      yellow_blue_edge: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: -CUBE_SIZE, z: 0.0, w: 1.0},
        color_dahe: CubeColor::Blue,
        color_efgh: CubeColor::Yellow,
        ..Default::default()
      },
      //
      yellow_blue_red_corner: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: -CUBE_SIZE, z: -CUBE_SIZE, w: 1.0},
        color_cdgh: CubeColor::Red,
        color_dahe: CubeColor::Blue,
        color_efgh: CubeColor::Yellow,
        ..Default::default()
      },
      yellow_red_lime_corner: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: -CUBE_SIZE, z: -CUBE_SIZE, w: 1.0},
        color_bcfg: CubeColor::Lime,
        color_cdgh: CubeColor::Red,
        color_efgh: CubeColor::Yellow,
        ..Default::default()
      },
      yellow_lime_orange_corner: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: -CUBE_SIZE, z: CUBE_SIZE, w: 1.0},
        color_bcfg: CubeColor::Lime,
        color_abef: CubeColor::Orange,
        color_efgh: CubeColor::Yellow,
        ..Default::default()
      },
      yellow_orange_blue_corner: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: -CUBE_SIZE, z: CUBE_SIZE, w: 1.0},
        color_dahe: CubeColor::Blue,
        color_abef: CubeColor::Orange,
        color_efgh: CubeColor::Black,
        ..Default::default()
      },
      //
      blue_center: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: 0.0, z: 0.0, w: 1.0},
        color_dahe: CubeColor::Blue,
        ..Default::default()
      },
      red_center: Cube {
        center_point: NormPoint {x: 0.0, y: 0.0, z: -CUBE_SIZE, w: 1.0},
        color_cdgh: CubeColor::Red,
        ..Default::default()
      },
      orange_center: Cube {
        center_point: NormPoint {x: 0.0, y: 0.0, z: CUBE_SIZE, w: 1.0},
        color_abef: CubeColor::Orange,
        ..Default::default()
      },
      lime_center: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: 0.0, z: 0.0, w: 1.0},
        color_bcfg: CubeColor::Lime,
        ..Default::default()
      },
      //
      blue_red_edge: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: 0.0, z: -CUBE_SIZE, w: 1.0},
        color_cdgh: CubeColor::Red,
        color_dahe: CubeColor::Blue,
        ..Default::default()
      },
      red_lime_edge: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: 0.0, z: -CUBE_SIZE, w: 1.0},
        color_bcfg: CubeColor::Lime,
        color_cdgh: CubeColor::Red,
        ..Default::default()
      },
      lime_orange_edge: Cube {
        center_point: NormPoint {x: CUBE_SIZE, y: 0.0, z: CUBE_SIZE, w: 1.0},
        color_bcfg: CubeColor::Lime,
        color_abef: CubeColor::Orange,
        ..Default::default()
      },
      orange_blue_edge: Cube {
        center_point: NormPoint {x: -CUBE_SIZE, y: 0.0, z: CUBE_SIZE, w: 1.0},
        color_dahe: CubeColor::Blue,
        color_abef: CubeColor::Orange,
        ..Default::default()
      },
    }
  }
}

impl CubeSet {
  pub fn rotate_test(&mut self) {
    self.white_orange_lime_corner.rotate(ROTATE_STEP, 0.0, 0.0, ROTATE_STEP);
  }

  pub fn get_all_cube(&self) -> [&Cube; 26] {
    [
      &self.white_center,
      &self.white_red_edge,
      &self.white_blue_edge,
      &self.white_lime_edge,
      &self.white_orange_edge,
      &self.white_red_blue_corner,
      &self.white_blue_orange_corner,
      &self.white_orange_lime_corner,
      &self.white_lime_red_corner,
    
      &self.yellow_center,
      &self.yellow_red_edge,
      &self.yellow_lime_edge,
      &self.yellow_orange_edge,
      &self.yellow_blue_edge,
      &self.yellow_blue_red_corner,
      &self.yellow_red_lime_corner,
      &self.yellow_lime_orange_corner,
      &self.yellow_orange_blue_corner,
    
      &self.blue_center,
      &self.red_center,
      &self.orange_center,
      &self.lime_center,
    
      &self.blue_red_edge,
      &self.red_lime_edge,
      &self.lime_orange_edge,
      &self.orange_blue_edge
    ]
  }
}
