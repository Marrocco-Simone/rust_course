pub trait Lengthable {
  fn get_len(&self) -> usize;
}

impl<T> Lengthable for Vec<T> {
  fn get_len(&self) -> usize {
    self.len()
  }
}

impl Lengthable for String {
  fn get_len(&self) -> usize {
    self.len()
  }
}

impl Lengthable for i32 {
  fn get_len(&self) -> usize {
    let mut count = 0;
    let mut copy = self.clone();
    while copy > 1 {
      count += 1;
      copy = copy / 10;
    }
    return count;
  }
}

pub fn print_len (elem: &dyn Lengthable) {
  println!("{}", elem.get_len());
}