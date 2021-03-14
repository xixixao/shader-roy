//! Vector types defaulted to f32 (C++ float).

// Needs to be added because Rust stdlib doesn't include it
#[allow(non_camel_case_types)]
pub type f16 = half::f16;

pub type Vec2<T = f32> = vek::Vec2<T>;
pub type Vec3<T = f32> = vek::Vec3<T>;
pub type Vec4<T = f32> = vek::Vec4<T>;
