use super::*;

prelude_macros::implement_constructors! {
  [i8, u8, i16, u16 , i32, u32, i64, u64, f16, f32], // no f16
  {
    Vec2 => {
      (all: Num),
      (x: Num, y: Num),
      (fr: Vec2),
    },
    Vec3 => {
      (all: Num),
      (x: Num, y: Num, z: Num),
      (x: Num, b: Vec2),
      (a: Vec2, z: Num),
      (fr: Vec3),
    },
    Vec4 => {
      (all: Num),
      (x: Num, y: Num, z: Num, w: Num),
      (a: Vec2, b: Vec2),
      (a: Vec2, z: Num, w: Num),
      (x: Num, y: Num, c: Vec2),
      (x: Num, b: Vec2, c: Num),
      (a: Vec3, w: Num),
      (x: Num, b: Vec3),
      (fr: Vec4),
    },
  }
}

#[test]
fn test() {
  1.0.vec3();
  (1.0, 2.0, 3.0).vec3();
  (1.0.vec2(), 3.0).vec3();

  1.vec3i32();
  (1, 2, 3).vec3i32();
  (1.vec2i32(), 3).vec3i32();
}
