use rust_olymp::dsu::Dsu;

#[test]
fn test_dsu() {
  let mut dsu = Dsu::new(5);
  assert_eq!(dsu.groups(), 5);
  assert_eq!(dsu.find(0), 0);
  assert_eq!(dsu.find(1), 1);
  assert_eq!(dsu.find(2), 2);
  assert_eq!(dsu.find(3), 3);
  assert_eq!(dsu.find(4), 4);
  assert_eq!(dsu.union(0, 1), true);
  assert_eq!(dsu.union(0, 1), false);
  assert_eq!(dsu.find(0), dsu.find(1));
  assert_eq!(dsu.size(0), 2);
  assert_eq!(dsu.size(1), 2);
  assert_eq!(dsu.union(2, 3), true);
  assert_eq!(dsu.union(3, 4), true);
  assert_eq!(dsu.groups(), 2);
  assert_eq!(dsu.size(2), 3);
  assert_eq!(dsu.union(1, 3), true);
  assert_eq!(dsu.groups(), 1);
  assert_eq!(dsu.size(0), 5);
  assert_eq!(dsu.find(1), dsu.find(0));
  assert_eq!(dsu.find(2), dsu.find(0));
  assert_eq!(dsu.find(3), dsu.find(0));
  assert_eq!(dsu.find(4), dsu.find(0));
}
