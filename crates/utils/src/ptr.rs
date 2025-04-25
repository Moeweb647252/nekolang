use std::ops::{Deref, DerefMut};

pub struct P<T>(Box<T>);

impl<T: Sized> P<T> {
  pub fn new(value: T) -> Self {
    Self(Box::new(value))
  }
}

impl<T> Deref for P<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for P<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> From<T> for P<T> {
  fn from(value: T) -> Self {
    Self::new(value)
  }
}

impl<T> AsMut<T> for P<T> {
  fn as_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl<T> AsRef<T> for P<T> {
  fn as_ref(&self) -> &T {
    &self.0
  }
}
