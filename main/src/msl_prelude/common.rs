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
    Self: MinMax<Float, Self>,
  {
    self.clamped(0.0, 1.0)
  }

  fn sign<V>(self) -> Self
  where
    Self: ScalarOrVector<V>,
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
    Tr: ScalarOrVector<V>,
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
