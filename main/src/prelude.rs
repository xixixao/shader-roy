#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub type Float = f32;
pub type Int = i32;
pub type Uint = u32;

pub type Float2 = vek::Vec2<f32>;
pub type Float3 = vek::Vec3<f32>;
pub type Float4 = vek::Vec4<f32>;

pub fn float2(x: f32, y: f32) -> Float2 {
  Float2 { x, y }
}

pub fn float3(x: f32, y: f32, z: f32) -> Float3 {
  Float3 { x, y, z }
}

pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
  Float4 { x, y, z, w }
}

prelude_macros::implement_constructors! {
  Float2 => [1, 2] => {
    (Float),
    (Float, b: Float),
    (Float2),
  },
  Float3 => [1, 2, 3] => {
    (Float),
    (Float, y: Float, z: Float),
    (Float, b: Float2),
    (Float2, b: Float),
    (Float3),
  },
  Float4 => [1, 2, 3, 4] => {
    (Float),
    (Float, y: Float, z: Float, w: Float),
    (Float2, b: Float2),
    (Float2, b: Float, c: Float),
    (Float, b: Float, c: Float2),
    (Float, b: Float2, c: Float),
    (Float3, b: Float),
    (Float, b: Float3),
    (Float4),
  },
}

// pub trait Float4Construct2<B> {
//   fn float4(self, b: B) -> Float4;
// }

// impl Float4Construct2<Float> for Float3 {
//   fn float4(self, b: Float) -> Float4 {
//     float4(self.x, self.y, self.z, b)
//   }
// }

// T clamp(T x, T minval, T maxval) Returns fmin(fmax(x, minval), maxval).
// Results are undefined if minval > maxval.
// T mix(T x, T y, T a) Returns the linear blend of x and y implemented
// as:
// x + (y – x) * a
// a must be a value in the range 0.0 to 1.0. If a is
// not in the range 0.0 to 1.0, the return values are
// undefined.
// T saturate(T x) Clamp the specified value within the range of 0.0
// to 1.0.
// T sign(T x) Returns 1.0 if x > 0, -0.0 if x = -0.0, +0.0 if x
// = +0.0, or -1.0 if x < 0. Returns 0.0 if x is a
// NaN.
// 2020-11-09 Copyright © 2020 Apple Inc. All Rights Reserved.
// Page 129 of 223
// Built-in Common Functions Description
// T smoothstep(T edge0, T edge1,
//  T x)
// Returns 0.0 if x <= edge0 and 1.0 if x >= edge1
// and performs a smooth Hermite interpolation
// between 0 and 1 when edge0 < x < edge1.
// This is useful in cases where you want a
// threshold function with a smooth transition.
// This is equivalent to:
// t = clamp((x – edge0)/(edge1 – edge0),
// 0, 1);
// return t * t * (3 – 2 * t);
// Results are undefined if edge0 >= edge1 or if x,
// edge0, or edge1 is a NaN.
// T step(T edge, T x) Returns 0.0 if x < edge, otherwise it returns 1.0.

pub trait UnstableMath {
  fn clamp(self, min: f32, max: f32) -> f32;
}

impl UnstableMath for f32 {
  fn clamp(self, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    let mut x = self;
    if x < min {
      x = min;
    }
    if x > max {
      x = max;
    }
    x
  }
}
