use super::*;

pub trait Geometric {
  fn length(self) -> Float;
}

impl Geometric for Float2 {
  fn length(self) -> Float {
    self.magnitude()
  }
}

pub fn length<T: Geometric>(x: T) -> Float {
  x.length()
}
