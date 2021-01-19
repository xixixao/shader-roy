#![allow(unused_variables)]
#![allow(unstable_name_collisions)]

use crate::prelude::*;

// pub fn pixel_color(coordinates: float2, size: float2) -> float4 {
//   let _x = float2(2.0, 2.0);
//   let _y = test();
//   let mut d: float = 1000.0;
//   d = d.min(100.0);

//   // d = min(d, sdCircle(p, vec2(-0.1, 0.4), 0.15));
//   // d = min(d, sdCircle(p, vec2( 0.5, 0.1), 0.35));

//   // return d;
//   float4(coordinates.x / size.x, coordinates.y / size.y, 0.0, 1.0)
// }

// fn test() -> float2 {
//   float2(1.0, 2.0)
// }

pub fn pixel_color(coordinates: Float2, size: Float2) -> Float4 {
  // project screen coordinate into world
  let p: Float2 = screen_to_world(coordinates, size);
  // signed distance for scene
  let sd: Float = sdf(p);
  // compute signed distance to a colour
  let col: Float3 = shade(sd);
  col.float4_2(1.0)
}

fn sdf(p: Float2) -> Float {
  0.6 - p.length()
}

// }

// // --- SDF utility library

// float sdCircle(in Float2 p, in Float2 pos, float radius)
// {
//     return length(p-pos)-radius;
// }

// float sdBox(in Float2 p, in Float2 pos, in Float2 size)
// {
//     Float2 d = abs(p-pos)-size;
//     return min(0.0, max(d.x, d.y))+length(max(d,0.0));
// }

// // polynomial smooth min (k = 0.1);
// float sminCubic(float a, float b, float k)
// {
//     float h = max(k-abs(a-b), 0.0);
//     return min(a, b) - h*h*h/(6.0*k*k);
// }

// float opU(float d1, float d2)
// {
//     return min(d1, d2);
// }

// float opBlend(float d1, float d2)
// {
//     float k = 0.2;
//     return sminCubic(d1, d2, k);
// }

// // --- Misc functions

// // https://www.shadertoy.com/view/ll2GD3
#[allow(clippy::many_single_char_names)]
fn palette(mut t: Float, a: Float3, b: Float3, c: Float3, d: Float3) -> Float3 {
  t = t.clamp(0., 1.);
  a + b * (6.28318 * (c * t + d)).cos()
}

fn screen_to_world(screen: Float2, size: Float2) -> Float2 {
  let mut result: Float2 = 2.0 * (screen / size - 0.5);
  result.x *= size.x / size.y;
  result
}

fn shade(sd: Float) -> Float3 {
  let max_dist: Float = 2.0;
  let pal_col: Float3 = palette(
    (0.5 - sd * 0.4).clamp(-max_dist, max_dist),
    float3(0.3, 0.3, 0.0),
    float3(0.8, 0.8, 0.1),
    float3(0.9, 0.7, 0.0),
    float3(0.3, 0.9, 0.8),
  );

  let mut col: Float3 = pal_col;
  // Darken around surface
  col = mix(col, col * 1.0 - (-10.0 * sd.abs()).exp(), 0.4);
  // col = 0.4.mix(col, col * 1.0 - (-10.0 * sd.abs()).exp());
  // repeating lines
  col *= 0.8 + 0.2 * (150.0 * sd).cos();
  // White outline at surface
  // col = col.mix(1.0.float3_1(), 1.0 - sd.abs().smoothstep(0.0, 0.01));
  col
}
