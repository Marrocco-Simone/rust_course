#[derive(Debug)]
pub struct ListElem {
    item: i32,
    next: Option<Box<ListElem>>,
}

impl ListElem {
    pub fn new(item: i32) -> ListElem {
        ListElem { item, next: None }
    }

    pub fn print(&self) {
        println!("{{{}}}", self.item);
    }
}

impl Clone for ListElem {
  fn clone (&self) -> ListElem {
    ListElem {
      item: self.item.clone(),
      next: self.next.clone(),
    }
  }
}

#[derive(Debug)]
pub struct List {
    pub start: Option<Box<ListElem>>,
}

impl List {
    pub fn new() -> List {
        List { start: None }
    }

    pub fn print(&mut self) {
        let mut buf = String::new();
        buf.push_str("{");
        match self.start.as_mut() {
            Some(start) => {
                let mut current = start;
                buf.push_str(current.item.to_string().as_str());
                while !(current.next.is_none()) {
                    let box_current = current.next.as_mut().unwrap();
                    buf.push_str(", ");
                    buf.push_str(box_current.item.to_string().as_str());
                    current = box_current;
                }
            }
            None => {},
        }
        buf.push_str("}");
        println!("{}", buf);
    }

    pub fn add(&mut self, new_elem: i32) {
        match self.start.as_mut() {
            Some(start) => {
                let mut current = start;
                while !(current.next.is_none()) {
                    let box_current = current.next.as_mut().unwrap();
                    current = box_current;
                }
                current.next = Some(Box::new(ListElem::new(new_elem)));
            }
            None => self.start = Some(Box::new(ListElem::new(new_elem))),
        }
    }

    pub fn remove(&mut self, elem_to_remove: i32) {
        match self.start.as_mut() {
            Some(start) => {
                let mut current = start;
                while !(current.next.is_none()) {
                    let box_current = current.next.as_ref().unwrap();
                    if box_current.item == elem_to_remove {
                      let new_next = box_current.next.clone();
                      current.next = new_next;
                      return;
                    } else {
                        current = current.next.as_mut().unwrap();
                    }
                }
            }
            None => {}
        }
    }

    pub fn pop(&mut self) -> i32 {
        match self.start.as_mut() {
            Some(start) => {
                let value_to_return = start.item;
                let new_start = start.next.clone();
                self.start = new_start;
                return value_to_return;
            }
            None => return 0,
        }
    }
}

impl Iterator for ListElem {
    type Item = ListElem;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item_option = &self.next;
        if next_item_option.is_none() {
            return Some(ListElem { item: 0, next: None });
        }
        let next_item_box = next_item_option.clone().unwrap();
        let next_item = *next_item_box;
        return Some(next_item);
    }
}
