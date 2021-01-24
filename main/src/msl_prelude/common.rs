use super::*;

use std::ops::*;

pub trait Op<T = Self, R = T>:
  Sized + Add<T, Output = R> + Sub<T, Output = R> + Div<T, Output = R> + Mul<T, Output = R>
{
}

impl<S, T, R> Op<T, R> for S where
  S: Sized + Add<T, Output = R> + Sub<T, Output = R> + Div<T, Output = R> + Mul<T, Output = R>
{
}

// TODO:
// T saturate(T x) Clamp the specified value within the range of 0.0
// to 1.0.

// TODO:
// T sign(T x) Returns 1.0 if x > 0, -0.0 if x = -0.0, +0.0 if x
// = +0.0, or -1.0 if x < 0. Returns 0.0 if x is a
// NaN.

// TODO:
// T step(T edge, T x) Returns 0.0 if x < edge, otherwise it returns 1.0.

pub fn clamp<T, U, V, R>(x: T, minval: U, maxval: V) -> R
where
  T: MinMax<U, R>,
  R: MinMax<V, R>,
{
  min(max(x, minval), maxval)
}

pub trait Clamp {
  // Called clamped instead of clamp because std has claimed
  // the name.
  fn clamped<U, V, R>(self, minval: U, maxval: V) -> R
  where
    Self: MinMax<U, R>,
    R: MinMax<V, R>,
  {
    clamp(self, minval, maxval)
  }
}

impl<T> Clamp for T {}

pub fn mix<Tx, Ty, Ta, Tr>(x: Tx, y: Ty, a: Ta) -> Tr
where
  Tx: Copy,
  Ty: Op<Tx, Tr>,
  Tr: Op<Ta, Tr>,
  Tx: Op<Tr, Tr>,
{
  x + (y - x) * a
}

pub trait Mix: Sized {
  fn mix<Tx, Ty, Tr>(self, x: Tx, y: Ty) -> Tr
  where
    Tx: Copy,
    Ty: Op<Tx, Tr>,
    Tr: Op<Self, Tr>,
    Tx: Op<Tr, Tr>,
  {
    mix(x, y, self)
  }
}

impl Mix for Float {}

pub fn smoothstep<Tx, Ty, Ta, Tr>(edge0: Tx, edge1: Ty, x: Ta) -> Tr
where
  Tx: Copy,
  Tr: Copy,
  Ty: Op<Tx, Tr>,
  Ta: Op<Tx, Tr>,
  Tr: Op,
  Tr: MinMax,
  Tr: std::convert::From<f32>,
{
  let t = clamp((x - edge0) / (edge1 - edge0), 0.0.into(), 1.0.into());
  t * t * (Tr::from(3.0) - Tr::from(2.0) * t)
}

pub trait SmoothStep {
  fn smoothstep<Tx, Ty, Tr>(self, edge0: Tx, edge1: Ty) -> Tr
  where
    Tx: Copy,
    Tr: Copy,
    Ty: Op<Tx, Tr>,
    Self: Op<Tx, Tr>,
    Tr: Op,
    Tr: MinMax,
    Tr: std::convert::From<f32>,
  {
    smoothstep(edge0, edge1, self)
  }
}

impl<T> SmoothStep for T {}

// macro_rules! implement {
//   ($trait:path > $first_type:ty $(, $type:tt)* { $($implementation:item)* }) => {
//     impl $trait for $first_type {
//       $($implementation)*
//     }
//     implement! {
//       $trait > $($type),* {
//         $($implementation)*
//       }
//     }
//   };
//   ($trait:path > $_:tt) => {};
// }

// implement! {
//   SharedMath > Uint, Int, Float, Float2, Float4 {
//     fn abs(self) -> Self {
//       self
//     }
//     fn min(self, b: Self) -> Self {
//       let _ = b;
//       self
//     }
//   }
// }
