use super::*;

pub trait ScalarOrVector<T> {
  fn map<F>(self, f: F) -> Self
  where
    F: FnMut(T) -> T;
}

prelude_macros::implement! {
  ScalarOrVector<Float> > Float2, Float3, Float4 {
    fn map<F>(self, f: F) -> Self
    where
      F: FnMut(Float) -> Float
    {
      self.map(f)
    }
  }
}

impl ScalarOrVector<Float> for Float {
  fn map<F>(self, mut f: F) -> Self
  where
    F: FnMut(Float) -> Float,
  {
    f(self)
  }
}
