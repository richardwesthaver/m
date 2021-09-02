#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Node {
  pub id: u8,
}

impl std::ops::Add for Node {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Node {
      id: self.id + other.id,
    }
  }
}

impl std::ops::Sub for Node {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Node {
      id: self.id - other.id,
    }
  }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Edge(pub u32);

impl std::ops::Add for Edge {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Edge(self.0 + other.0)
  }
}

impl std::ops::Sub for Edge {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Edge(self.0 - other.0)
  }
}
