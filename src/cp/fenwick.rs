use std::cmp;
use std::ops::{Add, Sub};

pub struct FenwickTree<T> {
  t: Vec<T>,
  op: fn(T, T) -> T,
}

impl<T: Default + Add<Output = T> + Copy> FenwickTree<T> {
  pub fn new_sum(n: usize) -> Self {
    Self { t: vec![T::default(); n], op: |a, b| a + b }
  }
}

impl<T: Ord + Copy> FenwickTree<T> {
  pub fn new_max(n: usize, v: T) -> Self {
    Self { t: vec![v; n], op: |a, b| cmp::max(a, b) }
  }

  pub fn new_min(n: usize, v: T) -> Self {
    Self { t: vec![v; n], op: |a, b| cmp::min(a, b) }
  }
}

impl<T: Copy> FenwickTree<T> {
  pub fn update(&mut self, mut p: usize, v: T) {
    while p < self.t.len() {
      self.t[p] = (self.op)(self.t[p], v);
      p |= p + 1;
    }
  }

  pub fn get(&self, mut p: usize) -> T {
    let mut res = self.t[p];
    loop {
      p = (p & (p + 1)).wrapping_sub(1);
      if p >= self.t.len() {
        break;
      }
      res = (self.op)(res, self.t[p]);
    }
    res
  }
}

impl<T: Sub<Output = T> + Copy> FenwickTree<T> {
  pub fn sum(&self, l: usize, r: usize) -> T {
    if l > 0 { self.get(r) - self.get(l - 1) } else { self.get(r) }
  }
}
