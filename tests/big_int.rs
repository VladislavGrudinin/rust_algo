use rust_olymp::big_int::BigInt;

#[test]
fn test_big_int_new() {
  let a = BigInt::new("123456789123456789");
  let s = "123456789123456789";
  assert_eq!(a.to_string(), s);
  assert_eq!(BigInt::new(123456789123456789i64).to_string(), s);
  assert_eq!(BigInt::new(123456789123456789u64).to_string(), s);
  let s = "123456789";
  assert_eq!(BigInt::new(123456789i32).to_string(), s);
  assert_eq!(BigInt::new(123456789u32).to_string(), s);
}

#[test]
fn test_big_int_ops() {
  let a = BigInt::new("123456789123456789");
  assert_eq!(a.clone() + &a, BigInt::new("246913578246913578"));
  assert_eq!(a.clone() * &a, BigInt::new("15241578780673678515622620750190521"));

  let p = 15;
  let mut b = BigInt::new(1);
  for _ in 0..p {
    b *= &a;
  }
  assert_eq!(a.clone().pow(p), b);
  let c = BigInt::new(
    "23589822009762165436600892787779637826323024118638501661166189236400065059157777807304425443670213086538649795756047854143716119205939598552169935335875543928987794948957287230584808421941220994039862424232121275322678486949270096182316006298534798711716349",
  );
  assert_eq!(b, c);

  let mut a = BigInt::new(5);
  assert_eq!(a.clone() + 5, BigInt::new(10));
  assert_eq!(a.clone() * 5, BigInt::new(25));
  a += 5;
  assert_eq!(a, BigInt::new(10));
  a *= 5;
  assert_eq!(a, BigInt::new(50));
}
