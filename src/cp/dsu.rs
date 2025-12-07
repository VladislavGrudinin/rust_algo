use std::mem::swap;

pub struct Dsu {
  p: Vec<i32>,
  groups: usize,
}

impl Dsu {
  pub fn new(n: usize) -> Dsu {
    Dsu {
      p: vec![-1; n],
      groups: n,
    }
  }

  pub fn reset(&mut self, n: usize) {
    self.p.resize(n, -1);
    self.p.fill(-1);
    self.groups = n;
  }

  pub fn find(&mut self, x: usize) -> usize {
    if self.p[x] < 0 {
      return x;
    }
    self.p[x] = self.find(self.p[x] as usize) as i32;
    self.p[x] as usize
  }

  pub fn size(&mut self, mut a: usize) -> usize {
    a = self.find(a);
    let v = -self.p[a];
    v as usize
  }

  pub fn groups(&self) -> usize {
    self.groups
  }

  pub fn union(&mut self, mut a: usize, mut b: usize) -> bool {
    a = self.find(a);
    b = self.find(b);
    if a == b {
      return false;
    }
    if self.size(a) < self.size(b) {
      swap(&mut a, &mut b);
    }
    self.p[a] += self.p[b];
    self.p[b] = a as i32;
    self.groups -= 1;
    true
  }
}
