use std::{cmp::min, mem::swap};

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
  if a == 0 {
    return b;
  }
  if b == 0 {
    return a;
  }
  let i = a.trailing_zeros();
  let j = b.trailing_zeros();
  a >>= i;
  b >>= j;
  loop {
    if b > a {
      swap(&mut a, &mut b);
    }
    a -= b;
    if a == 0 {
      return b << min(i, j);
    }
    a >>= a.trailing_zeros();
  }
}

pub fn lcm(a: u64, b: u64) -> u64 {
  a / gcd(a, b) * b
}
