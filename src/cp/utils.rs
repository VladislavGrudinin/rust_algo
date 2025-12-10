use std::marker::PhantomData;

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
