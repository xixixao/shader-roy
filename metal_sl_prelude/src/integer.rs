//! Type definitions for `<metal_integer>`.

// TODO (PRs welcome):
// T abs(T x)
// Tu absdiff(T x, T y)
// T addsat(T x, T y)
// T clamp(T x, T minval, T maxval)
// T clz(T x)
// T ctz(T x)
// T extract_bits(T x, uint offset,uint bits)
// T hadd(T x, T y)
// T insert_bits(T base, T insert,uint offset, uint bits)
// T32 mad24(T32 x, T32 y, T32 z)
// T madhi(T a, T b, T c)
// T madsat(T a, T b, T c)
// T max(T x, T y)
// T max3(T x, T y, T z)
// Returns max(x, max(y, z)
// T median3(T x, T y, T z)
// T min(T x, T y)
// T min3(T x, T y, T z)
// Returns min(x, min(y, z)
// T32 mul24(T32 x, T32 y)
// T mulhi(T x, T y)
// T popcount(T x)
// T reverse_bits(T x)
// T rhadd(T x, T y)
// T rotate(T v, T i)
// T subsat(T x, T y)

#[test]
fn test_abs() {
  1i32.abs();
  // TODO: 1.vec2i32().abs();
}
