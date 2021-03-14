use super::*;

pub trait ComponentWiseMath {
  fn abs(self) -> Self;
  fn cos(self) -> Self;
}

impl<T: Vector<Float>> ComponentWiseMath for T {
  fn abs(self) -> Self {
    self.map(|x| x.abs())
  }

  fn cos(self) -> Self {
    self.map(|x| x.cos())
  }
}

pub trait MinMax<TOther = Self, TResult = TOther>: Sized {
  fn min(self, b: TOther) -> TResult;
  fn max(self, b: TOther) -> TResult;
}

impl<TSelf, TOther, TResult> MinMax<TOther, TResult> for TSelf
where
  (TSelf, TOther): Map2<Float, TResult>,
{
  fn min(self, b: TOther) -> TResult {
    (self, b).map2(vek::ops::partial_min)
  }
  fn max(self, b: TOther) -> TResult {
    (self, b).map2(vek::ops::partial_max)
  }
}

pub trait Math<V = Self, R = V>: Sized {
  fn pow(self, b: V) -> R;
  fn fmod(self, b: V) -> R;
}

impl<T, V, R> Math<V, R> for T
where
  (T, V): Map2<Float, R>,
{
  fn pow(self, b: V) -> R {
    (self, b).map2(|a, b| a.powf(b))
  }
  fn fmod(self, b: V) -> R {
    (self, b).map2(|a, b| a % b)
  }
}
