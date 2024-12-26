use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        List { head: None, len: 0 }
    }

    pub fn size(&self) -> i32 {
        self.len
    }

    pub fn add(&mut self, e: T, p: i32) -> Result<(), String> {
        if p > self.len {
            return Err("wrong position".to_string());
        }

        let mut curr = &mut self.head;

        if p == 0 {
            let ret = Some(Box::new(Node {
                elem: e,
                next: curr.take(),
            }));

            self.head = ret;
            self.len += 1;

            return Ok(());
        }

        for _i in 0..p - 1 {
            match curr {
                None => panic!("Should not be here"),
                Some(c) => {
                    curr = &mut c.as_mut().next;
                }
            }
        }

        match curr {
            // maybe useless
            None => panic!("Should not be here 2"),
            Some(c) => {
                let ret = Some(Box::new(Node {
                    elem: e,
                    next: c.as_mut().next.take(),
                }));

                c.as_mut().next = ret;
            }
        };

        self.len += 1;

        return Ok(());
    }

    pub fn prepend(&mut self, e: T) -> Result<(), String> {
        self.add(e, 0)
    }

    pub fn append(&mut self, e: T) -> Result<(), String> {
        self.add(e, self.size())
    }

    pub fn get(&self, p: i32) -> Option<&T> {
        if p >= self.len {
            return None;
        }

        let mut curr = &self.head;

        for _i in 0..p {
            match curr {
                None => panic!("Should not be here"),
                Some(c) => {
                    curr = &c.as_ref().next;
                }
            }
        }

        match curr {
            None => panic!("Should not be here 2"),
            Some(c) => Some(&c.elem),
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct Content {
    s: String,
    b: bool,
    i: i32,
}

impl Content {
    pub fn new_with(s: String, b: bool, i: i32) -> Content {
        return Content { s, b, i };
    }
}
