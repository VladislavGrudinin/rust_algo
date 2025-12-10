pub struct Lca {
  p: Vec<Vec<usize>>,
  h: Vec<u32>,
  log_n: usize,
}

impl Lca {
  pub fn new(n: usize, root: usize) -> Lca {
    let log_n = (usize::BITS - n.leading_zeros()) as usize;
    Lca {
      p: vec![vec![root; log_n]; n],
      h: vec![0; n],
      log_n,
    }
  }

  pub fn set_parent(&mut self, v: usize, p: usize) {
    self.h[v] = self.h[p] + 1;
    self.p[v][0] = p;
    for i in 1..self.log_n {
      self.p[v][i] = self.p[self.p[v][i - 1]][i - 1];
    }
  }

  pub fn query(&self, mut v: usize, mut u: usize) -> usize {
    v = self.raise(v, u);
    u = self.raise(u, v);
    if v == u {
      return v;
    }
    for i in (0..self.log_n).rev() {
      if self.p[v][i] != self.p[u][i] {
        v = self.p[v][i];
        u = self.p[u][i];
      }
    }
    return self.p[v][0];
  }

  fn raise(&self, mut v: usize, u: usize) -> usize {
    for i in (0..self.log_n).rev() {
      let jump = self.p[v][i];
      if self.h[jump] >= self.h[u] {
        v = jump;
      }
    }
    v
  }
}
