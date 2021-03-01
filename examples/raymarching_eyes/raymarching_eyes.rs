#![allow(dead_code)]
#![allow(clippy::float_cmp)]

use sdf_utils::*;
use shader_roy_metal_sl_interface::*;

mod sdf_utils;

pub fn pixel_color(coordinates: Float2) -> Float4 {
  let num_samples_per_axis = 3;
  let mut color = 0.0.float4();
  for y in 0..num_samples_per_axis {
    for x in 0..num_samples_per_axis {
      color += sample_color(
        coordinates + float2(x as Float, y as Float) / (num_samples_per_axis as Float),
      );
    }
  }
  color /= (num_samples_per_axis * num_samples_per_axis) as Float;
  color
}

pub fn sample_color(coordinates: Float2) -> Float4 {
  let cam_pos = float3(0.0, 0.0, -1.0);
  let cam_target = float3(0.0, 0.0, 0.0);

  let uv = screen_to_world(coordinates);
  let ray_dir = get_camera_ray_dir(uv, cam_pos, cam_target);

  let col = render(cam_pos, ray_dir, uv);
  let gamma_corrected = col.pow(0.4545);
  (gamma_corrected, 1.0).float4()
}

fn scene(pos: Float3) -> Float2 {
  let grid = repeat(pos + 0.05, 0.2, float3(2.0, 2.0, 2.0));
  red_plush(sd_sphere(grid, float3(0.0, 0.0, 0.0), 0.01))

  // red_plush(sd_sphere(q, float3(0.0, 0.0, 0.0), 0.05)).min(sd_sphere(
  //   grid,
  //   float3(0.0, 0.0, 0.0),
  //   0.01,
  // ))
}

fn infinite_repeat(pos: Float3, period: Float) -> Float3 {
  (pos.abs() + 0.5 * period).fmod(period) - 0.5 * period
}

fn repeat(pos: Float3, period: Float, limit: Float3) -> Float3 {
  pos - period * (pos / period).round().clamped(-limit, limit)
}

fn sdf(pos: Float3) -> Float {
  scene(pos).x
}

fn render(ray_origin: Float3, ray_dir: Float3, uv: Float2) -> Float3 {
  let Float2 { x: d, y: material } = cast_ray(ray_origin, ray_dir);
  if material <= 0.0 {
    // Skybox colour
    float3(0.30, 0.36, 0.60) - (ray_dir.y * 0.7)
  } else {
    let pos = ray_origin + ray_dir * d;
    let normal = calc_normal(pos);
    let light_dir: Float3;
    if INPUT.is_cursor_inside_window == 1.0 {
      light_dir = (screen_to_world(INPUT.cursor_position) - uv, -0.5)
        .float3()
        .normalized();
    } else {
      light_dir = (
        INPUT.elapsed_time_secs.sin(),
        INPUT.elapsed_time_secs.cos() + 0.5,
        -0.5,
      )
        .float3()
        .normalized();
    }
    let surface_color = float3(0.4, 0.8, 0.1);
    // L is vector from surface point to light, N is surface normal. N and L must be normalized!
    let brightness = normal.dot(light_dir).max(0.0);
    let light_color = float3(1.80, 1.27, 0.99) * brightness;
    let ambient = float3(0.03, 0.04, 0.1);
    let diffuse = surface_color * (light_color + ambient);
    let mut shadow = 0.0;
    let shadow_ray_origin = pos + normal * 0.01;
    let shadow_t = cast_ray(shadow_ray_origin, light_dir).x;
    if shadow_t >= -1.0 {
      shadow = 1.0;
    }
    shadow.mix(diffuse, diffuse * 0.8)
    // let N = calc_normal(pos);
    // N * 0.5.float3() + 0.5.float3()
  }
}

fn cast_ray(ray_origin: Float3, ray_dir: Float3) -> Float2 {
  let mut t = 0.0; // Stores current distance along ray
  let z_clipping_distance = 6.0;

  for _ in 0..64 {
    let Float2 { x: d, y: material } = scene(ray_origin + ray_dir * t);
    if d < (0.0001 * t) {
      return float2(t, material);
    }
    t += d;
    if t > z_clipping_distance {
      return sky();
    }
  }
  sky()
}

fn calc_normal(pos: Float3) -> Float3 {
  // Center sample
  let c = sdf(pos);
  // Use offset samples to compute gradient / normal
  let eps_zero = float2(0.001, 0.0);
  (float3(
    sdf(pos + eps_zero.xyy()),
    sdf(pos + eps_zero.yxy()),
    sdf(pos + eps_zero.yyx()),
  ) - c)
    .normalized()
}

fn get_camera_ray_dir(uv: Float2, cam_pos: Float3, cam_target: Float3) -> Float3 {
  // Calculate camera's "orthonormal basis", i.e. its transform matrix components
  let cam_forward = (cam_target - cam_pos).normalized();
  let cam_right = (float3(0.0, 1.0, 0.0).cross(cam_forward)).normalized();
  let cam_up = (cam_forward.cross(cam_right)).normalized();

  let f_persp = 2.0;
  (uv.x * cam_right + uv.y * cam_up + cam_forward * f_persp).normalized()
}

fn sky() -> Float2 {
  float2(-1.0, 0.0)
}

fn red_plush(d: Float) -> Float2 {
  float2(d, 1.0)
}

// // --- Misc functions

// // https://www.shadertoy.com/view/ll2GD3
#[allow(clippy::many_single_char_names)]
fn palette(mut t: Float, a: Float3, b: Float3, c: Float3, d: Float3) -> Float3 {
  t = t.clamped(0., 1.);
  a + b * (6.28318 * (c * t + d)).cos()
}

fn screen_to_world(screen: Float2) -> Float2 {
  let size = INPUT.window_size;
  let mut result = 2.0 * (screen / size - 0.5);
  result.x *= size.x / size.y;
  result.y *= -1.0;
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
