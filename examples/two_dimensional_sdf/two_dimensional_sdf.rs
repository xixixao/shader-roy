// Ported from
// https://github.com/electricsquare/raymarching-workshop#2d-sdf-demo

use shader_roy_metal_sl_interface::*;

pub fn pixel_color(coordinates: Vec2) -> Vec4 {
  // project screen coordinate into world
  let p = screen_to_world(coordinates, INPUT.window_size);
  // signed distance for scene
  let sd = sdf(p);
  // compute signed distance to a colour
  let col = shade(sd);
  (col, 1.0).vec4()
}

fn sdf(p: Vec2) -> f32 {
  // Example of the helpers
  op_blend(
    op_u(
      sd_circle(p, (-0.2, 0.3).vec2(), 0.2),
      sd_circle(p, (-0.5, 0.3).vec2(), 0.3),
    ),
    sd_box(p, (0.2, 0.3).vec2(), 0.3.vec2()),
  )
}

// --- SDF utility library
fn sd_circle(p: Vec2, pos: Vec2, radius: f32) -> f32 {
  p.distance(pos) - radius
}

fn sd_box(p: Vec2, pos: Vec2, size: Vec2) -> f32 {
  let d: Vec2 = (p - pos).abs() - size;
  d.x.clamped(d.y, 0.0) + d.max(0.0).magnitude()
}

// polynomial smooth min (k = 0.1);
fn smin_cubic(a: f32, b: f32, k: f32) -> f32 {
  let h: f32 = k - (a - b).abs().max(0.0);
  a.min(b) - h * h * h / (6.0 * k * k)
}

fn op_u(d1: f32, d2: f32) -> f32 {
  d1.min(d2)
}

fn op_blend(d1: f32, d2: f32) -> f32 {
  let k: f32 = 0.2;
  smin_cubic(d1, d2, k)
}

// // --- Misc functions

// // https://www.shadertoy.com/view/ll2GD3
#[allow(clippy::many_single_char_names)]
fn palette(mut t: f32, a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Vec3 {
  t = t.clamped(0., 1.);
  a + b * (6.28318 * (c * t + d)).cos()
}

fn screen_to_world(screen: Vec2, size: Vec2) -> Vec2 {
  let mut result: Vec2 = 2.0 * (screen / size - 0.5);
  result.x *= size.x / size.y;
  result
}

fn shade(sd: f32) -> Vec3 {
  let max_dist: f32 = 2.0;
  let pal_col: Vec3 = palette(
    (0.5 - sd * 0.4).clamped(-max_dist, max_dist),
    (0.3, 0.3, 0.0).vec3(),
    (0.8, 0.8, 0.1).vec3(),
    (0.9, 0.7, 0.0).vec3(),
    (0.3, 0.9, 0.8).vec3(),
  );

  let mut col: Vec3 = pal_col;
  // Darken around surface
  col = 0.4.mix(col, col * 1.0 - (-10.0 * sd.abs()).exp());
  // repeating lines
  col *= 0.8 + 0.2 * (150.0 * sd).cos();
  // White outline at surface
  col = (1.0 - sd.abs().smoothstep(0.0, 0.01)).mix(col, 1.0.vec3());
  col
}
