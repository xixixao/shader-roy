use super::*;

pub trait ComponentWiseMath {
  // fn abs(self) -> Self;
  // fn min(self, b: Self) -> Self;
  fn cos(self) -> Self;
}

impl ComponentWiseMath for Float3 {
  fn cos(self) -> Self {
    float3(self.x.cos(), self.y.cos(), self.z.cos())
  }
}

pub trait MinMax<V = Self, R = V>: Sized {
  fn min(self, b: V) -> R;
  fn max(self, b: V) -> R;
}

// TODO: Use a macro to generate all implementations
impl MinMax for Float {
  fn min(self, b: Self) -> Self {
    vek::ops::partial_min(self, b)
  }

  fn max(self, b: Self) -> Self {
    vek::ops::partial_max(self, b)
  }
}

impl MinMax<Float2> for Float {
  fn min(self, b: Float2) -> Float2 {
    Float2::partial_min(self.into(), b)
  }

  fn max(self, b: Float2) -> Float2 {
    Float2::partial_max(self.into(), b)
  }
}

impl MinMax<Float, Float2> for Float2 {
  fn min(self, b: Float) -> Float2 {
    Float2::partial_min(self, b.into())
  }

  fn max(self, b: Float) -> Float2 {
    Float2::partial_max(self, b.into())
  }
}

pub fn min<T, U, R>(a: T, b: U) -> R
where
  T: MinMax<U, R>,
{
  a.min(b)
}

pub fn max<T, U, R>(a: T, b: U) -> R
where
  T: MinMax<U, R>,
{
  a.max(b)
}
