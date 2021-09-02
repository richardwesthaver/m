#[test]
fn is_node() {
  use crate::graph::Node;
  let x = Node { id: 20 };
  let y = Node { id: 10 };
  let z = x - y;
  let x2: Node = x + x;
  let x: Node = x + z;
  assert_eq!(x, Node { id: 30 });
  assert_eq!(x2, Node { id: 40 });
  assert_eq!(z, y); // this only works if Copy and Clone are implemented for Node
}

#[test]
fn is_edge() {
  use crate::graph::Edge;
  let x: Edge = Edge(20);
  assert_eq!(x, { Edge(10) + Edge(10) });
  let y: Edge = Edge(10);
  let z = x - y;
  assert_eq!(z, y); // this only works if Copy and Clone are implemented for Node
}

#[test]
#[allow(dead_code, unreachable_code, unused_variables)]
fn is_never() {
  use crate::always;
  let never: ! = { return always(Some(true)) };
  assert_eq!(never, ());
  assert_ne!(never, ());
  assert_eq!({}, never);
  assert_eq!(never, never);
  assert_ne!(never, never);
}

#[test]
fn option_map() {
  use crate::opt::Opt;
  assert_eq!(Opt::Some(5).map(|x| x + 1), Opt::Some(6));
  assert_eq!(None.map(|x: i32| x + 1), None);
}

#[test]
fn result_map() {
  use crate::opt::Res;
  assert_eq!(Res::Ok(5).map(|x| x + 1), Res::Ok::<i32, ()>(6));
  assert_eq!(Res::Err("hello").map(|x: i32| x + 1), Res::Err("hello"));
}
