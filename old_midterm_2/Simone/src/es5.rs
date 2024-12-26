pub trait SameBool{
  fn samebool(&self, other:&Self)->bool;
}
#[derive(Debug)]
pub struct Content{
  pub i:i32,
  pub b:bool
}
impl Content {
  pub fn new_with(i: i32, b: bool) -> Content {
      Content { i, b }
  }
}
type NodeRef<T> = Rc<RefCell<Node<T>>>;
struct Node<T> {
  inner_value: T,
  adjacent: Vec<NodeRef<T>>,
}
impl<T:Debug> Debug for Node<T>{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
  }
}
#[derive(Debug)]
struct Graph<T> {
  nodes: Vec<NodeRef<T>>,
}

// ! Copy from here

use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::{Debug, Formatter},
    rc::Rc,
};


impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph { nodes: vec![] }
    }
}

// ! iterator
struct GraphIterator<'a, T> {
    graph: &'a Graph<T>,
    index: usize,
}

impl<'a, T> IntoIterator for &'a Graph<T> {
    type IntoIter = GraphIterator<'a, T>;
    type Item = &'a NodeRef<T>;

    fn into_iter(self) -> Self::IntoIter {
        GraphIterator {
            graph: self,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for GraphIterator<'a, T> {
    type Item = &'a NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.graph.nodes.get(self.index)
    }
}

// ! add a new node
impl<T: PartialOrd + SameBool + Debug> Graph<T> {
    fn add_node(&mut self, c: T) {
        let n: NodeRef<T> = Rc::new(RefCell::new(Node {
            inner_value: c,
            adjacent: vec![],
        }));

        // println!("new: {:?}", n.borrow().inner_value);
        for node in &self.nodes {
          // println!("is in graph: {:?}", node.borrow().inner_value)
          if n.borrow().inner_value > node.borrow().inner_value{
            n.borrow_mut().adjacent.push(Rc::clone(&node));
          }

          if n.borrow().inner_value.samebool(&node.borrow().inner_value) {
            node.borrow_mut().adjacent.push(Rc::clone(&n));
          }
        }
        self.nodes.push(n);
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}
impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.i == other.i {
            return Some(Ordering::Equal);
        }
        if self.i > other.i {
            return Some(Ordering::Greater);
        }
        if self.i < other.i {
            return Some(Ordering::Less);
        }
        return None;
    }

    fn gt(&self, other: &Self) -> bool {
        self.i > other.i
    }

    fn lt(&self, other: &Self) -> bool {
        self.i < other.i
    }

    fn ge(&self, other: &Self) -> bool {
        self.i >= other.i
    }

    fn le(&self, other: &Self) -> bool {
        self.i <= other.i
    }
}

impl SameBool for Content {
    fn samebool(&self, other: &Self) -> bool {
        self.b == other.b
    }
}
