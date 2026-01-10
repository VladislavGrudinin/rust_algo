use std::{
  cmp::Ordering,
  fmt::Debug,
  fmt::Display,
  ops::{Add, AddAssign, Mul, MulAssign},
};

#[derive(Clone, PartialEq, Eq, Default)]
pub struct BigInt {
  d: Vec<i32>,
}

impl Add<&BigInt> for BigInt {
  type Output = Self;

  fn add(mut self, rhs: &Self) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign<&BigInt> for BigInt {
  fn add_assign(&mut self, rhs: &Self) {
    let mut carry = 0;
    let mut p = 0;
    while p < rhs.d.len() || carry > 0 {
      if p >= self.d.len() {
        self.d.push(0);
      }
      self.d[p] += if p < rhs.d.len() { rhs.d[p] } else { 0 };
      self.d[p] += carry;
      carry = 0;
      if self.d[p] >= Self::MOD as i32 {
        carry = 1;
        self.d[p] -= Self::MOD as i32;
      }
      p += 1;
    }
  }
}

impl Mul<&BigInt> for BigInt {
  type Output = BigInt;

  fn mul(mut self, rhs: &BigInt) -> Self::Output {
    self *= rhs;
    self
  }
}

impl MulAssign<&BigInt> for BigInt {
  fn mul_assign(&mut self, rhs: &Self) {
    let mut d = vec![0; self.d.len() + rhs.d.len()];
    let mut carry = 0;
    for i in 0..self.d.len() {
      let mut j = 0;
      while j < rhs.d.len() || carry > 0 {
        let v = d[i + j] as u64
          + self.d[i] as u64 * if j < rhs.d.len() { rhs.d[j] as u64 } else { 0 }
          + carry;
        d[i + j] = (v % Self::MOD) as i32;
        carry = v / Self::MOD;
        j += 1;
      }
    }
    while !d.is_empty() && *d.last().unwrap() == 0 {
      d.pop();
    }
    self.d = d;
  }
}

impl<T: Into<BigInt>> Add<T> for BigInt {
  type Output = BigInt;

  fn add(mut self, rhs: T) -> Self::Output {
    self += rhs;
    self
  }
}

impl<T: Into<BigInt>> AddAssign<T> for BigInt {
  fn add_assign(&mut self, rhs: T) {
    *self += &BigInt::new(rhs);
  }
}

impl<T: Into<BigInt>> Mul<T> for BigInt {
  type Output = BigInt;

  fn mul(mut self, rhs: T) -> Self::Output {
    self *= rhs;
    self
  }
}

impl<T: Into<BigInt>> MulAssign<T> for BigInt {
  fn mul_assign(&mut self, rhs: T) {
    *self *= &BigInt::new(rhs);
  }
}

impl PartialOrd for BigInt {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.d.len() == other.d.len() {
      self.d.iter().rev().partial_cmp(other.d.iter().rev())
    } else {
      self.d.len().partial_cmp(&other.d.len())
    }
  }
}

impl From<&[u8]> for BigInt {
  fn from(s: &[u8]) -> Self {
    std::str::from_utf8(s).unwrap().into()
  }
}

impl<const N: usize> From<&[u8; N]> for BigInt {
  fn from(s: &[u8; N]) -> Self {
    s.as_slice().into()
  }
}

impl From<&str> for BigInt {
  fn from(mut s: &str) -> Self {
    let mut d = Vec::new();
    while !s.is_empty() {
      let l = if s.len() < 9 { 0 } else { s.len() - 9 };
      d.push(s[l..].parse().unwrap());
      s = &s[0..l];
    }
    Self { d }
  }
}

impl From<u64> for BigInt {
  fn from(mut v: u64) -> Self {
    let mut d = Vec::new();
    while v > 0 {
      d.push((v % Self::MOD) as i32);
      v /= Self::MOD;
    }
    Self { d }
  }
}

macro_rules! impl_from_for_ints {
  ($($t:ident)+) => {
    $(impl From<$t> for BigInt {
      fn from(v: $t) -> Self {
        From::from(v as u64)
      }
    })+
  };
}

impl_from_for_ints!(i8 i16 i32 i64 u8 u16 u32 usize);

impl Display for BigInt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.d.is_empty() {
      return write!(f, "0");
    }
    write!(f, "{}", self.d.last().unwrap())?;
    for p in self.d.iter().rev().skip(1) {
      write!(f, "{:09}", p)?;
    }
    Ok(())
  }
}

impl Debug for BigInt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(self, f)
  }
}

impl BigInt {
  const MOD: u64 = 1_000_000_000;

  pub fn new<T: Into<BigInt>>(v: T) -> Self {
    v.into()
  }

  pub fn pow(self, mut b: usize) -> BigInt {
    let mut a = self;
    let mut res = BigInt::new(1);
    while b > 0 {
      if b % 2 == 1 {
        res *= &a;
      }
      a *= &a.clone();
      b >>= 1;
    }
    res
  }
}
