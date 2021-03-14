//! Type definitions for `<metal_math>`.

use super::*;

// TODO(PRs welcome):
// T acos(T x)
// T acosh(T x)
// T asin(T x)
// T asinh(T x)
// T atan(T y_over_x)
// T atan2(T y, T x)
// T atanh(T x)
// T ceil(T x)
// T copysign(T x, T y)
// T cosh(T x)
// T cospi(T x)
// T divide(T x, T y)
// T exp(T x)
// T exp2(T x)
// T exp10(T x)
// T fdim(T x, T y)
// T floor(T x)
// T fma(T a, T b, T c)
// T fmax(T x, T y)
// T max(T x, T y)
// T fmax3(T x, T y, T z)
// T max3(T x, T y, T z)
// T fmedian3(T x, T y, T z)
// T median3(T x, T y, T z)
// T fmin(T x, T y)
// T min(T x, T y)
// T fmin3(T x, T y, T z)
// T min3(T x, T y, T z)
// T fmod(T x, T y)
// T fract(T x)
// T frexp(T x, Ti &exponent)
// Ti ilogb(T x)
// T ldexp(T x, Ti k)
// T log(T x)
// T log2(T x)
// T log10(T x)
// T modf(T x, T &intval)
// T powr(T x, T y)
// T rint(T x)
// T round(T x)
// T rsqrt(T x)
// T sin(T x)
// T sincos(T x, T &cosval)
// T sinh(T x)
// T sinpi(T x)
// T sqrt(T x)
// T tan(T x)
// T tanh(T x)
// T tanpi(T x)
// T trunc(T x)

pub const MAXFLOAT: f32 = 0.0;
pub const HUGE_VALF: f32 = 0.0;
pub const INFINITY: f32 = 0.0;
pub const NAN: f32 = 0.0;
pub const M_E_F: f32 = 0.0;
pub const M_LOG2E_F: f32 = 0.0;
pub const M_LOG10E_F: f32 = 0.0;
pub const M_LN2_F: f32 = 0.0;
pub const M_LN10_F: f32 = 0.0;
pub const M_PI_F: f32 = 0.0;
pub const M_PI_2_F: f32 = 0.0;
pub const M_PI_4_F: f32 = 0.0;
pub const M_1_PI_F: f32 = 0.0;
pub const M_2_PI_F: f32 = 0.0;
pub const M_2_SQRTPI_F: f32 = 0.0;
pub const M_SQRT2_F: f32 = 0.0;
pub const M_SQRT1_2_F: f32 = 0.0;

pub const MAXHALF: f16 = f16::from_bits(0);
pub const HUGE_VALH: f16 = f16::from_bits(0);
pub const M_E_H: f16 = f16::from_bits(0);
pub const M_LOG2E_H: f16 = f16::from_bits(0);
pub const M_LOG10E_H: f16 = f16::from_bits(0);
pub const M_LN2_H: f16 = f16::from_bits(0);
pub const M_LN10_H: f16 = f16::from_bits(0);
pub const M_PI_H: f16 = f16::from_bits(0);
pub const M_PI_2_H: f16 = f16::from_bits(0);
pub const M_PI_4_H: f16 = f16::from_bits(0);
pub const M_1_PI_H: f16 = f16::from_bits(0);
pub const M_2_PI_H: f16 = f16::from_bits(0);
pub const M_2_SQRTPI_H: f16 = f16::from_bits(0);
pub const M_SQRT2_H: f16 = f16::from_bits(0);
pub const M_SQRT1_2_H: f16 = f16::from_bits(0);

pub trait ComponentWiseMath {
  fn abs(self) -> Self;
  fn cos(self) -> Self;
}

impl<T: Vector<f32>> ComponentWiseMath for T {
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
  (TSelf, TOther): Map2<f32, TResult>,
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
  (T, V): Map2<f32, R>,
{
  fn pow(self, b: V) -> R {
    (self, b).map2(|a, b| a.powf(b))
  }
  fn fmod(self, b: V) -> R {
    (self, b).map2(|a, b| a % b)
  }
}

#[test]
fn test_abs() {
  let _ = 1.0f32.abs();
  1.0.vec2().abs();
}

#[test]
fn test_cos() {
  let _ = 1.0f32.cos();
  1.0.vec2().cos();
}
