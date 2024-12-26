use std::fmt::Debug;

pub trait Nextable {
    type T:Debug;

    fn gimme_next(&self) -> Option<Self::T>;
}

impl Nextable for i32 {
    type T = i32;

    fn gimme_next(&self) -> Option<Self::T> {
        Some(self + 1)
    }
}

impl Nextable for char {
  type T = char;

  fn gimme_next(&self) -> Option<Self::T> {
      char::from_u32(*self as u32 + 1)
  }
}

pub fn printnext(n: impl Nextable + Debug) {
  match n.gimme_next() {
    Some(x) => println!("next of {:?} is Some({:?})", n, x),
    None => println!("next of {:?} is None", n),
  }
}
