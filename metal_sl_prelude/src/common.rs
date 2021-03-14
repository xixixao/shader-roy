//! Type definitions for `<metal_common>`.

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

pub trait Common: Sized {
  // Called clamped instead of clamp because std has claimed
  // the name.
  fn clamped<U, V, R>(self, minval: U, maxval: V) -> R
  where
    Self: MinMax<U, R>,
    R: MinMax<V, R>,
  {
    self.max(minval).min(maxval)
  }

  fn mix<Tx, Ty, Tr>(self, x: Tx, y: Ty) -> Tr
  where
    Tx: Copy,
    Ty: Op<Tx, Tr>,
    Tr: Op<Self, Tr>,
    Tx: Op<Tr, Tr>,
  {
    x + (y - x) * self
  }

  fn saturate(self) -> Self
  where
    Self: MinMax<f32, Self>,
  {
    self.clamped(0.0, 1.0)
  }

  fn sign<V>(self) -> Self
  where
    Self: Map<V>,
    V: num::Signed,
  {
    self.map(num::signum)
  }

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
    let t = ((self - edge0) / (edge1 - edge0)).clamped(0.0.into(), 1.0.into());
    t * t * (Tr::from(3.0) - Tr::from(2.0) * t)
  }

  fn step<Te, Tr, V>(self, edge: Te) -> Tr
  where
    Self: Op<Te, Tr>,
    Tr: Map<V>,
    V: PartialOrd,
    V: num::Signed,
    V: std::convert::From<f32>,
  {
    (self - edge)
      .sign()
      .map(|value| vek::partial_min(value, V::from(0.0)))
  }
}

impl<T> Common for T {}

#[test]
fn test() {
  // clamp
  1.0.clamped(0.2, 0.3);
  1.0.vec3().clamped(0.2.vec3(), 0.3.vec3());
  1.0.vec3().clamped(0.2, 0.3.vec3());
  // mix
  1.0.mix(0.2, 0.3);
  1.0.vec3().mix(0.2.vec3(), 0.3.vec3());
  // Not supported: 1.0.vec3().mix(0.2, 0.3.vec3());
  // saturate
  1.0.saturate();
  1.0.vec3().saturate();
  // sign
  1.0.sign();
  1.0.vec3().sign();

  // smoothstep
  1.0f32.smoothstep(0.2, 0.3); // Rust defaults to f64 for which this is not implemented
  1.0.vec3().smoothstep(0.2.vec3(), 0.3.vec3());
  1.0.vec3().smoothstep(0.2, 0.3.vec3());

  // step
  1.0f32.step(0.3); // Rust defaults to f64 for which this is not implemented
  1.0.vec3().step(0.3.vec3());
  1.0.vec3().step(0.3);
}
