fn kmp(s: &[u8]) -> Vec<usize> {
  let n = s.len();
  let mut p = vec![0; n];
  let mut k = 0;
  for i in 1..n {
    while k > 0 && s[i] != s[k] {
      k = p[k - 1];
    }
    if s[i] == s[k] {
      k += 1;
    }
    p[i] = k;
  }
  p
}
