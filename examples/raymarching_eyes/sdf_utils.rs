use shader_roy_metal_sl_interface::*;

// --- SDF utility library

pub fn subtract(d1: Float, d2: Float) -> Float {
  -d1.max(d2)
}

pub fn sd_sphere(p: Float3, pos: Float3, radius: Float) -> Float {
  p.distance(pos) - radius
}

pub fn sd_box(p: Float2, pos: Float2, size: Float2) -> Float {
  let d: Float2 = (p - pos).abs() - size;
  d.x.clamped(d.y, 0.0) + d.max(0.0).magnitude()
}

// polynomial smooth min (k = 0.1);
pub fn smin_cubic(a: Float, b: Float, k: Float) -> Float {
  let h: Float = k - (a - b).abs().max(0.0);
  a.min(b) - h * h * h / (6.0 * k * k)
}

pub fn op_u(d1: Float, d2: Float) -> Float {
  d1.min(d2)
}

pub fn op_blend(d1: Float, d2: Float) -> Float {
  let k: Float = 0.2;
  smin_cubic(d1, d2, k)
}
