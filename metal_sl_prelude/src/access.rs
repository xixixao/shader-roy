use super::*;

prelude_macros::implement_accessors! {
  Vec2,
  Vec3,
  Vec4,
  // Uint2,
  // Uint3,
  // Uint4,
}

#[test]
fn test() {
  let _ = 1.0.vec3().x;
  1.0.vec3().xy();
  1.0.vec3().zyx();
}
