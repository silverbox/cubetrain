#[allow(dead_code)]
pub enum CubeColor {
  White,
  Yellow,
  Blue,   // 青
  Red, // 赤
  Orange, // オレンジ
  Lime    // 黄緑
}

impl CubeColor {
  pub const fn as_int(&self) -> i32 {
      match self {
          Self::White => 0xffffff,
          Self::Yellow => 0xffff00,
          Self::Blue => 0x0000ff,
          Self::Red => 0xff4500,
          Self::Orange => 0xff4500,
          Self::Lime => 0x00ff00
      }
  }
  pub const fn as_css_str(&self) -> &str {
    match self {
        Self::White => "white",
        Self::Yellow => "yellow",
        Self::Blue => "navy",
        Self::Red => "maroon",
        Self::Orange => "orangered",
        Self::Lime => "lime"
    }
  }
}