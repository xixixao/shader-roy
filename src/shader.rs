use crate::prelude::*;

pub fn pixel_color(coordinates: Float2) -> Float4 {
  float4(coordinates.x, coordinates.y, 0.0, 1.0)
}
