use rust_olymp::mod_int::*;

#[test]
fn test_from() {
  let v = 0i64;
  let _: ModInt7 = v.into();
}

#[test]
fn test_add() {
  let a: ModInt7 = 5.into();
  let b: ModInt7 = 1_000_000_002.into();
  let c = a + b;
  let mut d = a;
  d += b;
  let result = 0.into();
  assert_eq!(c, d);
  assert_eq!(c, result);
}

#[test]
fn test_sub() {
  let a: ModInt7 = 5.into();
  let b: ModInt7 = 1_000_000_002.into();
  let c = a + b;
  assert_eq!(c, 0.into());
  assert_eq!(c - b, a);
  assert_eq!(c - a, b);
}

#[test]
fn test_mul() {
  let a: ModInt7 = 2.into();
  let b: ModInt7 = (1_000_000_008 / 2).into();
  assert_eq!(a * b, 1.into());
}

#[test]
fn test_inv() {
  for i in 1..100 {
    let a: ModInt7 = i.into();
    assert_eq!(a * a.inv(), 1.into());
  }
}
