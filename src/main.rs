#![allow(dead_code, unused_imports)]
#![allow(non_snake_case)]
use std::cmp::{self, Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{self, Read, Write, stdin};
use std::mem::{replace, swap, take};
use std::ops::{self, Add, AddAssign, Deref, Mul, MulAssign, Neg, Sub, SubAssign};

fn solve(input: &mut Input, out: &mut Output) {
}

fn main() {
  let mut input = Input::stdin(io::stdin());
  let mut output = Output::stdout(io::stdout());
  //let tests = input.read_int();
  let tests = 1;
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
      stdin,
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
    result * sign
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
