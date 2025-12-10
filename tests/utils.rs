use rust_olymp::utils::*;

#[test]
fn test_fn_hack() {
  let mut n = 1;
  let mut res = 1;
  let mut fact = Fn0::new(|f| {
    if n == 5 {
      return n;
    }
    let x = n;
    n += 1;
    res *= f.call();
    return x;
  });
  fact.call();
  assert_eq!(res, 120);

  let n = 5;
  let mut used = vec![false; n];
  let mut g = vec![Vec::new(); n];
  g[0].push(1);
  g[1].push(2);
  g[2].push(3);
  g[2].push(4);
  let mut dfs = Fn1::new(|f, v| {
    if used[v] {
      return;
    }
    used[v] = true;
    for &u in &g[v] {
      f.call(u);
    }
  });
  dfs.call(0);
  assert!(used.iter().all(|a| *a));
}
