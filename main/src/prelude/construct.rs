use super::*;

pub fn float2(x: f32, y: f32) -> Float2 {
  Float2 { x, y }
}

pub fn float3(x: f32, y: f32, z: f32) -> Float3 {
  Float3 { x, y, z }
}

pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
  Float4 { x, y, z, w }
}

prelude_macros::implement_constructors! {
  Float2 => [1, 2] => {
    (Float),
    (Float, b: Float),
    (Float2),
  },
  Float3 => [1, 2, 3] => {
    (Float),
    (Float, y: Float, z: Float),
    (Float, b: Float2),
    (Float2, b: Float),
    (Float3),
  },
  Float4 => [1, 2, 3, 4] => {
    (Float),
    (Float, y: Float, z: Float, w: Float),
    (Float2, b: Float2),
    (Float2, b: Float, c: Float),
    (Float, b: Float, c: Float2),
    (Float, b: Float2, c: Float),
    (Float3, b: Float),
    (Float, b: Float3),
    (Float4),
  },
}
