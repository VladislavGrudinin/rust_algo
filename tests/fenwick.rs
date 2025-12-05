use rust_olymp::fenwick::FenwickTree;

#[test]
fn test_fenwick_add() {
  let mut t = FenwickTree::<i32>::new_sum(5);
  assert_eq!(t.sum(0, 4), 0);
  t.update(2, 5);
  assert_eq!(t.get(0), 0);
  assert_eq!(t.get(1), 0);
  assert_eq!(t.get(2), 5);
  assert_eq!(t.get(3), 5);
  assert_eq!(t.get(4), 5);
  assert_eq!(t.sum(0, 1), 0);
  assert_eq!(t.sum(2, 2), 5);
  assert_eq!(t.sum(3, 4), 0);

  t.update(0, -5);
  t.update(3, 5);
  assert_eq!(t.get(0), -5);
  assert_eq!(t.get(1), -5);
  assert_eq!(t.get(2), 0);
  assert_eq!(t.get(3), 5);
  assert_eq!(t.get(4), 5);

  assert_eq!(t.sum(0, 1), -5);
  assert_eq!(t.sum(2, 2), 5);
  assert_eq!(t.sum(0, 4), 5);
  assert_eq!(t.sum(2, 4), 10);
}

#[test]
fn test_fenwick_min() {
  let mut t = FenwickTree::<i32>::new_min(5, i32::MAX);
  assert_eq!(t.get(4), i32::MAX);
  t.update(2, 10);
  assert_eq!(t.get(0), i32::MAX);
  assert_eq!(t.get(1), i32::MAX);
  assert_eq!(t.get(2), 10);
  assert_eq!(t.get(3), 10);
  assert_eq!(t.get(4), 10);

  t.update(3, 5);
  assert_eq!(t.get(0), i32::MAX);
  assert_eq!(t.get(1), i32::MAX);
  assert_eq!(t.get(2), 10);
  assert_eq!(t.get(3), 5);
  assert_eq!(t.get(4), 5);

  t.update(0, 7);
  assert_eq!(t.get(0), 7);
  assert_eq!(t.get(1), 7);
  assert_eq!(t.get(2), 7);
  assert_eq!(t.get(3), 5);
  assert_eq!(t.get(4), 5);
}

#[test]
fn test_fenwick_max() {
  let mut t = FenwickTree::<i32>::new_max(5, 0);
  assert_eq!(t.get(4), 0);
  t.update(2, 5);
  assert_eq!(t.get(0), 0);
  assert_eq!(t.get(1), 0);
  assert_eq!(t.get(2), 5);
  assert_eq!(t.get(3), 5);
  assert_eq!(t.get(4), 5);

  t.update(3, 10);
  assert_eq!(t.get(0), 0);
  assert_eq!(t.get(1), 0);
  assert_eq!(t.get(2), 5);
  assert_eq!(t.get(3), 10);
  assert_eq!(t.get(4), 10);

  t.update(0, 7);
  assert_eq!(t.get(0), 7);
  assert_eq!(t.get(1), 7);
  assert_eq!(t.get(2), 7);
  assert_eq!(t.get(3), 10);
  assert_eq!(t.get(4), 10);
}
