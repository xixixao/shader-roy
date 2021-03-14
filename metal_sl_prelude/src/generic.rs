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

impl<T> Vector<T> for Vec2<T> {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(T) -> T,
  {
    self.map(f)
  }

  fn map2<F>(self, b: Self, f: F) -> Self
  where
    F: FnMut(T, T) -> T,
  {
    self.map2(b, f)
  }
}

impl<T> Vector<T> for Vec3<T> {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(T) -> T,
  {
    self.map(f)
  }

  fn map2<F>(self, b: Self, f: F) -> Self
  where
    F: FnMut(T, T) -> T,
  {
    self.map2(b, f)
  }
}

impl<T> Vector<T> for Vec4<T> {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(T) -> T,
  {
    self.map(f)
  }

  fn map2<F>(self, b: Self, f: F) -> Self
  where
    F: FnMut(T, T) -> T,
  {
    self.map2(b, f)
  }
}

pub trait Map<T> {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(T) -> T;
}

impl Map<f32> for f32 {
  fn map<F>(self, mut f: F) -> Self
  where
    F: FnMut(f32) -> f32,
  {
    f(self)
  }
}

impl<V: Vector<f32>> Map<f32> for V {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(f32) -> f32,
  {
    self.map(f)
  }
}

pub trait Map2<T, R>: Sized {
  fn map2<F>(self, f: F) -> R
  where
    F: FnMut(T, T) -> T;
}

impl Map2<f32, f32> for (f32, f32) {
  fn map2<F>(self, mut f: F) -> f32
  where
    F: FnMut(f32, f32) -> f32,
  {
    f(self.0, self.1)
  }
}

impl<T: Vector<f32>> Map2<f32, T> for (T, T) {
  fn map2<F>(self, f: F) -> T
  where
    F: FnMut(f32, f32) -> f32,
  {
    self.0.map2(self.1, f)
  }
}

impl<T: Vector<f32>> Map2<f32, T> for (T, f32) {
  fn map2<F>(self, mut f: F) -> T
  where
    F: FnMut(f32, f32) -> f32,
  {
    let (a, b) = self;
    a.map(|x| f(x, b))
  }
}

impl<T: Vector<f32>> Map2<f32, T> for (f32, T) {
  fn map2<F>(self, mut f: F) -> T
  where
    F: FnMut(f32, f32) -> f32,
  {
    let (a, b) = self;
    b.map(|x| f(a, x))
  }
}
