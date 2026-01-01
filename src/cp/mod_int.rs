use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SModInt<const MOD: i64> {
  v: i64,
}

impl<const MOD: i64> From<i64> for SModInt<MOD> {
  fn from(v: i64) -> Self {
    let v = v % MOD;
    Self { v }
  }
}

impl<const MOD: i64> From<SModInt<MOD>> for i64 {
  fn from(value: SModInt<MOD>) -> Self {
    value.v
  }
}

impl<const MOD: i64> Add<SModInt<MOD>> for SModInt<MOD> {
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self {
    self += rhs;
    self
  }
}

impl<const MOD: i64> AddAssign<SModInt<MOD>> for SModInt<MOD> {
  fn add_assign(&mut self, rhs: SModInt<MOD>) {
    self.v += rhs.v;
    if self.v >= MOD {
      self.v -= MOD;
    }
  }
}

impl<const MOD: i64> Sub<SModInt<MOD>> for SModInt<MOD> {
  type Output = Self;

  fn sub(mut self, rhs: Self) -> Self {
    self -= rhs;
    self
  }
}

impl<const MOD: i64> SubAssign<SModInt<MOD>> for SModInt<MOD> {
  fn sub_assign(&mut self, rhs: SModInt<MOD>) {
    self.v -= rhs.v;
    if self.v < 0 {
      self.v += MOD;
    }
  }
}

impl<const MOD: i64> Mul<SModInt<MOD>> for SModInt<MOD> {
  type Output = Self;

  fn mul(mut self, rhs: Self) -> Self {
    self *= rhs;
    self
  }
}

impl<const MOD: i64> MulAssign<SModInt<MOD>> for SModInt<MOD> {
  fn mul_assign(&mut self, rhs: SModInt<MOD>) {
    self.v = self.v * rhs.v % MOD;
  }
}

impl<const MOD: i64> Div<SModInt<MOD>> for SModInt<MOD> {
  type Output = Self;

  fn div(mut self, rhs: Self) -> Self {
    self /= rhs;
    self
  }
}

impl<const MOD: i64> DivAssign<SModInt<MOD>> for SModInt<MOD> {
  fn div_assign(&mut self, rhs: SModInt<MOD>) {
    *self *= rhs.inv();
  }
}

impl<const MOD: i64> SModInt<MOD> {
  pub fn new<T: TryInto<i64>>(v: T) -> Self {
    Self { v: v.try_into().map_err(|_| ()).unwrap() }
  }

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

pub type ModInt3 = SModInt<998_244_353>;
pub type ModInt7 = SModInt<1_000_000_007>;
pub type ModInt9 = SModInt<1_000_000_009>;
pub type ModInt = ModInt7;
