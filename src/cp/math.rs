use std::{cmp::min, mem::swap};

pub fn gcd(mut a: usize, mut b: usize) -> usize {
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

pub fn lcm(a: usize, b: usize) -> usize {
  a / gcd(a, b) * b
}

pub fn get_primes(max_prime: usize) -> (Vec<usize>, Vec<usize>) {
  let mut lt = vec![0; max_prime];
  let mut primes = Vec::new();
  for i in 2..max_prime {
    if lt[i] == 0 {
      lt[i] = i;
      primes.push(i);
    }
    for &p in &primes {
      if i * p >= max_prime || lt[i] < p {
        break;
      }
      lt[i * p] = p;
    }
  }
  (primes, lt)
}
