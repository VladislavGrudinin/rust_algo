use rust_olymp::io::*;

#[test]
fn test_writable_ints() {
  let mut out = Output::buffer();

  out.println(1i8);
  out.println(1u8);
  out.println(1i16);
  out.println(1u16);
  out.println(1i32);
  out.println(1u32);
  out.println(1i64);
  out.println(1u64);
  out.println(1usize);

  let expected = "1\n1\n1\n1\n1\n1\n1\n1\n1\n";
  let result = out.str();
  assert_eq!(result, expected);
}

#[test]
fn test_writable_strings() {
  let mut out = Output::buffer();

  out.println("a");
  let a = "a";
  out.println(a);
  let a = "a".to_string();
  out.println(a);

  let expected = "a\na\na\n";
  let result = out.str();
  assert_eq!(result, expected);
}

#[test]
fn test_writable_refs() {
  let mut out = Output::buffer();

  out.println(1);
  out.println(&1);

  let expected = "1\n1\n";
  let result = out.str();
  assert_eq!(result, expected);
}

#[test]
fn test_writable_vec() {
  let mut out = Output::buffer();

  let vec = vec![1, 2, 3];
  out.println(&vec);
  out.println(vec);
  let arr = [1, 2, 3];
  out.println(&arr);
  out.println(arr);

  let expected = "1 2 3\n1 2 3\n1 2 3\n1 2 3\n";
  let result = out.str();
  assert_eq!(result, expected);
}

#[test]
fn test_writable_tuples() {
  let mut out = Output::buffer();

  let t = (1,);
  out.println(t);
  let t = (1, 2);
  out.println(t);
  let t = (1, 2, 3);
  out.println(t);
  let t = (1, 2, 3, 4);
  out.println(t);

  let expected = "1\n1 2\n1 2 3\n1 2 3 4\n";
  let result = out.str();
  assert_eq!(result, expected);
}
