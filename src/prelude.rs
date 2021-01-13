pub struct Float2 {
  pub x: f32,
  pub y: f32,
}
pub struct Float4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
  Float4 { x, y, z, w }
}
