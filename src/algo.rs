use std::cmp::Ordering;

pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
  for i in (0..a.len() - 1).rev() {
    if a[i] < a[i + 1] {
      let j = a[i + 1..]
        .binary_search_by(|x| match a[i].cmp(x) {
          Ordering::Equal => Ordering::Greater,
          ord => ord,
        })
        .unwrap_err();
      a.swap(i, i + j);
      a[i + 1..].reverse();
      return true;
    }
  }
  a.reverse();
  return false;
}
