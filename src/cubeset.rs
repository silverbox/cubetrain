#[allow(dead_code)]
#[allow(clippy::wildcard_imports)]

use super::cube::Cube;
use super::cube::ROTATE_STEP;
use super::cube::CUBE_SIZE;
use super::cube::CubeColor;
use super::cubic_calc::*;

// 右手座標系
// x軸：右側面方向
// y軸：上方向
// z軸：手前方向
pub enum RotateAxis {
  X,
  Y,
  Z
}
pub enum RotateLayer {
  Positive,
  Neutral,
  Negative
}
pub struct RotateTarget {
  axis: RotateAxis,
  layer: RotateLayer
}

pub struct CubeSet {
  white_center: Cube,
  white_red_edge: Cube,
  white_blue_edge: Cube,
  white_lime_edge: Cube,
  white_orange_edge: Cube,
  white_red_blue_corner: Cube,
  white_blue_orange_corner: Cube,
  white_orange_lime_corner: Cube,
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
  orange_blue_edge: Cube,

  center: Cube
}

fn is_target_layer(axis_val: f32, layer: &RotateLayer) -> bool {
  match layer {
    RotateLayer::Positive => axis_val > 0.1,
    RotateLayer::Neutral => {axis_val >= -0.1 && axis_val <= 0.1},
    RotateLayer::Negative => axis_val < -0.1
  }
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
      center: Cube::default()
    }
  }
}

impl CubeSet {

  pub fn get_all_cube(&self) -> Vec<&Cube> {
    vec![
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
      &self.orange_blue_edge,

      &self.center,
    ]
  }
  pub fn get_all_cube_mut(&mut self) -> Vec<&mut Cube> {
    vec![
      &mut self.white_center,
      &mut self.white_red_edge,
      &mut self.white_blue_edge,
      &mut self.white_lime_edge,
      &mut self.white_orange_edge,
      &mut self.white_red_blue_corner,
      &mut self.white_blue_orange_corner,
      &mut self.white_orange_lime_corner,
      &mut self.white_lime_red_corner,
    
      &mut self.yellow_center,
      &mut self.yellow_red_edge,
      &mut self.yellow_lime_edge,
      &mut self.yellow_orange_edge,
      &mut self.yellow_blue_edge,
      &mut self.yellow_blue_red_corner,
      &mut self.yellow_red_lime_corner,
      &mut self.yellow_lime_orange_corner,
      &mut self.yellow_orange_blue_corner,
    
      &mut self.blue_center,
      &mut self.red_center,
      &mut self.orange_center,
      &mut self.lime_center,
    
      &mut self.blue_red_edge,
      &mut self.red_lime_edge,
      &mut self.lime_orange_edge,
      &mut self.orange_blue_edge,

      &mut self.center,
    ]
  }

  pub fn rotate_layer(&mut self, rotate_target: &RotateTarget) {
    for cube in self.get_target_cubes(rotate_target).into_iter() {
      cube.rotate(ROTATE_STEP, 0.0, 0.0, ROTATE_STEP);
      match rotate_target.axis {
        RotateAxis::X => {
          let new_center = x_rotate(&cube.center_point, ROTATE_STEP);
          cube.center_point = new_center;
        },
        RotateAxis::Y => {},
        RotateAxis::Z => {}
      };
    };
  }

  fn get_target_cubes(&mut self, rotate_target: &RotateTarget) -> Vec<&mut Cube> {
    // let mut retcubes: Vec<&mut Cube> = Vec::with_capacity(9);
    let mut retcubes = vec![];
    for cube in self.get_all_cube_mut().into_iter() {
      let chkval = match rotate_target.axis {
        RotateAxis::X => cube.center_point.x,
        RotateAxis::Y => cube.center_point.y,
        RotateAxis::Z => cube.center_point.z
      };
      if is_target_layer(chkval, &rotate_target.layer) {
        retcubes.push(cube);
      };
    };
    retcubes
  }

  pub fn rotate_test(&mut self) {
    let rt = RotateTarget {
      axis: RotateAxis::X,
      layer: RotateLayer::Positive
    };
    self.rotate_layer(&rt)
    // for cube in self.get_target_cubes(&rt).iter_mut() {
    //   cube.rotate(ROTATE_STEP, 0.0, 0.0, ROTATE_STEP);
    // }
  }

  fn get_test_cubes(&mut self) -> [&mut Cube; 9] {
    [
      &mut self.white_lime_edge,
      &mut self.white_orange_lime_corner,
      &mut self.white_lime_red_corner,
      &mut self.lime_center,
      &mut self.red_lime_edge,
      &mut self.lime_orange_edge,
      &mut self.yellow_lime_edge,
      &mut self.yellow_red_lime_corner,
      &mut self.yellow_lime_orange_corner
    ]
  }
}
