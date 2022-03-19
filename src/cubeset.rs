#[allow(dead_code)]
#[allow(clippy::wildcard_imports)]

use rand::prelude::*;

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
impl RotateAxis {
  fn random() -> Self {
    let mut rng = rand::thread_rng();
    let axisval: f32 = rng.gen::<f32>() * 3.0;
    if axisval < 1.0 {
      RotateAxis::X
    } else if axisval < 2.0 {
      RotateAxis::Y
    } else {
      RotateAxis::Z
    }
  }
}

pub enum RotateLayer {
  Positive,
  Neutral,
  Negative,
  All
}
impl RotateLayer {
  fn random() -> Self {
    let mut rng = rand::thread_rng();
    let axisval: f32 = rng.gen::<f32>() * 3.0;
    if axisval < 1.0 {
      RotateLayer::Positive
    } else if axisval < 2.0 {
      RotateLayer::Neutral
    } else {
      RotateLayer::Negative
    }
  }
}

pub struct RotateTarget {
  pub axis: RotateAxis,
  pub layer: RotateLayer,
  pub rad: f32
}
impl Default for RotateTarget {
  fn default() -> Self {
    Self {
      axis: RotateAxis::X,
      layer: RotateLayer::Positive,
      rad: 0.0
    }
  }
}
impl RotateTarget {
  pub fn random(rad: f32) -> Self {
    let mut rng = rand::thread_rng();
    Self {
      axis: RotateAxis::random(),
      layer: RotateLayer::random(),
      rad: if rng.gen::<bool>() { rad } else { -rad }
    }
  }
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
        color_efgh: CubeColor::Yellow,
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
      match rotate_target.axis {
        RotateAxis::X => {
          cube.rotate(rotate_target.rad, 0.0, 0.0);
          let new_center = xyz_rotate(&cube.center_point, rotate_target.rad, 0.0, 0.0);
          cube.center_point = new_center;
        },
        RotateAxis::Y => {
          cube.rotate(0.0, rotate_target.rad, 0.0);
          let new_center = xyz_rotate(&cube.center_point, 0.0, rotate_target.rad, 0.0);
          cube.center_point = new_center;
        },
        RotateAxis::Z => {
          cube.rotate(0.0, 0.0, rotate_target.rad);
          let new_center = xyz_rotate(&cube.center_point, 0.0, 0.0, rotate_target.rad);
          cube.center_point = new_center;
        }
      };
      CubeSet::adjust_position(cube);
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
      if CubeSet::is_target_layer(chkval, &rotate_target.layer) {
        retcubes.push(cube);
      };
    };
    retcubes
  }

  pub fn rotate_test(&mut self) {
    let rt = RotateTarget {
      axis: RotateAxis::X,
      layer: RotateLayer::Positive,
      rad: ROTATE_STEP
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

  fn is_target_layer(axis_val: f32, layer: &RotateLayer) -> bool {
    let threshold = CUBE_SIZE / 5.0;
    match layer {
      RotateLayer::All => true,
      RotateLayer::Positive => axis_val > threshold,
      RotateLayer::Neutral => {axis_val >= -threshold && axis_val <= threshold},
      RotateLayer::Negative => axis_val < -threshold
    }
  }

  fn adjust_position(cube: &mut Cube) {
    let threshold = CUBE_SIZE / 100.0;
    if cube.center_point.x < (CUBE_SIZE + threshold) && cube.center_point.x > (CUBE_SIZE - threshold) {
      cube.center_point.x = CUBE_SIZE;
    } else if cube.center_point.x < threshold && cube.center_point.x > -threshold {
      cube.center_point.x = 0.0;
    } else if cube.center_point.x < - (CUBE_SIZE - threshold) && cube.center_point.x > - (CUBE_SIZE + threshold) {
      cube.center_point.x = - CUBE_SIZE;
    }
    if cube.center_point.y < (CUBE_SIZE + threshold) && cube.center_point.y > (CUBE_SIZE - threshold) {
      cube.center_point.y = CUBE_SIZE;
    } else if cube.center_point.y < threshold && cube.center_point.y > -threshold {
      cube.center_point.y = 0.0;
    } else if cube.center_point.y < - (CUBE_SIZE - threshold) && cube.center_point.y > - (CUBE_SIZE + threshold) {
      cube.center_point.y = - CUBE_SIZE;
    }
    if cube.center_point.z < (CUBE_SIZE + threshold) && cube.center_point.z > (CUBE_SIZE - threshold) {
      cube.center_point.z = CUBE_SIZE;
    } else if cube.center_point.z < threshold && cube.center_point.z > -threshold {
      cube.center_point.z = 0.0;
    } else if cube.center_point.z < - (CUBE_SIZE - threshold) && cube.center_point.z > - (CUBE_SIZE + threshold) {
      cube.center_point.z = - CUBE_SIZE;
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rotate_layer_test() {
    let mut testcubeset = CubeSet::default();
    let r_step = ROTATE_STEP * 8.0;
    let rotate_xp = RotateTarget {
      axis: RotateAxis::X,
      layer: RotateLayer::Positive,
      rad: r_step
    };
    let rotate_yn = RotateTarget {
      axis: RotateAxis::Y,
      layer: RotateLayer::Negative,
      rad: r_step
    };

    testcubeset.rotate_layer(&rotate_xp);
    let cube_wol = &testcubeset.white_orange_lime_corner;
    assert_eq!(cube_wol.center_point.y as i32, - CUBE_SIZE as i32);
    // assert_eq!(cube_wol.x_axis_rotate_rad, r_step);

    testcubeset.rotate_layer(&rotate_yn);
    let cube_yl = &testcubeset.yellow_lime_edge;
    let cube_wol2 = &testcubeset.white_orange_lime_corner;
    assert_eq!(cube_yl.center_point.x as i32, CUBE_SIZE as i32);
    assert_eq!(cube_yl.center_point.y as i32, 0);
    assert_eq!(cube_yl.center_point.z as i32, - CUBE_SIZE as i32);
    // assert_eq!(cube_wol2.x_axis_rotate_rad, r_step);
    // assert_eq!(cube_wol2.y_axis_rotate_rad, r_step);
  }

  #[test]
  fn rotate_layer_test2() {
    let mut testcubeset = CubeSet::default();
    let r_step = ROTATE_STEP * 8.0;
    let rotate_xp = RotateTarget {
      axis: RotateAxis::X,
      layer: RotateLayer::Positive,
      rad: r_step
    };
    let rotate_zp = RotateTarget {
      axis: RotateAxis::Z,
      layer: RotateLayer::Positive,
      rad: r_step
    };
    testcubeset.rotate_layer(&rotate_zp);
    testcubeset.rotate_layer(&rotate_xp);
    let cube_ylo = &testcubeset.yellow_lime_orange_corner;
    // assert_eq!(cube_ylo.x_axis_rotate_rad, r_step);
    // assert_eq!(cube_ylo.y_axis_rotate_rad, 0.0);
    // assert_eq!(cube_ylo.z_axis_rotate_rad, r_step);
  }

  #[test]
  fn rotate_layer_test3() {
    let mut testcubeset = CubeSet::default();
    let r_step = ROTATE_STEP * 8.0;
    let rotate_xp = RotateTarget {
      axis: RotateAxis::X,
      layer: RotateLayer::Positive,
      rad: r_step
    };
    let rotate_zp = RotateTarget {
      axis: RotateAxis::Z,
      layer: RotateLayer::Positive,
      rad: r_step
    };
    testcubeset.rotate_layer(&rotate_xp);
    testcubeset.rotate_layer(&rotate_zp);
    let cube_wol = &testcubeset.white_orange_lime_corner;
    // assert_eq!(cube_wol.x_axis_rotate_rad, r_step);
    // assert_eq!(cube_wol.y_axis_rotate_rad, 0.0);
    // assert_eq!(cube_wol.z_axis_rotate_rad, r_step);
  }

  #[test]
  fn random_target_test() {
    let rotate_rand = RotateTarget::random(PI / 2.0);
  }
}