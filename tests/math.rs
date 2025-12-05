use rust_olymp::math::*;

#[test]
fn test_gcd() {
  assert_eq!(gcd(0, 0), 0);
  assert_eq!(gcd(1, 0), 1);
  assert_eq!(gcd(0, 1), 1);
  assert_eq!(gcd(1, 1), 1);
  assert_eq!(gcd(2, 1), 1);
  assert_eq!(gcd(1, 2), 1);
  assert_eq!(gcd(2, 2), 2);
  assert_eq!(gcd(3, 2), 1);
  assert_eq!(gcd(2, 3), 1);
  assert_eq!(gcd(16, 8), 8);
  assert_eq!(gcd(8, 16), 8);
  assert_eq!(gcd(2 * 3 * 5, 2 * 3 * 7), 6);
  assert_eq!(gcd(1024, 243), 1);
  assert_eq!(gcd(243, 1024), 1);
  assert_eq!(gcd(2 * 2 * 2 * 3 * 3 * 5, 2 * 2 * 3 * 5 * 5), 60);
}
