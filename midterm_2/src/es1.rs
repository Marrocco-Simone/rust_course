/* define the Doublable trait with a method `gimme_double`
implement Doublable for i32, `gimme_double` returns a new i32 that is twice self
implement Doublable for String, `gimme_double` returns a new String that is self concatenated with self

implement a function `printdouble` that takes a `Doublable`
and prints the argument and its `gimme_double` using the ":?" formatter
    it behaves as the example:
    doubling 5 is 10
    doubling "what" is "whatwhat" */

use std::fmt::Debug;

pub trait Doublable {
  fn gimme_double(&self) -> Self;
}

impl Doublable for i32 {
  fn gimme_double(&self) -> Self {
      self * 2
  }
}

impl Doublable for String {
  fn gimme_double(&self) -> Self {
    self.repeat(2)
  }
}

pub fn printdouble(x: impl Doublable + Debug) {
  println!("doubling {:?} is {:?}", x, x.gimme_double());
}