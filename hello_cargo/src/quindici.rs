// write a struct X with 2 fields s : string and i : i32
// write a struct Y with 2 fields z : bool and c : string

// give each struct a constructor function `new`. The default values are "xxx", 10 for X and true, "op" for Y.
// give each struct a method `getstring` for replacing the string with "", moving the string out of the struct and returning said string
// use std::mem::replace

// write a function `swapstr` that takes a X and a Y and then moves s into c and c into s
// make D displayable both with :? the formatter as well as with a {} argument in println. With a {} argument, see the example for the result

#[derive(Debug)]
pub struct X {
  pub s: String,
  pub i: i32
}

#[derive(Debug)]
pub struct Y {
  pub z: bool,
  pub c: String
}

impl X {
  pub fn new() -> X {
    X {s: "xxx".to_string(), i: 10}
  }

  pub fn getstring(&mut self) -> String {
    let s = self.s.clone();
    self.s = "".to_string();
    s
  }
}

impl Y {
  pub fn new() -> Y {
    Y {z: true, c: "op".to_string()}
  }

  pub fn getstring(&mut self) -> String {
    let s = self.c.clone();
    self.c = "".to_string();
    s
  }
}

pub fn swapstr(mut x: X, mut y: Y) -> (X, Y) {
  let sx = x.getstring();
  let sy = y.getstring();

  x.s = sy;
  y.c = sx;

  (x, y)
}