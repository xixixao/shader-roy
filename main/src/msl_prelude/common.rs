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

pub fn clamp<T, U, V, R>(x: T, minval: U, maxval: V) -> R
where
  T: MinMax<U, R>,
  R: MinMax<V, R>,
{
  min(max(x, minval), maxval)
}

pub fn mix<Tx, Ty, Ta, Tr>(x: Tx, y: Ty, a: Ta) -> Tr
where
  Tx: Copy,
  Ty: Op<Tx, Tr>,
  Tr: Op<Ta, Tr>,
  Tx: Op<Tr, Tr>,
{
  x + (y - x) * a
}

pub fn saturate<T>(x: T) -> T
where
  T: MinMax<Float, T>,
{
  min(max(x, 1.0), 1.0)
}

pub fn sign<T, V>(x: T) -> T
where
  T: ScalarOrVector<V>,
  V: num::Signed,
{
  x.map(num::signum)
}

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

pub fn step<Tx, Te, Tr, V>(edge: Te, x: Tx) -> Tr
where
  Tx: Op<Te, Tr>,
  Tr: ScalarOrVector<V>,
  V: PartialOrd,
  V: num::Signed,
  V: std::convert::From<f32>,
{
  sign(x - edge).map(|value| vek::partial_min(value, V::from(0.0)))
}

pub trait Common: Sized {
  // Called clamped instead of clamp because std has claimed
  // the name.
  fn clamped<U, V, R>(self, minval: U, maxval: V) -> R
  where
    Self: MinMax<U, R>,
    R: MinMax<V, R>,
  {
    clamp(self, minval, maxval)
  }

  fn mix<Tx, Ty, Tr>(self, x: Tx, y: Ty) -> Tr
  where
    Tx: Copy,
    Ty: Op<Tx, Tr>,
    Tr: Op<Self, Tr>,
    Tx: Op<Tr, Tr>,
  {
    mix(x, y, self)
  }

  fn saturate(self) -> Self
  where
    Self: MinMax<Float, Self>,
  {
    min(max(self, 1.0), 1.0)
  }

  fn sign<V>(self) -> Self
  where
    Self: ScalarOrVector<V>,
    V: num::Signed,
  {
    sign(self)
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
    smoothstep(edge0, edge1, self)
  }

  fn step<Te, Tr, V>(self, edge: Te) -> Tr
  where
    Self: Op<Te, Tr>,
    Tr: ScalarOrVector<V>,
    V: PartialOrd,
    V: num::Signed,
    V: std::convert::From<f32>,
  {
    step(edge, self)
  }
}

impl<T> Common for T {}
