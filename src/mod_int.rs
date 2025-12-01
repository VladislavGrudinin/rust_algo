use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ModInt<const MOD: i64> {
  v: i64,
}

impl<const MOD: i64> From<i64> for ModInt<MOD> {
  fn from(v: i64) -> Self {
    Self { v }
  }
}

impl<const MOD: i64> Add<ModInt<MOD>> for ModInt<MOD> {
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self {
    self += rhs;
    self
  }
}

impl<const MOD: i64> AddAssign<ModInt<MOD>> for ModInt<MOD> {
  fn add_assign(&mut self, rhs: ModInt<MOD>) {
    self.v += rhs.v;
    if self.v >= MOD {
      self.v -= MOD;
    }
  }
}

impl<const MOD: i64> Sub<ModInt<MOD>> for ModInt<MOD> {
  type Output = Self;

  fn sub(mut self, rhs: Self) -> Self {
    self -= rhs;
    self
  }
}

impl<const MOD: i64> SubAssign<ModInt<MOD>> for ModInt<MOD> {
  fn sub_assign(&mut self, rhs: ModInt<MOD>) {
    self.v -= rhs.v;
    if self.v < 0 {
      self.v += MOD;
    }
  }
}

impl<const MOD: i64> Mul<ModInt<MOD>> for ModInt<MOD> {
  type Output = Self;

  fn mul(mut self, rhs: Self) -> Self {
    self *= rhs;
    self
  }
}

impl<const MOD: i64> MulAssign<ModInt<MOD>> for ModInt<MOD> {
  fn mul_assign(&mut self, rhs: ModInt<MOD>) {
    self.v = self.v * rhs.v % MOD;
  }
}

impl<const MOD: i64> ModInt<MOD> {
  fn inv(self) -> Self {
    self.pow(MOD - 2)
  }

  fn pow(self, mut b: i64) -> Self {
    let mut a = self;
    let mut result = 1.into();
    while b != 0 {
      if b & 1 != 0 {
        result *= a;
      }
      a *= a;
      b >>= 1;
    }
    result
  }
}

type ModInt3 = ModInt<998_244_353>;
type ModInt7 = ModInt<1_000_000_007>;
type ModInt9 = ModInt<1_000_000_009>;

mod tests {
  use super::*;

  #[test]
  fn test_from() {
    let v = 0i64;
    let _: ModInt7 = v.into();
  }

  #[test]
  fn test_add() {
    let a: ModInt7 = 5.into();
    let b: ModInt7 = 1_000_000_002.into();
    let c = a + b;
    let mut d = a;
    d += b;
    let result = 0.into();
    assert_eq!(c, d);
    assert_eq!(c, result);
  }

  #[test]
  fn test_sub() {
    let a: ModInt7 = 5.into();
    let b: ModInt7 = 1_000_000_002.into();
    let c = a + b;
    assert_eq!(c, 0.into());
    assert_eq!(c - b, a);
    assert_eq!(c - a, b);
  }

  #[test]
  fn test_mul() {
    let a: ModInt7 = 2.into();
    let b: ModInt7 = (1_000_000_008 / 2).into();
    assert_eq!(a * b, 1.into());
  }

  #[test]
  fn test_inv() {
    for i in 1..100 {
      let a: ModInt7 = i.into();
      assert_eq!(a * a.inv(), 1.into());
    }
  }
}
