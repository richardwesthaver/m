pub trait Functor {
  type Unwrapped;
  type Wrapped<B>: Functor;

  fn map<F, B>(self, f: F) -> Self::Wrapped<B>
  where
    F: FnMut(Self::Unwrapped) -> B;
}

pub trait Pointed: Functor {
  fn wrap<T>(t: T) -> Self::Wrapped<T>;
}

pub trait Applicative: Pointed {
  fn lift_a2<F, B, C>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
  where
    F: FnMut(Self::Unwrapped, B) -> C;
}

pub trait Monad: Applicative {
  fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
  where
    F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>;
}

pub trait MonadTrans {
  type Base: Monad;

  fn lift(base: Self::Base) -> Self;
}
