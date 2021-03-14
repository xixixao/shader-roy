use shader_roy_metal_sl_interface::*;

// --- SDF utility library

pub fn subtract(d1: f32, d2: f32) -> f32 {
  -d1.max(d2)
}

pub fn sd_sphere(p: Vec3, center: Vec3, radius: f32) -> f32 {
  p.distance(center) - radius
}

pub fn sd_box(p: Vec2, pos: Vec2, size: Vec2) -> f32 {
  let d: Vec2 = (p - pos).abs() - size;
  d.x.clamped(d.y, 0.0) + d.max(0.0).magnitude()
}

// polynomial smooth min (k = 0.1);
pub fn smin_cubic(a: f32, b: f32, k: f32) -> f32 {
  let h: f32 = k - (a - b).abs().max(0.0);
  a.min(b) - h * h * h / (6.0 * k * k)
}

pub fn op_u(d1: f32, d2: f32) -> f32 {
  d1.min(d2)
}

pub fn op_blend(d1: f32, d2: f32) -> f32 {
  let k: f32 = 0.2;
  smin_cubic(d1, d2, k)
}
