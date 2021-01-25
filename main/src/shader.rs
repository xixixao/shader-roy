#![allow(clippy::float_cmp)]
#![allow(dead_code)]
use crate::msl_prelude::*;

pub fn pixel_color(coordinates: Float2, input: PixelInput) -> Float4 {
  let cam_pos = float3(0.0, 0.0, -1.0);
  let cam_target = float3(0.0, 0.0, 0.0);

  let uv = screen_to_world(coordinates, input.window_size);
  let ray_dir = get_camera_ray_dir(uv, cam_pos, cam_target);

  let col = render(cam_pos, ray_dir);
  (col, 1.0).float4()
}

fn sdf(pos: Float3) -> Float {
  // Example of the helpers
  // length(uint2(p))
  sd_sphere(pos, float3(0.0, 0.0, 10.0), 3.0)
  // op_blend(
  //   subtract(
  //     sd_circle(p, float2(elapsed_time_secs.sin(), 0.3), 0.2),
  //     op_u(
  //       sd_circle(p, float2(-0.5, 0.3), 0.3),
  //       sd_circle(p, float2(elapsed_time_secs.sin().cos(), 0.3), 0.3),
  //     ),
  //   ),
  //   sd_box(p, float2(0.2, 0.3), 0.3.float2()),
  // )
}

fn render(ray_origin: Float3, ray_dir: Float3) -> Float3 {
  let t = cast_ray(ray_origin, ray_dir);
  if t == -1.0 {
    // Skybox colour
    float3(0.30, 0.36, 0.60) - (ray_dir.y * 0.7)
  } else {
    let object_surface_colour = float3(0.4, 0.8, 0.1);
    let ambient = float3(0.02, 0.021, 0.02);
    ambient * object_surface_colour
  }
}

fn cast_ray(ray_origin: Float3, ray_dir: Float3) -> Float {
  let mut t = 0.0; // Stores current distance along ray

  for _ in 0..64 {
    let res = sdf(ray_origin + ray_dir * t);
    if res < (0.0001 * t) {
      return t;
    }
    t += res;
  }

  -1.0
}

fn calcNormal(pos: Float3) -> Float3 {
  // Center sample
  let c = sdf(pos);
  // Use offset samples to compute gradient / normal
  let eps_zero = float2(0.001, 0.0);
  return (float3(
    // TODO: Create all permutation accessors for all vectors up to 4th dimension
    sdf(pos + eps_zero.xyy()),
    sdf(pos + eps_zero.yxy()),
    sdf(pos + eps_zero.yyx()),
  ) - c)
    .normalized();
}

fn get_camera_ray_dir(uv: Float2, cam_pos: Float3, cam_target: Float3) -> Float3 {
  // Calculate camera's "orthonormal basis", i.e. its transform matrix components
  let cam_forward = (cam_target - cam_pos).normalized();
  let cam_right = (float3(0.0, 1.0, 0.0).cross(cam_forward)).normalized();
  let cam_up = (cam_forward.cross(cam_right)).normalized();

  let f_persp = 2.0;
  (uv.x * cam_right + uv.y * cam_up + cam_forward * f_persp).normalized()
}

// --- SDF utility library

fn subtract(d1: Float, d2: Float) -> Float {
  -d1.max(d2)
}

fn sd_sphere(p: Float3, pos: Float3, radius: Float) -> Float {
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
  let mut result = 2.0 * (screen / size - 0.5);
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
