use super::*;

use std::ops::*;

pub trait Geometric {
  fn length(self) -> Float;
  fn length_squared(self) -> Float;
  fn dot(self, y: Self) -> Float;
}

prelude_macros::implement! {
  Geometric > Float2, Float3, Float4 {
    fn length(self) -> Float {
      self.magnitude()
    }

    fn length_squared(self) -> Float {
      self.magnitude_squared()
    }

    fn dot(self, y: Self) -> Float {
      self.dot(y)
    }
  }
}

pub fn length<T: Geometric>(x: T) -> Float {
  x.length()
}

pub fn length_squared<T: Geometric>(x: T) -> Float {
  x.length_squared()
}

pub fn cross(x: Float3, y: Float3) -> Float3 {
  x.cross(y)
}

pub fn distance<Tx, Ty, Tr>(x: Tx, y: Ty) -> Float
where
  Tx: Sub<Ty, Output = Tr>,
  Tr: Geometric,
{
  length(x - y)
}

pub fn distance_squared<Tx, Ty, Tr>(x: Tx, y: Ty) -> Float
where
  Tx: Sub<Ty, Output = Tr>,
  Tr: Geometric,
{
  length_squared(x - y)
}

fn dot<T>(x: T, y: T) -> Float
where
  T: Geometric,
{
  x.dot(y)
}
