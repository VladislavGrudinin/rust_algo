#[derive(Clone, Copy, Default)]
pub struct SegmentNode {
  v: i64,
}

impl SegmentNode {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn merge(a: &SegmentNode, b: &SegmentNode) -> Self {
    Self { v: a.v + b.v }
  }

  pub fn apply(&mut self, v: i64) {}

  pub fn push(&mut self, a: &mut SegmentNode, b: &mut SegmentNode) {}
}

struct SegmentTree {
  t: Vec<SegmentNode>,
  n: usize,
  h: u32,
}

impl SegmentTree {
  pub fn new(n: usize) -> SegmentTree {
    let n = n.next_power_of_two();
    SegmentTree {
      t: vec![SegmentNode::new(); 2 * n],
      n,
      h: n.ilog2(),
    }
  }

  pub fn build(&mut self) {
    let mut p = self.n - 1;
    while p > 0 {
      self.t[p] = SegmentNode::merge(&self.t[2 * p], &self.t[2 * p + 1]);
      p -= 1;
    }
  }

  pub fn update(&mut self, mut p: usize) {
    p += self.n;
    while p > 1 {
      p >>= 1;
      self.push_at(p);
      self.t[p] = SegmentNode::merge(&self.t[2 * p], &self.t[2 * p + 1]);
    }
  }

  fn push_at(&mut self, p: usize) {
    let parent = self.node_raw(p);
    let left = self.node_raw(2 * p);
    let right = self.node_raw(2 * p + 1);
    parent.push(left, right);
  }

  fn push(&mut self, leaf: usize) {
    for i in (0..self.h).rev() {
      self.push_at(leaf >> (i + 1));
    }
  }

  fn node_raw(&mut self, p: usize) -> &'static mut SegmentNode {
    let ptr = &mut self.t[p] as *mut SegmentNode;
    unsafe { &mut *ptr }
  }

  pub fn leaf(&mut self, p: usize) -> &mut SegmentNode {
    &mut self.t[p + self.n]
  }

  pub fn query(&self, mut l: usize, mut r: usize) -> SegmentNode {
    let mut left = SegmentNode::default();
    let mut right = SegmentNode::default();
    l += self.n;
    r += self.n;
    while l < r {
      if l & 1 == 1 {
        left = SegmentNode::merge(&left, &self.t[l]);
        l += 1;
      }
      if r & 1 == 1 {
        r -= 1;
        right = SegmentNode::merge(&self.t[r], &right);
      }
      l >>= 1;
      r >>= 1;
    }
    SegmentNode::merge(&left, &right)
  }

  pub fn update_range(&mut self, mut l: usize, mut r: usize, v: i64) {
    let l0 = l;
    let r0 = r;
    l += self.n;
    r += self.n;
    //self.push(l);
    //self.push(r - 1);
    while l < r {
      if l & 1 == 1 {
        self.t[l].apply(v);
        l += 1;
      }
      if r & 1 == 1 {
        r -= 1;
        self.t[r].apply(v);
      }
      l >>= 1;
      r >>= 1;
    }
    self.update(l0);
    self.update(r0 - 1);
  }

  pub fn query_lazy(&mut self, l: usize, r: usize) -> SegmentNode {
    self.push(l + self.n);
    self.push(r + self.n - 1);
    self.query(l, r)
  }
}
