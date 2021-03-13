use super::*;

prelude_macros::implement_constructors! {
  [Bool, Char, UChar, Short, UShort, Int, UInt, Long, ULong, Half, Float],
  {
    Type2 => {
      (all: Type),
      (x: Type, y: Type),
      (fr: Type2),
    },
    Type3 => {
      (all: Type),
      (x: Type, y: Type, z: Type),
      (x: Type, b: Type2),
      (a: Type2, z: Type),
      (fr: Type3),
    },
    Type4 => {
      (all: Type),
      (x: Type, y: Type, z: Type, w: Type),
      (a: Type2, b: Type2),
      (a: Type2, z: Type, w: Type),
      (x: Type, y: Type, c: Type2),
      (x: Type, b: Type2, c: Type),
      (a: Type3, w: Type),
      (x: Type, b: Type3),
      (fr: Type4),
    },
  }
}
