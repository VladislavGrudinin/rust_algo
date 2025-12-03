pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
  if a.len() < 2 {
    return false;
  }
  for i in (0..a.len() - 1).rev() {
    if a[i] < a[i + 1] {
      let mut j = a.len() - 1;
      while a[i] >= a[j] {
        j -= 1;
      }
      a.swap(i, j);
      a[i + 1..].reverse();
      return true;
    }
  }
  a.reverse();
  return false;
}
