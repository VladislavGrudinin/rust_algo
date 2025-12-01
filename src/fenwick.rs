
struct FenwickTree<T> {
  t: Vec<T>,
  op: fn(T, T) -> T,
}

impl<T: Default + Add<Output = T> + Copy> FenwickTree<T> {
  fn new_sum(n: usize) -> Self {
    Self {
      t: vec![T::default(); n],
      op: |a, b| a + b,
    }
  }
}

impl<T: Default + Ord + Copy> FenwickTree<T> {
  fn new_max(n: usize) -> Self {
    Self {
      t: vec![T::default(); n],
      op: |a, b| cmp::max(a, b),
    }
  }

  fn new_min(n: usize) -> Self {
    Self {
      t: vec![T::default(); n],
      op: |a, b| cmp::min(a, b),
    }
  }
}

impl<T: Default + Copy> FenwickTree<T> {
  fn update(&mut self, mut p: usize, v: T) {
    while p < self.t.len() {
      self.t[p] = (self.op)(self.t[p], v);
      p |= p + 1;
    }
  }

  fn get(&self, mut p: usize) -> T {
    let mut res = T::default();
    while p < self.t.len() {
      res = (self.op)(res, self.t[p]);
      p = (p & (p + 1)).wrapping_sub(1);
    }
    res
  }
}
