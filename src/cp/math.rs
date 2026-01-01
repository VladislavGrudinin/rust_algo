use std::{cmp::min, collections::HashMap, mem::swap};

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

fn pow(mut a: usize, mut b: usize, modulo: usize) -> usize {
  let mut res = 1;
  while b > 0 {
    if b % 2 == 1 {
      res = res * a % modulo;
    }
    a = a * a % modulo;
    b >>= 1;
  }
  res
}

fn inv(a: usize, modulo: usize) -> usize {
  pow(a, modulo - 2, modulo)
}

fn get_factorization(mut a: usize) -> Vec<usize> {
  let mut primes = Vec::new();
  if a % 2 == 0 {
    primes.push(2);
  }
  while a % 2 == 0 {
    a /= 2;
  }
  let mut i = 3;
  while i * i <= a {
    if a % i == 0 {
      primes.push(i);
    }
    while a % i == 0 {
      a /= i;
    }
    i += 2;
  }
  if a > 1 {
    primes.push(a);
  }
  primes
}

fn find_generator(modulo: usize) -> usize {
  let primes = get_factorization(modulo - 1);
  'next_g: for g in 2.. {
    for p in &primes {
      if pow(g, (modulo - 1) / p, modulo) == 1 {
        continue 'next_g;
      }
    }
    return g;
  }
  return usize::MAX;
}

fn get_order(a: usize, primes: &Vec<usize>, modulo: usize) -> usize {
  let mut order = modulo - 1;
  for p in primes {
    while order % p == 0 {
      if pow(a, order / p, modulo) == 1 {
        order /= p;
      } else {
        break;
      }
    }
  }
  order
}

fn get_ind(a: usize, primes: &Vec<usize>, modulo: usize) -> usize {
  (modulo - 1) / get_order(a, primes, modulo)
}

struct DiscreteLog {
  map: HashMap<usize, usize>,
  an: usize,
  n: usize,
  modulo: usize,
}

impl DiscreteLog {
  fn new(a: usize, modulo: usize) -> Self {
    let n = 4_500_000;
    let mut map = HashMap::new();
    let mut cur = 1;
    for q in 0..n {
      map.insert(cur, q);
      cur = cur * a % modulo;
    }
    let an = inv(pow(a, n, modulo), modulo);
    Self { map, an, n, modulo }
  }

  fn get(&mut self, b: usize) -> usize {
    let mut cur = b;
    for p in 0..self.n {
      let v = self.map.get(&cur);
      if v.is_some() {
        return self.n * p + v.unwrap();
      }
      cur = cur * self.an % self.modulo;
    }
    return usize::MAX;
  }
}
