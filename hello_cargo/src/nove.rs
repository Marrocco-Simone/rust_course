// write enum XX with variants Y1 Y2, the former takes 2 ints, the latter a string.
// write a function `data` that takes an XX and then returns the contents of y1 multiplied by each other, or the length of Y2

pub enum XX {
  Y1(i32, i32),
  Y2(String)
}

pub fn data(xx: &XX) -> i32 {
  match xx {
    XX::Y1(first, second) => first*second,
    XX::Y2(s) => s.len() as i32
  }
}