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
  Float2 => {
    (all: Float),
    (x: Float, y: Float),
    (fr: Float2),
  },
  Float3 => {
    (all: Float),
    (x: Float, y: Float, z: Float),
    (x: Float, b: Float2),
    (a: Float2, z: Float),
    (fr: Float3),
  },
  Float4 => {
    (all: Float),
    (x: Float, y: Float, z: Float, w: Float),
    (a: Float2, b: Float2),
    (a: Float2, z: Float, w: Float),
    (x: Float, y: Float, c: Float2),
    (x: Float, b: Float2, c: Float),
    (a: Float3, w: Float),
    (x: Float, b: Float3),
    (fr: Float4),
  },
}
