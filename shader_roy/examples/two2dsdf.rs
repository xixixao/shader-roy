// Ported from
// https://github.com/electricsquare/raymarching-workshop#2d-sdf-demo

use metal_sl_prelude::*;

pub fn pixel_color(coordinates: Float2, input: PixelInput) -> Float4 {
  // project screen coordinate into world
  let p: Float2 = screen_to_world(coordinates, input.window_size);
  // signed distance for scene
  let sd: Float = sdf(p);
  // compute signed distance to a colour
  let col: Float3 = shade(sd);
  (col, 1.0).float4()
}

fn sdf(p: Float2) -> Float {
  // Example of the helpers
  op_blend(
    op_u(
      sd_circle(p, float2(-0.2, 0.3), 0.2),
      sd_circle(p, float2(-0.5, 0.3), 0.3),
    ),
    sd_box(p, float2(0.2, 0.3), 0.3.float2()),
  )
}

// --- SDF utility library
fn sd_circle(p: Float2, pos: Float2, radius: Float) -> Float {
  p.distance(pos) - radius
}

fn sd_box(p: Float2, pos: Float2, size: Float2) -> Float {
  let d: Float2 = (p - pos).abs() - size;
  d.x.clamped(d.y, 0.0) + d.max(0.0).magnitude()
}

// polynomial smooth min (k = 0.1);
fn smin_cubic(a: Float, b: Float, k: Float) -> Float {
  let h: Float = k - (a - b).abs().max(0.0);
  a.min(b) - h * h * h / (6.0 * k * k)
}

fn op_u(d1: Float, d2: Float) -> Float {
  d1.min(d2)
}

fn op_blend(d1: Float, d2: Float) -> Float {
  let k: Float = 0.2;
  smin_cubic(d1, d2, k)
}

// // --- Misc functions

// // https://www.shadertoy.com/view/ll2GD3
#[allow(clippy::many_single_char_names)]
fn palette(mut t: Float, a: Float3, b: Float3, c: Float3, d: Float3) -> Float3 {
  t = t.clamped(0., 1.);
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
    (0.5 - sd * 0.4).clamped(-max_dist, max_dist),
    float3(0.3, 0.3, 0.0),
    float3(0.8, 0.8, 0.1),
    float3(0.9, 0.7, 0.0),
    float3(0.3, 0.9, 0.8),
  );

  let mut col: Float3 = pal_col;
  // Darken around surface
  col = 0.4.mix(col, col * 1.0 - (-10.0 * sd.abs()).exp());
  // repeating lines
  col *= 0.8 + 0.2 * (150.0 * sd).cos();
  // White outline at surface
  col = (1.0 - sd.abs().smoothstep(0.0, 0.01)).mix(col, 1.0.float3());
  col
}
