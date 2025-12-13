#![allow(dead_code, unused_imports)]
#![allow(non_snake_case)]
use std::cmp::{self, Ordering, Reverse, max, min};
use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{self, BufWriter, Read, Write, stdin, stdout};
use std::marker::PhantomData;
use std::mem::{replace, swap, take};
use std::ops::{self, Add, AddAssign, Bound::Excluded, Deref, Mul, MulAssign, Neg, Sub, SubAssign};
use std::process::exit;

fn solve(input: &mut Input, out: &mut Output, _data: &Precalc) {
}

struct Precalc {}

impl Precalc {
  fn new() -> Self {
    Precalc {}
  }
}

fn main() {
  let mut input = Input::stdin();
  let mut output = Output::stdout();
  let precalc = Precalc::new();
  let tests = input.read_int();
  for _ in 0..tests {
    solve(&mut input, &mut output, &precalc);
  }
  //let child = std::thread::Builder::new()
  //  .stack_size(128 * 1024 * 1024)
  //  .spawn(run)
  //  .unwrap();
  //child.join().unwrap();
}

pub struct Input {
  stdin: io::Stdin,
  buffer: Vec<u8>,
  pos: usize,
  size: usize,
}

impl Input {
  pub fn stdin() -> Self {
    Input {
      stdin: stdin(),
      buffer: vec![0; 4096],
      pos: 0,
      size: 0,
    }
  }

  fn get(&mut self) -> u8 {
    if self.pos == self.size {
      self.pos = 0;
      self.size = self.stdin.read(&mut self.buffer).unwrap();
    }
    self.pos += 1;
    self.buffer[self.pos - 1]
  }

  fn is_eof(&mut self) -> bool {
    self.skip_whitespace();
    self.pos -= 1;
    self.size == 0
  }

  fn skip_whitespace(&mut self) -> u8 {
    let mut c = self.get();
    while c.is_ascii_whitespace() {
      c = self.get();
    }
    c
  }

  pub fn read_char(&mut self) -> u8 {
    self.skip_whitespace()
  }

  pub fn read_str(&mut self) -> Vec<u8> {
    let mut c = self.skip_whitespace();
    let mut res = Vec::with_capacity(32);
    while !c.is_ascii_whitespace() {
      res.push(c);
      c = self.get();
    }
    res
  }

  pub fn read_int(&mut self) -> i64 {
    let mut c = self.skip_whitespace();
    let sign = if c == b'-' {
      c = self.get();
      -1
    } else {
      1
    };
    let mut result = 0;
    while c.is_ascii_digit() {
      result *= 10;
      result += (c - b'0') as i64;
      c = self.get();
    }
    result * sign
  }

  pub fn read_size(&mut self) -> usize {
    self.read_int() as usize
  }

  pub fn read_i32_vec(&mut self, n: usize) -> Vec<i32> {
    (0..n).map(|_| self.read_size() as i32).collect()
  }

  pub fn read_i64_vec(&mut self, n: usize) -> Vec<i64> {
    (0..n).map(|_| self.read_size() as i64).collect()
  }

  pub fn read_size_vec(&mut self, n: usize) -> Vec<usize> {
    (0..n).map(|_| self.read_size()).collect()
  }
}

pub trait Writable {
  fn write(&self, output: &mut Output);
}

impl Writable for char {
  fn write(&self, output: &mut Output) {
    output.put(*self as u8);
  }
}

impl Writable for str {
  fn write(&self, output: &mut Output) {
    output.write_all(self.as_bytes()).unwrap();
  }
}

impl Writable for String {
  fn write(&self, output: &mut Output) {
    self.as_str().write(output);
  }
}

impl<T: Writable + ?Sized> Writable for &T {
  fn write(&self, output: &mut Output) {
    T::write(self, output);
  }
}

impl<T: Writable> Writable for [T] {
  fn write(&self, output: &mut Output) {
    output.print_iter(self.iter());
  }
}

impl<T: Writable, const N: usize> Writable for [T; N] {
  fn write(&self, output: &mut Output) {
    self.as_slice().write(output);
  }
}

impl<T: Writable> Writable for Vec<T> {
  fn write(&self, output: &mut Output) {
    self.as_slice().write(output);
  }
}

macro_rules! impl_writable_for_ints {
  ($($t:ident)+) => {
    $(impl Writable for $t {
      fn write(&self, output: &mut Output) {
        self.to_string().write(output);
      }
    })+
  };
}
impl_writable_for_ints!(i8 i16 i32 i64 u8 u16 u32 u64 usize);

// lol (A,) != (A)
macro_rules! impl_writable_for_tuples {
  ($t0:ident $($t:ident)*) => {
    impl<$t0: Writable $(, $t: Writable)*> Writable for ($t0, $($t,)*) {
      fn write(&self, output: &mut Output) {
        let ($t0, $($t,)*) = self;
        $t0.write(output);
        $(
          output.put(output.delim);
          $t.write(output);
        )*
      }
    }
  };
}
impl_writable_for_tuples!(A);
impl_writable_for_tuples!(A B);
impl_writable_for_tuples!(A B C);
impl_writable_for_tuples!(A B C D);

enum OutputDest {
  Stdout(BufWriter<io::Stdout>),
  Buffer(BufWriter<Vec<u8>>),
}

impl OutputDest {
  fn inner(&mut self) -> &mut dyn Write {
    match self {
      OutputDest::Stdout(buf) => buf,
      OutputDest::Buffer(buf) => buf,
    }
  }
}

impl Write for OutputDest {
  fn write(&mut self, data: &[u8]) -> io::Result<usize> {
    self.inner().write(data)
  }
  fn flush(&mut self) -> io::Result<()> {
    self.inner().flush()
  }
}

pub struct Output {
  out: OutputDest,
  delim: u8,
}

impl Output {
  pub fn stdout() -> Self {
    let buf = BufWriter::new(stdout());
    let out = OutputDest::Stdout(buf);
    Output { out, delim: b' ' }
  }

  pub fn buffer() -> Self {
    let buf = BufWriter::new(Vec::new());
    let out = OutputDest::Buffer(buf);
    Output { out, delim: b' ' }
  }

  pub fn str(mut self) -> String {
    self.flush().unwrap();
    match self.out {
      OutputDest::Stdout(_) => String::new(),
      OutputDest::Buffer(buf) => String::from_utf8(buf.into_inner().unwrap()).unwrap(),
    }
  }

  pub fn print<T: Writable>(&mut self, a: T) {
    a.write(self);
  }

  pub fn println<T: Writable>(&mut self, a: T) {
    self.print(a);
    self.put(b'\n');
  }

  pub fn print_lines<T: Writable>(&mut self, a: &[T]) {
    self.print_iter_lines(a.iter());
  }

  fn print_iter_lines<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
    let old_delim = self.delim;
    self.delim = b'\n';
    self.print_iter(iter);
    self.put(self.delim);
    self.delim = old_delim;
  }

  fn put(&mut self, c: u8) {
    self.out.write_all(&[c]).unwrap();
  }

  fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
    let mut print_delim = false;
    for a in iter {
      if print_delim {
        self.put(self.delim);
      }
      print_delim = true;
      a.write(self);
    }
  }
}

impl Write for Output {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.out.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.out.flush()
  }
}

//trace_macros!(true);
macro_rules! recursive_lambda {
  ($name:ident $trait:ident $($type:ident $arg:ident)*) => {
    pub trait $trait<$($type, )*R> {
      fn call(&mut self, $($arg: $type,)*) -> R;
    }
    pub struct $name<F, $($type, )*R>
    where
      F: FnMut(&mut dyn $trait<$($type, )*R>, $($type, )*) -> R
    {
      f: F,
      $($arg: PhantomData<$type>,
      )*
      r: PhantomData<R>,
    }
    impl<F, $($type, )*R> $name<F, $($type, )*R>
    where
      F: FnMut(&mut dyn $trait<$($type, )*R>, $($type, )*) -> R
    {
      pub fn new(f: F) -> Self {
        Self {
          f,
          $($arg: Default::default(),
          )*
          r: Default::default()
        }
      }
    }
    impl<F, $($type, )*R> $trait<$($type, )*R> for $name<F, $($type, )*R>
    where
      F: FnMut(&mut dyn $trait<$($type, )*R>, $($type, )*) -> R
    {
      fn call(&mut self, $($arg: $type,)*) -> R
      {
        let p = &mut self.f as *mut F;
        unsafe { (*p)(self, $($arg, )*) }
      }
    }
  };
}
trait Callable<A0, A1, R> {
  fn call(&mut self, a0: A0, a1: A1) -> R;
}

//struct FnExample<F, A0, A1, R>
//where
//  F: FnMut(&mut dyn Callable<A0, A1, R>, A0, A1) -> R,
//{
//  f: F,
//  a0: PhantomData<A0>,
//  a1: PhantomData<A1>,
//  r: PhantomData<R>,
//}
//
//impl<F, A0, A1, R> FnExample<F, A0, A1, R>
//where
//  F: FnMut(&mut dyn Callable<A0, A1, R>, A0, A1) -> R,
//{
//  fn new(f: F) -> Self {
//    FnExample {
//      f,
//      a0: Default::default(),
//      a1: Default::default(),
//      r: Default::default(),
//    }
//  }
//}
//
//impl<F, A0, A1, R> Callable<A0, A1, R> for FnExample<F, A0, A1, R>
//where
//  F: FnMut(&mut dyn Callable<A0, A1, R>, A0, A1) -> R,
//{
//  fn call(&mut self, a0: A0, a1: A1) -> R {
//    let p: *mut F = &mut self.f as *mut F;
//    unsafe { (*p)(self, a0, a1) }
//  }
//}

recursive_lambda!(Fn0 Fn0Impl);
recursive_lambda!(Fn1 Fn1Impl A0 a0);
recursive_lambda!(Fn2 Fn2Impl A0 a0 A1 a1);
recursive_lambda!(Fn3 Fn3Impl A0 a0 A1 a1 A2 a2);
recursive_lambda!(Fn4 Fn4Impl A0 a0 A1 a1 A2 a2 A3 a3);
recursive_lambda!(Fn5 Fn5Impl A0 a0 A1 a1 A2 a2 A3 a3 A4 a4);
recursive_lambda!(Fn6 Fn6Impl A0 a0 A1 a1 A2 a2 A3 a3 A4 a4 A5 a5);
