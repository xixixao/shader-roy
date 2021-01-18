use crate::prelude::*;

pub fn pixel_color(coordinates: float2, size: float2) -> float4 {
  let _x = float2(2.0, 2.0);
  let _y = test();
  let mut d: float = 0.0;
  d = 3.0;
  float4(coordinates.x / size.x, coordinates.y / size.y, 0.0, 1.0)
}

fn test() -> float2 {
  float2(1.0, 2.0)
}
