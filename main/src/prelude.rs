#![allow(non_camel_case_types)]

pub type float = f32;

pub struct float2 {
  pub x: f32,
  pub y: f32,
}
pub struct float4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

pub fn float4(x: f32, y: f32, z: f32, w: f32) -> float4 {
  float4 { x, y, z, w }
}

pub fn float2(x: f32, y: f32) -> float2 {
  float2 { x, y }
}
