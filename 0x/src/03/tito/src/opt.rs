use crate::imp::{Applicative, Functor, Monad, Pointed};

#[derive(Debug, PartialEq)]
pub enum Opt<A> {
  Some(A),
  None,
}

impl<A> Opt<A> {
  pub fn map<F: FnOnce(A) -> B, B>(self, f: F) -> Opt<B> {
    match self {
      Opt::Some(a) => Opt::Some(f(a)),
      Opt::None => Opt::None,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Res<A, E> {
  Ok(A),
  Err(E),
}

impl<A, E> Res<A, E> {
  pub fn map<F: FnOnce(A) -> B, B>(self, f: F) -> Res<B, E> {
    match self {
      Res::Ok(a) => Res::Ok(f(a)),
      Res::Err(e) => Res::Err(e),
    }
  }
}

#[derive(PartialEq, Debug)]
enum Validation<A, E> {
  Ok(A),
  Err(E),
}

impl<A> Functor for Option<A> {
  type Unwrapped = A;
  type Wrapped<B> = Option<B>;

  fn map<F: FnMut(A) -> B, B>(self, mut f: F) -> Option<B> {
    match self {
      Some(x) => Some(f(x)),
      None => None,
    }
  }
}

impl<A> Pointed for Option<A> {
  fn wrap<T>(t: T) -> Option<T> {
    Some(t)
  }
}

impl<A> Applicative for Option<A> {
  fn lift_a2<F, B, C>(self, b: Self::Wrapped<B>, mut f: F) -> Self::Wrapped<C>
  where
    F: FnMut(Self::Unwrapped, B) -> C,
  {
    let a = self?;
    let b = b?;
    Some(f(a, b))
  }
}

impl<A> Monad for Option<A> {
  fn bind<B, F>(self, f: F) -> Option<B>
  where
    F: FnMut(A) -> Option<B>,
  {
    self.and_then(f)
  }
}
