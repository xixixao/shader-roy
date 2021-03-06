#![allow(dead_code)]
#![allow(clippy::float_cmp)]

use sdf_utils::*;
use shader_roy_metal_sl_interface::*;

mod sdf_utils;

pub fn pixel_color(coordinates: Vec2) -> Vec4 {
  let num_samples_per_axis = 3;
  let mut color = 0.0.vec4();
  for y in 0..num_samples_per_axis {
    for x in 0..num_samples_per_axis {
      color +=
        sample_color(coordinates + (x as f32, y as f32).vec2() / (num_samples_per_axis as f32));
    }
  }
  color /= (num_samples_per_axis * num_samples_per_axis) as f32;
  color
}

pub fn sample_color(coordinates: Vec2) -> Vec4 {
  let cam_pos = (0.0, 0.0, -1.0).vec3();
  let cam_target = (0.0, 0.0, 0.0).vec3();

  let uv = screen_to_world(coordinates);
  let ray_dir = get_camera_ray_dir(uv, cam_pos, cam_target);

  let col = render(cam_pos, ray_dir, uv);
  let gamma_corrected = col.pow(0.4545);
  (gamma_corrected, 1.0).vec4()
}

fn scene(pos: Vec3) -> Vec2 {
  let copy = repeat(pos + (0.11, 0.0, 0.0).vec3(), 0.3, (1.0, 1.0, 0.0).vec3());
  let grid = repeat(pos, 0.3, (1.0, 1.0, 0.0).vec3());

  let eye_dir: Vec3;
  if INPUT.is_cursor_inside_window {
    eye_dir = (
      screen_to_world(INPUT.cursor_position) / 2.0 - pos.xy(),
      -0.05,
    )
      .vec3()
      .normalized();
  } else {
    eye_dir = (
      INPUT.elapsed_time_secs.sin(),
      INPUT.elapsed_time_secs.cos() + 0.5,
      -0.5,
    )
      .vec3()
      .normalized();
  }

  join(
    white(sd_sphere(grid, (0.0, 0.0, 0.0).vec3(), 0.05)),
    join(
      black(sd_sphere(grid, eye_dir / 80.0, 0.04)),
      join(
        white(sd_sphere(copy, (0.0, 0.0, 0.0).vec3(), 0.05)),
        black(sd_sphere(copy, eye_dir / 80.0, 0.04)),
      ),
    ),
  )
  // red_plush(sd_sphere(q, vec3(0.0, 0.0, 0.0), 0.05)).min(sd_sphere(
  //   grid,
  //   vec3(0.0, 0.0, 0.0),
  //   0.01,
  // ))
}

fn infinite_repeat(pos: Vec3, period: f32) -> Vec3 {
  (pos.abs() + 0.5 * period).fmod(period) - 0.5 * period
}

fn repeat(pos: Vec3, period: f32, limit: Vec3) -> Vec3 {
  pos - period * (pos / period).round().clamped(-limit, limit)
}

fn sdf(pos: Vec3) -> f32 {
  scene(pos).x
}

fn render(ray_origin: Vec3, ray_dir: Vec3, uv: Vec2) -> Vec3 {
  let Vec2 { x: d, y: material } = cast_ray(ray_origin, ray_dir);
  if material <= 0.0 {
    // Skybox colour
    (0.30, 0.36, 0.60).vec3() - (ray_dir.y * 0.7)
  } else {
    let pos = ray_origin + ray_dir * d;
    let normal = calc_normal(pos);
    let light_dir: Vec3;
    if INPUT.is_cursor_inside_window {
      light_dir = (screen_to_world(INPUT.cursor_position) - uv, -0.5)
        .vec3()
        .normalized();
    } else {
      light_dir = (
        INPUT.elapsed_time_secs.sin(),
        INPUT.elapsed_time_secs.cos() + 0.5,
        -0.5,
      )
        .vec3()
        .normalized();
    }
    // L is vector from surface point to light, N is surface normal. N and L must be normalized!
    let brightness = normal.dot(light_dir).max(0.0);
    let light_color = (1.80, 1.27, 0.99).vec3() * brightness;
    let ambient = (0.03, 0.04, 0.1).vec3();
    let diffuse = surface_color(material) * (light_color + ambient);
    let mut shadow = 0.0;
    let shadow_ray_origin = pos + normal * 0.01;
    let shadow_t = cast_ray(shadow_ray_origin, light_dir).x;
    if shadow_t >= -1.0 {
      shadow = 1.0;
    }
    shadow.mix(diffuse, diffuse * 0.8)
    // let N = calc_normal(pos);
    // N * 0.5.vec3() + 0.5.vec3()
  }
}

fn cast_ray(ray_origin: Vec3, ray_dir: Vec3) -> Vec2 {
  let mut t = 0.0; // Stores current distance along ray
  let z_clipping_distance = 6.0;

  for _ in 0..64 {
    let Vec2 { x: d, y: material } = scene(ray_origin + ray_dir * t);
    if d < (0.0001 * t) {
      return (t, material).vec2();
    }
    t += d;
    if t > z_clipping_distance {
      return sky();
    }
  }
  sky()
}

fn calc_normal(pos: Vec3) -> Vec3 {
  // Center sample
  let c = sdf(pos);
  // Use offset samples to compute gradient / normal
  let eps_zero = (0.001, 0.0).vec2();
  ((
    sdf(pos + eps_zero.xyy()),
    sdf(pos + eps_zero.yxy()),
    sdf(pos + eps_zero.yyx()),
  )
    .vec3()
    - c)
    .normalized()
}

fn get_camera_ray_dir(uv: Vec2, cam_pos: Vec3, cam_target: Vec3) -> Vec3 {
  // Calculate camera's "orthonormal basis", i.e. its transform matrix components
  let cam_forward = (cam_target - cam_pos).normalized();
  let cam_right = ((0.0, 1.0, 0.0).vec3().cross(cam_forward)).normalized();
  let cam_up = (cam_forward.cross(cam_right)).normalized();

  let f_persp = 2.0;
  (uv.x * cam_right + uv.y * cam_up + cam_forward * f_persp).normalized()
}

fn sky() -> Vec2 {
  (-1.0, 0.0).vec2()
}

fn white(d: f32) -> Vec2 {
  (d, 1.0).vec2()
}

fn black(d: f32) -> Vec2 {
  (d, 2.0).vec2()
}

fn surface_color(material: f32) -> Vec3 {
  if material <= 1.0 {
    (0.9, 0.9, 0.9).vec3()
  } else {
    (0.1, 0.1, 0.1).vec3()
  }
}

fn join(a: Vec2, b: Vec2) -> Vec2 {
  if a.x < b.x {
    a
  } else {
    b
  }
}

// // --- Misc functions

// // https://www.shadertoy.com/view/ll2GD3
#[allow(clippy::many_single_char_names)]
fn palette(mut t: f32, a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Vec3 {
  t = t.clamped(0., 1.);
  a + b * (6.28318 * (c * t + d)).cos()
}

fn screen_to_world(screen: Vec2) -> Vec2 {
  let size = INPUT.window_size;
  let mut result = 2.0 * (screen / size - 0.5);
  result.x *= size.x / size.y;
  result.y *= -1.0;
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
