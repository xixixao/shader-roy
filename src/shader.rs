use crate::prelude::*;

pub fn pixel_color(coordinates: float2, size: float2) -> float4 {
  let x: float2 = float2(1.0, 1.0);
  float4(coordinates.x / 255.0, coordinates.y / 255.0, 0.0, 1.0)
}
