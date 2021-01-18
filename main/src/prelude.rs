#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub type Float = f32;
pub type Int = i32;
pub type Uint = u32;

pub struct Float2 {
  pub x: f32,
  pub y: f32,
}

pub struct Float3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

pub struct Float4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

pub fn float2(x: f32, y: f32) -> Float2 {
  Float2 { x, y }
}
pub fn float3(x: f32, y: f32, z: f32) -> Float3 {
  Float3 { x, y, z }
}
pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
  Float4 { x, y, z, w }
}

pub trait SharedMath {
  fn abs(self) -> Self;
  fn min(self, b: Self) -> Self;
}

macro_rules! implement {
  ($trait:path > $first_type:ty $(, $type:tt)* { $($implementation:item)* }) => {
    impl $trait for $first_type {
      $($implementation)*
    }
    implement! {
      $trait > $($type),* {
        $($implementation)*
      }
    }
  };
  ($trait:path > $_:tt) => {};
}

implement! {
  SharedMath > Uint, Int, Float, Float2, Float4 {
    fn abs(self) -> Self {
      self
    }
    fn min(self, b: Self) -> Self {
      let _ = b;
      self
    }
  }
}

macro_rules! implement_op {
  // For each $type in first argument {}-bounded
  ({}, $_:tt) => {};
  ({$first_type:ty $(, $types:ty)*}, $traits:tt) => {
    implement_op! {
      $first_type,
      $traits
    }

    implement_op!{
      {$($types),*},
      $traits
    }
  };
  // For each $pair in second argument []-bounded
  ($type:ty, []) => {};
  ($type:ty, [$pair:tt $(, $pairs:tt)*$(,)?]) => {
    implement_op!{
      $type,
      $pair
    }

    implement_op!{
      $type,
      [$($pairs),*]
    }
  };
  ($type:ty, ($($tpi:ident)::*, $method:ident)) => {
    // Implement OpTrait for $type using $method
    impl $($tpi)::* for $type {
      type Output = Self;
      fn $method(self, other: Self) -> Self {
        let _ = other;
        self
      }
    }

    implement_op_implicit_conversion! {
      [Uint, Int, Float],
      $($tpi)::* for $type > $method
    }
  };
}

macro_rules! implement_op_implicit_conversion {
  // For each $other_type in first argument []-bounded
  ([], $($_:tt)*) => {};
  (
    [$other_type:ty $(, $other_types:ty)*],
    $($tpi:ident)::* for $type:ty > $method:ident
  ) => {
    implement_op_implicit_conversion! {
      $($tpi)::* for $type, $other_type > $method
    }
    implement_op_implicit_conversion! {
      [$($other_types),*],
      $($tpi)::* for $type > $method
    }
  };
  ($($tpi:ident)::* for $type:ty, $other_type:ty > $method:ident) => {
    // Implement OpTrait<$other_type> for $type using $method
    impl $($tpi)::*<$other_type> for $type {
      type Output = Self;
      fn $method(self, other: $other_type) -> Self {
        let _ = other;
        self
      }
    }
    // Implement OpTrait<$type> for $other_type using $method
    impl $($tpi)::*<$type> for $other_type {
      type Output = $type;
      fn $method(self, other: $type) -> $type {
        let _ = self;
        other
      }
    }
  }
}

implement_op! {
  {Float2, Float3, Float4},
  [
    (std::ops::Add, add),
    (std::ops::Sub, sub),
    (std::ops::Mul, mul),
    (std::ops::Div, div),
  ]
}
