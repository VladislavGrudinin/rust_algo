use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModInt<const MOD: i64> {
  v: i64,
}

impl<const MOD: i64> From<i64> for ModInt<MOD> {
  fn from(v: i64) -> Self {
    let v = v % MOD;
    Self { v }
  }
}

impl<const MOD: i64> From<ModInt<MOD>> for i64 {
  fn from(value: ModInt<MOD>) -> Self {
    value.v
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
  pub fn inv(self) -> Self {
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

pub type ModInt3 = ModInt<998_244_353>;
pub type ModInt7 = ModInt<1_000_000_007>;
pub type ModInt9 = ModInt<1_000_000_009>;
