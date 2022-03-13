pub const PI: f32 = 3.14;

// pub struct Point {
//   pub x: f32,
//   pub y: f32,
//   pub z: f32
// }

// 正規化デバイス座標
// x軸：右側面方向
// y軸：上方向
// z軸：奥方向
pub struct NormPoint {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32
}

pub struct CameraVec {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

pub struct CameraAxisPoint {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32
}

pub struct ViewPoint2D {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32
}

// 視野錐台
pub struct ViewFrustum {
  pub left: f32,
  pub right: f32,
  pub top: f32,
  pub bottom: f32,
  pub near: f32,
  pub far: f32
}

pub struct CameraModel {
  pub pos: CameraVec,
  pub x_axis: CameraVec,
  pub y_axis: CameraVec,
  pub z_axis: CameraVec,
  pub view_frustum: ViewFrustum
}

// 中心と点Bを結ぶ方向から中心にカメラを向ける形
impl Default for CameraModel {
  fn default() -> Self {
    Self {
      pos: CameraVec { x: 200.0, y: 200.0, z: 200.0 },
      x_axis: CameraVec { x: -0.706, y:  0.0  , z:  0.706 },
      y_axis: CameraVec { x: -0.405, y:  0.810, z: -0.405 }, // vec X * vec Z
      z_axis: CameraVec { x: -0.577, y: -0.577, z: -0.577 },
      view_frustum: ViewFrustum { left: 200.0, right: -200.0, top: 200.0, bottom: -200.0, near: 100.0, far: 200.0 }
    }
  }
}

// 平行移動
pub fn shift(point: &NormPoint, tx: f32, ty: f32, tz: f32) -> NormPoint {
  NormPoint {
    x: point.x + point.w * tx,
    y: point.y + point.w * ty,
    z: point.z + point.w * tz,
    w: point.w
  }
}

// 拡大縮小
pub fn scale(point: &NormPoint, sx: f32, sy: f32, sz: f32) -> NormPoint {
  NormPoint {
    x: point.x * sx,
    y: point.y * sy,
    z: point.z * sz,
    w: point.w
  }
}

// x軸中心回転
pub fn x_rotate(point: &NormPoint, x_rad: f32) -> NormPoint {
  let sin = x_rad.sin();
  let cos = x_rad.cos();
  NormPoint {
    x: point.x,
    y: point.y * cos - point.z * sin,
    z: point.y * sin + point.z * cos,
    w: point.w
  }
}

// y軸中心回転
pub fn y_rotate(point: &NormPoint, y_rad: f32) -> NormPoint {
  let sin = y_rad.sin();
  let cos = y_rad.cos();
  NormPoint {
    x: point.x * cos + point.z * sin,
    y: point.y,
    z: point.z * cos - point.x * sin,
    w: point.w
  }
}

// z軸中心回転
pub fn z_rotate(point: &NormPoint, z_rad: f32) -> NormPoint {
  let sin = z_rad.sin();
  let cos = z_rad.cos();
  NormPoint {
    x: point.x * cos - point.y * sin,
    y: point.x * sin + point.y * cos,
    z: point.z,
    w: point.w
  }
}

// オイラー角回転
pub fn xyz_rotate(point: &NormPoint, x_rad: f32, y_rad: f32, z_rad: f32) -> NormPoint {
  let sin_a = x_rad.sin();
  let cos_a = x_rad.cos();
  let sin_b = y_rad.sin();
  let cos_b = y_rad.cos();
  let sin_c = z_rad.sin();
  let cos_c = z_rad.cos();
  NormPoint {
    x: point.x * (cos_c * cos_b) + point.y * (cos_c * sin_b * sin_a - cos_a * sin_c) + point.z * (sin_c * sin_a + cos_c * cos_a * sin_b),
    y: point.x * (cos_b * sin_c) + point.y * (cos_c * cos_a + sin_c * sin_b * sin_a) + point.z * (cos_a * sin_c * sin_b - cos_c * sin_a),
    z: - point.x * sin_b + point.y * cos_b * sin_a + point.z * cos_b * cos_a,
    w: point.w
  }
}

// 透視投影
pub fn perspective_projection(point: &NormPoint, camera: &CameraModel) -> CameraAxisPoint {
  let camera_vec = CameraVec {
    x: camera.x_axis.x * camera.pos.x + camera.x_axis.y * camera.pos.y + camera.x_axis.z * camera.pos.z,
    y: camera.y_axis.x * camera.pos.x + camera.y_axis.y * camera.pos.y + camera.y_axis.z * camera.pos.z,
    z: camera.z_axis.x * camera.pos.x + camera.z_axis.y * camera.pos.y + camera.z_axis.z * camera.pos.z,
  };
  CameraAxisPoint {
    x: camera.x_axis.x * point.x + camera.x_axis.y * point.y + camera.x_axis.z * point.z - camera_vec.x,
    y: camera.y_axis.x * point.x + camera.y_axis.y * point.y + camera.y_axis.z * point.z - camera_vec.y,
    z: camera.z_axis.x * point.x + camera.z_axis.y * point.y + camera.z_axis.z * point.z - camera_vec.z,
    w: point.w
  }
}

// 2D視点に変換
pub fn viewing_transform(camera_point: &CameraAxisPoint, vf: &ViewFrustum) -> ViewPoint2D {
  ViewPoint2D {
    x:  2.0 * vf.near * camera_point.x / (vf.right - vf.left) + camera_point.z * (vf.right + vf.left) / (vf.right - vf.left),
    y:  2.0 * vf.near * camera_point.y / (vf.top - vf.bottom) + camera_point.z * (vf.top + vf.bottom) / (vf.top - vf.bottom),
    z: -1.0 * camera_point.z * (vf.far + vf.near) / (vf.far - vf.near) - 2.0 * camera_point.w * vf.far * vf.near / (vf.far - vf.near),
    w: -1.0 * camera_point.z
  }
}

pub fn center_point(point_a: &ViewPoint2D, point_b: &ViewPoint2D, point_c: &ViewPoint2D) -> ViewPoint2D {
  ViewPoint2D {
    x: (point_a.x + point_b.x + point_c.x) / 3.0 ,
    y: (point_a.y + point_b.y + point_c.y) / 3.0 ,
    z: (point_a.z + point_b.z + point_c.z) / 3.0 ,
    w: (point_a.w + point_b.w + point_c.w) / 3.0
  }
}

pub fn get_permil(val: f32) -> i32 {
  (val * 1000.0) as i32
}

// ------ ------
//     Test
// ------ ------

#[cfg(test)]
mod tests {
  use super::*;

  const ROT_HALF_RIGHT_ANGLE: f32 = PI / 4.0;

  #[test]
  fn sincos_test() {
    assert_eq!(get_permil(ROT_HALF_RIGHT_ANGLE.sin()), 706);
  }

  #[test]
  fn rotate_test() {
    let org_point = NormPoint {x: -1.0, y:  1.0, z:  1.0, w: 1.0};
    let rot_x_point = x_rotate(&org_point, ROT_HALF_RIGHT_ANGLE);
    assert_eq!(get_permil(rot_x_point.y), 0);
    assert_eq!(get_permil(rot_x_point.z), 1414);

    let rot_y_point = y_rotate(&org_point, ROT_HALF_RIGHT_ANGLE);
    assert_eq!(get_permil(rot_y_point.x), 0);
    assert_eq!(get_permil(rot_y_point.y), 1000);
    assert_eq!(get_permil(rot_y_point.z), 1414);

    let rot_z_point = z_rotate(&org_point, ROT_HALF_RIGHT_ANGLE);
    assert_eq!(get_permil(rot_z_point.x), -1414);
    assert_eq!(get_permil(rot_z_point.y), 0);
    assert_eq!(get_permil(rot_z_point.z), 1000);
  }

  #[test]
  fn perspective_test() {
    let camera = &CameraModel::default();

    let org_point_a = NormPoint {x: -1.0, y:  1.0, z:  1.0, w: 1.0};
    let scaled_point_a = scale(&org_point_a, 100.0, 100.0, 100.0);
    assert_eq!(scaled_point_a.x, -100.0);
    assert_eq!(scaled_point_a.y,  100.0);
    assert_eq!(scaled_point_a.z,  100.0);

    let perspective_point = perspective_projection(&scaled_point_a, &camera);
    assert_eq!(perspective_point.x, 141.2);
    assert_eq!(perspective_point.y,  81.0);
    assert_eq!(perspective_point.z, 288.5);

    let org_point_b = NormPoint {x: 100.0, y: 100.0, z: 100.0, w: 1.0};
    let perspective_point_b = perspective_projection(&org_point_b, &camera);
    assert_eq!(perspective_point_b.x, 0.0);
    assert_eq!(perspective_point_b.y, 0.0);
    assert_eq!(perspective_point_b.z, 173.1);

    let org_point_c = NormPoint {x: 100.0, y: 100.0, z: -100.0, w: 1.0};
    let perspective_point_c = perspective_projection(&org_point_c, &camera);
    assert_eq!(perspective_point_c.x, -141.2);
    assert_eq!(perspective_point_c.y,   81.0);
    assert_eq!(perspective_point_c.z,  288.5);

    let org_point_f = NormPoint {x: 100.0, y: -100.0, z: 100.0, w: 1.0};
    let perspective_point_f = perspective_projection(&org_point_f, &camera);
    assert_eq!(perspective_point_f.x,    0.0);
    assert_eq!(perspective_point_f.y, -162.0);
    assert_eq!(perspective_point_f.z,  288.5);

    let org_point_e = NormPoint {x: -100.0, y: -100.0, z: 100.0, w: 1.0};
    let perspective_point_e = perspective_projection(&org_point_e, &camera);
    assert_eq!(perspective_point_e.x, 141.2);
    assert_eq!(perspective_point_e.y, -81.0);
    assert_eq!(get_permil(perspective_point_e.z), 403900);

    let vf = &camera.view_frustum;

    let view_p_a = viewing_transform(&perspective_point, vf);
    print!("view a x={}, y={}\n", view_p_a.x, view_p_a.y);
    assert_eq!(view_p_a.x, -70.6);
    assert_eq!(view_p_a.y,  40.5);

    let view_p_e = viewing_transform(&perspective_point_e, vf);
    print!("view e x={}, y={}\n", view_p_e.x, view_p_e.y);
    assert_eq!(view_p_e.x, -70.6);
    assert_eq!(view_p_e.y, -40.5);
  }
}
