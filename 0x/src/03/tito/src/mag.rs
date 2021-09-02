struct Unlimited;

trait Limit {
  fn within_limit(self, n: usize) -> bool;
}

impl Limit for usize {
  fn within_limit(self, n: usize) -> bool {
    n < self
  }
}

impl Limit for Unlimited {
  fn within_limit(self, _: usize) -> bool {
    true
  }
}

trait Semigroup {
  fn append(self, rhs: Self) -> Self;
}

impl Semigroup for String {
  fn append(mut self, rhs: Self) -> Self {
    self += &rhs;
    self
  }
}

impl<T> Semigroup for Vec<T> {
  fn append(mut self, mut rhs: Self) -> Self {
    Vec::append(&mut self, &mut rhs);
    self
  }
}

impl Semigroup for () {
  fn append(self, (): ()) -> () {}
}
