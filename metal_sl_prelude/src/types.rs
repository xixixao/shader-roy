//! Type aliases for Metal types.
//!
//! The primitive types are named so that the vector types are their natural extension. So we don't
//! provide the other names Metal Shading Language exposes.
//! They are capitalized to get correct syntax highlighting in IDEs. This means that unlike
//! in normal Rust, primitive types are capitalized. The only exception is `bool`, which is
//! the same between Rust and Metal SL.

pub type Char = i8;
pub type UChar = u8;
pub type Short = i16;
pub type UShort = u16;
pub type Int = i32;
pub type UInt = u32;
pub type Long = i32;
pub type ULong = i32;
// there's no f16 in Rust stdlib and it doesn't matter to us really
pub type Half = f32;
pub type Float = f32;

pub type Bool2 = vek::Vec2<bool>;
pub type Bool3 = vek::Vec3<bool>;
pub type Bool4 = vek::Vec4<bool>;

pub type Char2 = vek::Vec2<Char>;
pub type Char3 = vek::Vec3<Char>;
pub type Char4 = vek::Vec4<Char>;

pub type UChar2 = vek::Vec2<UChar>;
pub type UChar3 = vek::Vec3<UChar>;
pub type UChar4 = vek::Vec4<UChar>;

pub type Short2 = vek::Vec2<Short>;
pub type Short3 = vek::Vec3<Short>;
pub type Short4 = vek::Vec4<Short>;

pub type UShort2 = vek::Vec2<UShort>;
pub type UShort3 = vek::Vec3<UShort>;
pub type UShort4 = vek::Vec4<UShort>;

pub type Int2 = vek::Vec2<Int>;
pub type Int3 = vek::Vec3<Int>;
pub type Int4 = vek::Vec4<Int>;

pub type UInt2 = vek::Vec2<UInt>;
pub type UInt3 = vek::Vec3<UInt>;
pub type UInt4 = vek::Vec4<UInt>;

pub type Long2 = vek::Vec2<Long>;
pub type Long3 = vek::Vec3<Long>;
pub type Long4 = vek::Vec4<Long>;

pub type ULong2 = vek::Vec2<ULong>;
pub type ULong3 = vek::Vec3<ULong>;
pub type ULong4 = vek::Vec4<ULong>;

pub type Half2 = vek::Vec2<Half>;
pub type Half3 = vek::Vec3<Half>;
pub type Half4 = vek::Vec4<Half>;

pub type Float2 = vek::Vec2<Float>;
pub type Float3 = vek::Vec3<Float>;
pub type Float4 = vek::Vec4<Float>;
