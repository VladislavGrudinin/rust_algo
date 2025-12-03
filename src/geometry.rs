use std::ops::{Add, Sub, Neg, Mul};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
struct Vec2<T> {
  x: T,
  y: T,
}

impl<T: Neg<Output = T>> Vec2<T> {
  fn ortho(self) -> Self {
    Vec2 {
      x: self.y,
      y: -self.x,
    }
  }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }
}

impl<T: Mul<Output = T> + Add<Output = T>> Mul for Vec2<T> {
  type Output = T;
  fn mul(self, rhs: Self) -> T {
    dot(self, rhs)
  }
}

fn dot<T: Mul<Output = T> + Add<Output = T>>(a: Vec2<T>, b: Vec2<T>) -> T {
  a.x * b.x + a.y * b.y
}

fn cross<T: Mul<Output = T> + Sub<Output = T>>(a: Vec2<T>, b: Vec2<T>) -> T {
  a.x * b.y - a.y * b.x
}
