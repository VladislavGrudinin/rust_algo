#[derive(Clone, Copy)]
pub struct SegmentNode {
  mx: usize,
  pos: usize,
}

impl SegmentNode {
  pub fn new() -> SegmentNode {
    SegmentNode { mx: 0, pos: 0 }
  }
  pub fn merge(a: SegmentNode, b: SegmentNode) -> SegmentNode {
    if a.mx > b.mx { a } else { b }
  }
}

struct SegmentTree {
  t: Vec<SegmentNode>,
  n: usize,
}

impl SegmentTree {
  pub fn new(a: &Vec<usize>) -> SegmentTree {
    let mut tree = SegmentTree {
      t: vec![SegmentNode::new(); 2 * a.len()],
      n: a.len(),
    };
    for i in 0..a.len() {
      tree.t[i + tree.n] = SegmentNode { mx: a[i], pos: i };
    }
    let mut i = tree.n - 1;
    while i > 0 {
      tree.t[i] = SegmentNode::merge(tree.t[2 * i], tree.t[2 * i + 1]);
      i -= 1;
    }
    tree
  }

  pub fn leaf(&mut self, p: usize) -> &mut SegmentNode {
    &mut self.t[p + self.n]
  }

  pub fn update(&mut self, mut p: usize) {
    p += self.n;
    while p > 0 {
      p >>= 1;
      self.t[p] = SegmentNode::merge(self.t[p * 2], self.t[p * 2 + 1]);
    }
  }

  pub fn query(&self, mut l: usize, mut r: usize) -> SegmentNode {
    let mut result = SegmentNode::new();
    l += self.n;
    r += self.n;
    while l < r {
      if l & 1 == 1 {
        result = SegmentNode::merge(self.t[l], result);
        l += 1;
      }
      if r & 1 == 1 {
        r -= 1;
        result = SegmentNode::merge(result, self.t[r]);
      }
      l >>= 1;
      r >>= 1;
    }
    result
  }
}
