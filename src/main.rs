#![allow(dead_code, unused_imports)]
#![allow(non_snake_case)]
use std::cmp::{self, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};
use std::io::{self, Read, Write};
use std::ops::{self, Add, AddAssign, Deref, Mul, MulAssign, Neg, Sub, SubAssign};

fn solve(input: &mut Input, out: &mut Output) {}

fn main() {
  let mut input = Input::stdin(io::stdin());
  let mut output = Output::stdout(io::stdout());
  let tests = input.read_int();
  for _ in 0..tests {
    solve(&mut input, &mut output);
  }
}

struct Input {
  stdin: io::Stdin,
  buffer: Vec<u8>,
  pos: usize,
  size: usize,
}

impl Input {
  fn stdin(stdin: io::Stdin) -> Self {
    Input {
      stdin: stdin,
      buffer: vec![0; 4096],
      pos: 0,
      size: 0,
    }
  }

  fn get(&mut self) -> u8 {
    if self.pos == self.size {
      self.pos = 0;
      self.size = self.stdin.read(&mut self.buffer).unwrap();
    }
    self.pos += 1;
    self.buffer[self.pos - 1]
  }

  fn skip_whitespace(&mut self) -> u8 {
    let mut c = self.get();
    while c.is_ascii_whitespace() {
      c = self.get();
    }
    c
  }

  fn read_str(&mut self) -> Vec<u8> {
    let mut c = self.skip_whitespace();
    let mut res = Vec::with_capacity(32);
    while !c.is_ascii_whitespace() {
      res.push(c);
      c = self.get();
    }
    res
  }

  fn read_int(&mut self) -> i64 {
    let mut c = self.skip_whitespace();
    let sign = if c == b'-' {
      c = self.get();
      -1
    } else {
      1
    };
    let mut result = 0;
    while c.is_ascii_digit() {
      result *= 10;
      result += (c - b'0') as i64;
      c = self.get();
    }
    return result * sign;
  }

  fn read_size(&mut self) -> usize {
    self.read_int() as usize
  }

  fn read_i32_vec(&mut self, n: usize) -> Vec<i32> {
    (0..n).map(|_| self.read_size() as i32).collect()
  }

  fn read_size_vec(&mut self, n: usize) -> Vec<usize> {
    (0..n).map(|_| self.read_size()).collect()
  }
}

struct Output {
  out: io::BufWriter<io::Stdout>,
}

impl Output {
  fn stdout(stdout: io::Stdout) -> Self {
    Output {
      out: io::BufWriter::new(stdout),
    }
  }
}

impl Write for Output {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.out.write(buf)
  }
  fn flush(&mut self) -> io::Result<()> {
    self.out.flush()
  }
}

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
