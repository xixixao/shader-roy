//! Vector types defaulted to f32 (C++ float).

// Needs to be added because Rust stdlib doesn't include it
pub use half::f16;

pub type Vec2<T = f32> = vek::Vec2<T>;
pub type Vec3<T = f32> = vek::Vec3<T>;
pub type Vec4<T = f32> = vek::Vec4<T>;

pub type Mat2<T = f32> = vek::Mat2<T>;
pub type Mat3<T = f32> = vek::Mat3<T>;
pub type Mat4<T = f32> = vek::Mat4<T>;
// TODO: All matrix types. Needs custom definitions, since vek doesn't have them
// pub type Mat2x3<T = f32>;
// pub type Mat2x4<T = f32>;
// pub type Mat3x2<T = f32>;
// pub type Mat3x4<T = f32>;
// pub type Mat4x2<T = f32>;
// pub type Mat4x3<T = f32>;
