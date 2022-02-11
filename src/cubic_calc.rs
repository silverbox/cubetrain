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
pub fn y_rotate(point: NormPoint, x_rad: f32) -> NormPoint {
  let sin = x_rad.sin();
  let cos = x_rad.cos();
  NormPoint {
    x: point.x * cos + point.z * sin,
    y: point.y,
    z: point.z * cos - point.x * sin,
    w: point.w
  }
}

// z軸中心回転
pub fn z_rotate(point: NormPoint, x_rad: f32) -> NormPoint {
  let sin = x_rad.sin();
  let cos = x_rad.cos();
  NormPoint {
    x: point.x * cos + point.z * sin,
    y: point.y,
    z: point.z * cos - point.x * sin,
    w: point.w
  }
}