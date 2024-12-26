/* take the following `Tree`, `Node`, and `Content` structs define these functions / methods for `Tree`
- new [1] : creates an empty tree
- add_node [6]: takes a generic element `el` and adds a node to the tree whose content is `el` and such that
nodes on the left have contents which are < smaller than the current node,
nodes on the center have contents which are == to the current node,
nodes on the right have contents which are > than the current node
   - howmany_smaller [4] : takes a generic element `el` and returns an i32 telling how many nodes
       does the tree have that are < than `el`
- implement `PartialOrd` for `Content` [4]: contents can be compared by comparing the `len` of
their String fields */

#[derive(Debug)]
pub struct Content {
    pub i: i32,
    pub s: String,
}
impl Content {
    pub fn new(i: i32, s: String) -> Content {
        Content { i, s }
    }
}
type TreeLink<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: TreeLink<T>,
    center: TreeLink<T>,
    right: TreeLink<T>,
}
impl<T> Node<T> {
    pub fn new(elem: T) -> Node<T> {
        Node {
            elem,
            left: None,
            center: None,
            right: None,
        }
    }
}
#[derive(Debug)]
pub struct Tree<T> {
    root: TreeLink<T>,
    size: i32,
}

pub fn run() {
    let mut t = Tree::new();
    let e1 = Content::new(10, "asd".to_string());
    let e2 = Content::new(11, "oneasd".to_string());
    let e3 = Content::new(8, "bhas".to_string());
    let e4 = Content::new(19, "xax".to_string());
    let e5 = Content::new(45, "gip".to_string());
    t.add(e1);
    t.add(e2);
    t.add(e3);
    t.add(e4);
    t.add(e5);
    println!("{:?}", t);
}

// ! COPY FROM HERE

impl<T: PartialEq + PartialOrd> Node<T> {
    fn recoursive_count(&self, elem: &T) -> i32 {
        let mut new_count = 0;
        if self.elem < *elem {
            new_count += 1;
        }

        let left_count = match &self.left {
            None => 0,
            Some(c) => c.recoursive_count(elem),
        };
        let center_count = match &self.center {
            None => 0,
            Some(c) => c.recoursive_count(elem),
        };
        let right_count = match &self.right {
            None => 0,
            Some(c) => c.recoursive_count(elem),
        };

        return new_count + center_count + left_count + right_count;
    }
}

impl<T: PartialEq + PartialOrd + Debug> Tree<T> {
    pub fn new() -> Self {
        Tree {
            root: None,
            size: 0,
        }
    }

    /* pub fn add(&mut self, elem: T) {
        println!("called add");
        let new_node = Node {
            elem,
            right: None,
            left: None,
            center: None,
        };

        if self.root.is_none() {
            self.root = Some(Box::new(new_node));
            return;
        }

        let mut curr = &mut self.root;
        loop {
            if let Some(c) = curr.as_mut() {
                println!("{:?}", c.elem.partial_cmp(&new_node.elem));
                if c.elem == new_node.elem {
                    match c.center {
                        Some(_) => curr = &mut c.center,
                        None => {
                            c.center = Some(Box::new(new_node));
                            break;
                        }
                    }
                } else if c.elem < new_node.elem {
                    println!("gone to less");
                    match c.left {
                        Some(_) => curr = &mut c.left,
                        None => {
                            c.left = Some(Box::new(new_node));
                            break;
                        }
                    }
                } else if c.elem > new_node.elem {
                    match c.right {
                        Some(_) => curr = &mut c.right,
                        None => {
                            c.right = Some(Box::new(new_node));
                            break;
                        }
                    }
                }
            }
        }
    } */

    pub fn howmany_smaller(&self, elem: &T) -> i32 {
        match &self.root {
            None => 0,
            Some(c) => c.recoursive_count(elem),
        }
    }
}

use std::{cmp::Ordering, fmt::Debug};

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.s.len() == other.s.len()
    }
}
impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.s.len() == other.s.len() {
            return Some(Ordering::Equal);
        }
        if self.s.len() > other.s.len() {
            return Some(Ordering::Greater);
        }
        if self.s.len() < other.s.len() {
            return Some(Ordering::Less);
        }
        return None;
    }

    fn gt(&self, other: &Self) -> bool {
        self.s.len() > other.s.len()
    }

    fn lt(&self, other: &Self) -> bool {
        self.s.len() < other.s.len()
    }

    fn ge(&self, other: &Self) -> bool {
        self.s.len() >= other.s.len()
    }

    fn le(&self, other: &Self) -> bool {
        self.s.len() <= other.s.len()
    }
}
