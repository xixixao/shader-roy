use super::*;

pub trait Geometric {
  fn length(self) -> Float;
}

prelude_macros::implement! {
  Geometric > Float2, Float3, Float4 {
    fn length(self) -> Float {
      self.magnitude()
    }
  }
}

pub fn length<T: Geometric>(x: T) -> Float {
  x.length()
}
