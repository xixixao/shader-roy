//! Internal module for implementing MSL utils
//!
//! It provides the Map<T> and Map2<T> traits implemented for floating point scalars and vectors.
//! When we want to implement only for vectors we use Vector<T>.

use super::*;

pub trait Vector<T> {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(T) -> T;

  fn map2<F>(self, b: Self, f: F) -> Self
  where
    F: FnMut(T, T) -> T;
}

prelude_macros::implement! {
  Vector<Float> > Float2, Float3, Float4 {
    fn map<F>(self, f: F) -> Self
    where
      F: FnMut(Float) -> Float
    {
      self.map(f)
    }

    fn map2<F>(self, b: Self, f: F) -> Self
    where
      F: FnMut(Float, Float) -> Float {
        self.map2(b, f)
      }
  }
}

pub trait Map<T> {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(T) -> T;
}

impl Map<Float> for Float {
  fn map<F>(self, mut f: F) -> Self
  where
    F: FnMut(Float) -> Float,
  {
    f(self)
  }
}

impl<V: Vector<Float>> Map<Float> for V {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(Float) -> Float,
  {
    self.map(f)
  }
}

pub trait Map2<T, R>: Sized {
  fn map2<F>(self, f: F) -> R
  where
    F: FnMut(T, T) -> T;
}

impl Map2<Float, Float> for (Float, Float) {
  fn map2<F>(self, mut f: F) -> Float
  where
    F: FnMut(Float, Float) -> Float,
  {
    f(self.0, self.1)
  }
}

impl<T: Vector<Float>> Map2<Float, T> for (T, T) {
  fn map2<F>(self, f: F) -> T
  where
    F: FnMut(Float, Float) -> Float,
  {
    self.0.map2(self.1, f)
  }
}

impl<T: Vector<Float>> Map2<Float, T> for (T, Float) {
  fn map2<F>(self, mut f: F) -> T
  where
    F: FnMut(Float, Float) -> Float,
  {
    let (a, b) = self;
    a.map(|x| f(x, b))
  }
}

impl<T: Vector<Float>> Map2<Float, T> for (Float, T) {
  fn map2<F>(self, mut f: F) -> T
  where
    F: FnMut(Float, Float) -> Float,
  {
    let (a, b) = self;
    b.map(|x| f(a, x))
  }
}
